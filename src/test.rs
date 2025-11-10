use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::Signer,
    transaction::Transaction,
    system_instruction,
    native_token::{sol_to_lamports, lamports_to_sol},
    commitment_config::CommitmentConfig,
};
use anyhow::{Result, Context};
use serde_json::json;
use crate::utils::{log_action, log_info, load_keypair_from_env, log_balance, output_json};

pub async fn run(rpc_url: &str, json_output: bool) -> Result<()> {
    // Load .env from current directory
    dotenvy::dotenv().context("Failed to load .env file. Make sure you're in the project directory and have run 'xforth init' first.")?;

    if !json_output {
        log_action("Testing x402 payment flow...");
    }

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let payer_keypair = load_keypair_from_env("PAYER_KEYPAIR")?;
    let facilitator_keypair = load_keypair_from_env("FACILITATOR_KEYPAIR")?;

    // Check balances
    if !json_output {
        log_info("Checking wallet balances...");
    }

    let payer_balance_lamports = client.get_balance(&payer_keypair.pubkey())?;
    let facilitator_balance_lamports = client.get_balance(&facilitator_keypair.pubkey())?;

    let payer_balance = lamports_to_sol(payer_balance_lamports);
    let facilitator_balance = lamports_to_sol(facilitator_balance_lamports);

    if !json_output {
        log_balance("Payer balance", payer_balance);
        log_balance("Facilitator balance", facilitator_balance);
    }

    // Validate balances
    if payer_balance < 0.1 {
        return Err(anyhow::anyhow!(
            "Insufficient payer balance: {} SOL. Run 'xforth fund' first.",
            payer_balance
        ));
    }

    // Execute test payment (transfer 0.1 SOL from payer to facilitator)
    if !json_output {
        log_action("Executing test payment...");
    }

    let transfer_amount = 0.1;
    let instruction = system_instruction::transfer(
        &payer_keypair.pubkey(),
        &facilitator_keypair.pubkey(),
        sol_to_lamports(transfer_amount),
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;

    // Get updated balances
    let payer_balance_after = lamports_to_sol(client.get_balance(&payer_keypair.pubkey())?);
    let facilitator_balance_after = lamports_to_sol(client.get_balance(&facilitator_keypair.pubkey())?);

    if json_output {
        output_json(&json!({
            "command": "test",
            "result": "success",
            "transaction_signature": sig.to_string(),
            "transfer_amount_sol": transfer_amount,
            "payer_balance_before": payer_balance,
            "payer_balance_after": payer_balance_after,
            "facilitator_balance_before": facilitator_balance,
            "facilitator_balance_after": facilitator_balance_after,
        }));
    } else {
        log_info(&format!("Payment successful! Tx: {}", sig));
        log_info("All tests passed! Your x402 setup is ready to use.");
    }

    Ok(())
}