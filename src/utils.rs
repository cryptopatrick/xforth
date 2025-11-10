use solana_sdk::signature::Keypair;
use anyhow::{Result, Context};
use colored::Colorize;
use std::env;

/// Generate a new Solana keypair
pub fn generate_keypair() -> Keypair {
    Keypair::new()
}

/// Convert keypair to JSON array format for .env storage
pub fn keypair_to_json(keypair: &Keypair) -> String {
    let bytes = keypair.to_bytes();
    serde_json::to_string(&bytes.to_vec()).unwrap()
}

/// Load keypair from JSON array string
pub fn keypair_from_json(json_str: &str) -> Result<Keypair> {
    let bytes: Vec<u8> = serde_json::from_str(json_str)
        .context("Failed to parse keypair JSON")?;
    Keypair::from_bytes(&bytes)
        .map_err(|e| anyhow::anyhow!("Invalid keypair bytes: {}", e))
}

/// Load keypair from environment variable
pub fn load_keypair_from_env(env_var: &str) -> Result<Keypair> {
    let json_str = env::var(env_var)
        .context(format!("Environment variable {} not found", env_var))?;
    keypair_from_json(&json_str)
}

/// Logging functions with colored output
pub fn log_action(msg: &str) {
    println!("{} {}", "Action:".green().bold(), msg);
}

pub fn log_info(msg: &str) {
    println!("{} \"{}\"", "Log:".yellow(), msg);
}

pub fn log_error(msg: &str) {
    eprintln!("{} {}", "Error:".red().bold(), msg);
}

pub fn log_balance(label: &str, balance: f64) {
    println!("{}: {} SOL", label, balance);
}

/// JSON output for programmatic consumption
pub fn output_json(data: &serde_json::Value) {
    println!("{}", serde_json::to_string_pretty(data).unwrap());
}

/// Truncate public key for display (first 4 chars...last 4 chars)
pub fn truncate_pubkey(pubkey: &str) -> String {
    if pubkey.len() > 8 {
        format!("{}...{}", &pubkey[..4], &pubkey[pubkey.len()-4..])
    } else {
        pubkey.to_string()
    }
}