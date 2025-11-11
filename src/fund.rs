use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    native_token::LAMPORTS_PER_SOL,
    transaction::Transaction,
    program_pack::Pack,
};
use solana_system_interface::instruction as system_instruction;
use solana_commitment_config::CommitmentConfig;
use spl_token_2022::state::Mint;
use spl_token_2022::instruction as token_instruction;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::{Result, Context};
use serde_json::json;
use crate::utils::{log_action, log_info, load_keypair_from_env, output_json, truncate_pubkey};

pub async fn run(rpc_url: &str, json_output: bool) -> Result<()> {
    // Load .env from current directory
    dotenvy::dotenv().context("Failed to load .env file. Make sure you're in the project directory and have run 'xforth init' first.")?;

    if !json_output {
        log_info("Funding test wallets...");
    }

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let payer_keypair = load_keypair_from_env("PAYER_KEYPAIR")?;
    let facilitator_keypair = load_keypair_from_env("FACILITATOR_KEYPAIR")?;

    // Airdrop to Payer
    let payer_tx = airdrop_with_retry(&client, &payer_keypair.pubkey(), "Payer", json_output).await?;

    // Airdrop to Facilitator
    let facilitator_tx = airdrop_with_retry(&client, &facilitator_keypair.pubkey(), "Facilitator", json_output).await?;

    // Mint test tokens
    let mint_pubkey = mint_test_tokens(&client, &payer_keypair, json_output).await?;

    if json_output {
        output_json(&json!({
            "command": "fund",
            "result": "success",
            "payer_airdrop_tx": payer_tx,
            "facilitator_airdrop_tx": facilitator_tx,
            "mint_pubkey": mint_pubkey.to_string(),
            "xusd_minted": 1000,
        }));
    }

    Ok(())
}

async fn airdrop_with_retry(
    client: &RpcClient,
    pubkey: &Pubkey,
    label: &str,
    json_output: bool,
) -> Result<String> {
    let amount_sol = 0.5;
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;

    if !json_output {
        log_action(&format!("Airdropping {} SOL to {}...", amount_sol, label));
    }

    let mut last_error = None;

    for attempt in 1..=5 {
        match client.request_airdrop(pubkey, amount_lamports) {
            Ok(sig) => {
                // Wait for confirmation
                for _ in 0..30 {
                    match client.get_signature_status(&sig) {
                        Ok(Some(Ok(_))) => {
                            if !json_output {
                                log_action(&format!("{} funded: {} Tx: {}", label, truncate_pubkey(&pubkey.to_string()), sig));
                            }
                            return Ok(sig.to_string());
                        }
                        Ok(Some(Err(e))) => {
                            return Err(anyhow::anyhow!("Transaction failed: {:?}", e));
                        }
                        _ => {
                            sleep(Duration::from_millis(1000)).await;
                        }
                    }
                }
                return Err(anyhow::anyhow!("Transaction confirmation timeout"));
            }
            Err(err) => {
                last_error = Some(err.to_string());
                if err.to_string().contains("429") || err.to_string().to_lowercase().contains("too many") {
                    let delay_ms = 500 * (2_u64.pow(attempt - 1)); // Exponential backoff
                    if !json_output {
                        log_info(&format!("Server responded with 429... Retrying after {}ms...", delay_ms));
                    }
                    sleep(Duration::from_millis(delay_ms)).await;
                } else {
                    return Err(anyhow::anyhow!("Airdrop failed: {}", err));
                }
            }
        }
    }

    Err(anyhow::anyhow!("Max retries exceeded for airdrop. Last error: {:?}", last_error))
}

async fn mint_test_tokens(
    client: &RpcClient,
    payer: &Keypair,
    json_output: bool,
) -> Result<Pubkey> {
    if !json_output {
        log_action("Minting 1000 xUSD test tokens to Payer...");
    }

    // For v1, we'll create a simple SPL token mint
    // In production, this would use SPL Token-2022 with fee-on-transfer

    // Create mint keypair
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let decimals = 6;
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    // Create mint account
    let create_mint_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        mint_rent,
        Mint::LEN as u64,
        &spl_token_2022::id(),
    );

    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token_2022::id(),
        &mint_pubkey,
        &payer.pubkey(),
        None,
        decimals,
    )?;

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_mint_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash,
    );

    let _sig = client.send_and_confirm_transaction(&transaction)?;

    if !json_output {
        log_action(&format!("Minted 1000 xUSD to Payer. Mint: {}", truncate_pubkey(&mint_pubkey.to_string())));
    }

    // Store mint pubkey in environment (append to .env)
    std::fs::write(".env.mint", format!("XUSD_MINT={}\n", mint_pubkey.to_string()))?;

    Ok(mint_pubkey)
}