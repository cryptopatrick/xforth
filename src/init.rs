use solana_sdk::signature::Signer;
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde_json::json;
use crate::utils::{log_action, log_info, generate_keypair, keypair_to_json, output_json};

const CARGO_TOML_TEMPLATE: &str = r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.37.0", features = ["full"] }
solana-sdk = "1.18.0"
solana-client = "1.18.0"
anyhow = "1.0.82"
"#;

const MAIN_RS_TEMPLATE: &str = r#"#[tokio::main]
async fn main() {
    // TODO: Implement x402 agent logic here
    // Example: Use solana-sdk to build x402 payment transaction for HTTP 402 response
    println!("x402 agent ready!");
}
"#;

const X402_TOML_TEMPLATE: &str = r#"# x402 Protocol Configuration
[protocol]
version = "0.1.0"

[facilitator]
# Facilitator program ID will be set after deployment
program_id = "{{facilitator_program_id}}"

[payments]
# Default payment settings for x402 transactions
default_amount = 0.1
"#;

const README_TEMPLATE: &str = r#"# {{project_name}}

An x402 payment agent project bootstrapped with xforth.

## Setup Complete

Your project has been initialized with:
- Agent/Payer keypair
- Facilitator/Receiver keypair
- Project template files
- Configuration in `.env`

## Next Steps

1. Fund your wallets: `xforth fund`
2. Test the setup: `xforth test`
3. Start building your x402 agent logic in `src/main.rs`

## Security Note

⚠️ **IMPORTANT**: Never commit your `.env` file to version control. It contains your keypairs.
The `.gitignore` file has been configured to exclude it automatically.
"#;

const GITIGNORE_TEMPLATE: &str = r#"# Environment and secrets
.env
.env.*

# Rust
target/
Cargo.lock
**/*.rs.bk

# IDE
.idea/
.vscode/
*.swp
*.swo
*~
"#;

pub async fn run(project_name: &str, rpc_url: &str, json_output: bool) -> Result<()> {
    if !json_output {
        log_info("Generating keypairs...");
    }

    let payer_keypair = generate_keypair();
    let facilitator_keypair = generate_keypair();

    let payer_pubkey = payer_keypair.pubkey().to_string();
    let facilitator_pubkey = facilitator_keypair.pubkey().to_string();

    if !json_output {
        log_action(&format!("Generated Agent/Payer keypair: {}", payer_pubkey));
        log_action(&format!("Generated Facilitator/Receiver keypair: {}", facilitator_pubkey));
    }

    // Create project directory
    let project_dir = Path::new(project_name);
    fs::create_dir_all(project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;

    if !json_output {
        log_info("Creating project template...");
    }

    // Create Cargo.toml
    let cargo_toml = CARGO_TOML_TEMPLATE.replace("{{project_name}}", project_name);
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    // Create src/main.rs
    fs::write(project_dir.join("src/main.rs"), MAIN_RS_TEMPLATE)?;

    // Create x402.toml
    let x402_toml = X402_TOML_TEMPLATE.replace("{{facilitator_program_id}}", &facilitator_pubkey);
    fs::write(project_dir.join("x402.toml"), x402_toml)?;

    // Create README.md
    let readme = README_TEMPLATE.replace("{{project_name}}", project_name);
    fs::write(project_dir.join("README.md"), readme)?;

    // Create .gitignore
    fs::write(project_dir.join(".gitignore"), GITIGNORE_TEMPLATE)?;

    if !json_output {
        log_action("Project template created");
    }

    // Placeholder deployment log
    if !json_output {
        log_info("Note: This is a placeholder deployment. In production, the actual x402 facilitator program would be deployed here.");
        log_action(&format!("Facilitator deployed: {}", facilitator_pubkey));
    }

    // Create .env file
    let env_content = format!(
        "PAYER_KEYPAIR={}\nFACILITATOR_KEYPAIR={}\nFACILITATOR_PROGRAM_ID={}\nRPC_URL={}\n",
        keypair_to_json(&payer_keypair),
        keypair_to_json(&facilitator_keypair),
        facilitator_pubkey,
        rpc_url
    );
    fs::write(project_dir.join(".env"), env_content)?;

    if json_output {
        output_json(&json!({
            "command": "init",
            "result": "success",
            "project_name": project_name,
            "payer_pubkey": payer_pubkey,
            "facilitator_pubkey": facilitator_pubkey,
            "facilitator_program_id": facilitator_pubkey,
        }));
    } else {
        log_action("Configuration file (.env) created");
        log_info("Project initialized successfully!");
        println!("\nNext steps:");
        println!("1. cd {}", project_name);
        println!("2. xforth fund");
        println!("3. xforth test");
    }

    Ok(())
}