use clap::{Parser, Subcommand};
use anyhow::Result;

mod init;
mod fund;
mod test;
mod utils;

#[derive(Parser)]
#[command(name = "xforth")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for bootstrapping x402 Solana projects in under 90 seconds", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Override RPC endpoint
    #[arg(long, global = true)]
    rpc: Option<String>,

    /// Use local Solana validator
    #[arg(long, global = true)]
    local: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new x402 project
    Init {
        /// Project name (default: "my-x402-agent")
        #[arg(default_value = "my-x402-agent")]
        name: String,
    },
    /// Fund wallets with SOL and mint test tokens
    Fund,
    /// Validate payment flow with a test transaction
    Test,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set global config
    if cli.no_color {
        colored::control::set_override(false);
    }

    let rpc_url = if cli.local {
        "http://127.0.0.1:8899".to_string()
    } else {
        cli.rpc.unwrap_or_else(|| "https://api.devnet.solana.com".to_string())
    };

    match cli.command {
        Commands::Init { name } => {
            init::run(&name, &rpc_url, cli.json).await?;
        }
        Commands::Fund => {
            fund::run(&rpc_url, cli.json).await?;
        }
        Commands::Test => {
            test::run(&rpc_url, cli.json).await?;
        }
    }

    Ok(())
}