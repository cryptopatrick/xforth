<h1 align="center">
  <br>
  xforth
  <br>
</h1>

<h4 align="center">
  CLI tool for bootstrapping
  <a href="https://github.com/cryptopatrick/x402" target="_blank">
    x402 Solana projects</a></h4>

<p align="center">
  <a href="https://crates.io/crates/xforth" target="_blank">
    <img src="https://img.shields.io/crates/v/xforth" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/xforth" target="_blank">
    <img src="https://img.shields.io/crates/d/xforth" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/xforth" target="_blank">
    <img src="https://docs.rs/xforth/badge.svg" alt="Documentation"/>
  </a>
  <a href="LICENSE" target="_blank">
      <img src="https://img.shields.io/github/license/sulu/sulu.svg" alt="GitHub license"/>
  </a>
</p>

<b>Author's bio:</b> Hi, I'm CryptoPatrick! I'm currently enrolled as an
Undergraduate student in Mathematics, at Chalmers & the University of Gothenburg, Sweden. <br>
If you have any questions or need more info, then please <a href="https://discord.gg/T8EWmJZpCB">join my Discord Channel: AiMath</a>

---

<p align="center">
  <a href="#-what-is-xforth">What is xforth</a> •
  <a href="#-features">Features</a> •
  <a href="#-how-to-use">How To Use</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-license">License</a>
</p>

## Important Notices
* This tool is designed for **Solana Devnet** development and testing
* Not intended for production mainnet deployments
* Automatically generates keypairs and configuration files

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-xforth"> What is xforth</a></li>
    <li><a href="#-features"> Features</a></li>
      <ul>
        <li><a href="#-instant-project-initialization"> Instant Project Initialization</a></li>
        <li><a href="#-automated-funding"> Automated Funding</a></li>
        <li><a href="#-payment-validation"> Payment Validation</a></li>
        <li><a href="#-developer-experience"> Developer Experience</a></li>
      </ul>
    <li><a href="#-how-to-use"> How to Use</a></li>
    <li><a href="#-documentation"> Documentation</a></li>
    <li><a href="#-author"> Author</a></li>
    <li><a href="#-support"> Support</a>
    <li><a href="#-contributing"> Contributing</a></li>
    <li><a href="#-license">License</a></li>
    </li>
  </ol>
</details>

## What is xforth

`xforth` is a Rust-based CLI tool that automates the complete setup process for x402 projects on the Solana blockchain. Through intelligent automation of keypair generation, wallet funding, token minting, and payment flow validation - the whole process is much smoother.

### Use Cases

- **Rapid Prototyping**: Spin up x402 payment projects instantly for testing and development
- **Blockchain Education**: Quickly set up demo environments for learning Solana development
- **Testing Environments**: Create isolated test configurations with funded wallets and tokens
- **Agent Development**: Bootstrap AI agent payment infrastructure on Solana Devnet
- **Workshop & Training**: Enable multiple developers to get started simultaneously

### Architecture

The tool provides three core commands that handle the complete setup lifecycle:

1. **Init**: Generates keypairs, creates project structure, deploys placeholder programs, and configures environment
2. **Fund**: Airdrops SOL to wallets and mints test SPL tokens with automatic retry logic
3. **Test**: Validates the entire setup by executing a test payment transaction

## Features

###  Instant Project Initialization
- **Keypair Generation**: Automatically creates Agent/Payer and Facilitator/Receiver keypairs
- **Project Scaffolding**: Sets up complete project directory with Cargo configuration
- **Environment Configuration**: Generates `.env` file with all necessary keys and program IDs
- **Program Deployment**: Deploys placeholder facilitator program to Devnet

###  Automated Funding
- **SOL Airdrops**: Automatically requests 0.5 SOL for each wallet with exponential backoff retries
- **Token Minting**: Creates and mints test SPL tokens for payment testing
- **Rate Limit Handling**: Intelligent retry logic handles Devnet rate limiting gracefully
- **Balance Verification**: Confirms successful funding before proceeding

###  Payment Validation
- **Transaction Testing**: Executes end-to-end test payment to verify complete setup
- **Balance Checks**: Validates wallet balances before and after test transactions
- **Transaction Logging**: Provides detailed transaction IDs and explorer links
- **Error Diagnostics**: Clear error messages with troubleshooting guidance

###  Developer Experience
- **Color-Coded Output**: Enhanced terminal output with colored status indicators
- **JSON Mode**: Machine-readable output for CI/CD integration
- **Flexible RPC Configuration**: Support for custom RPC endpoints and local validators
- **Progress Tracking**: Real-time feedback on all operations
- **Zero Configuration**: Works out-of-the-box with sensible defaults

## How to Use

### Requirements
xforth requires:
- Rust 1.70 or higher
- Cargo package manager
- Internet connection for Devnet access

### Installation

Install directly from crates.io:

```bash
cargo install xforth
```

Or build from source:

```bash
git clone https://github.com/cryptopatrick/xforth.git
cd xforth
cargo build --release
```

### Quick Start

Bootstrap a complete x402 project in three commands:

```bash
# 1. Initialize your project (creates keypairs, config, and project structure)
xforth init my-payment-agent

# 2. Fund the wallets with SOL and mint test tokens
xforth fund

# 3. Validate the setup with a test payment transaction
xforth test
```

That's it! Your x402 project is now fully configured and ready for development.

### Command Reference

#### Init Command

Initialize a new x402 project:

```bash
xforth init [PROJECT_NAME]
```

**Options:**
- `PROJECT_NAME`: Optional project name (default: "my-x402-agent")
- `--rpc <URL>`: Override default Devnet RPC endpoint
- `--local`: Use local Solana validator (http://127.0.0.1:8899)
- `--json`: Output results in JSON format
- `--no-color`: Disable colored output

**What it does:**
- Generates two keypairs (Agent/Payer and Facilitator/Receiver)
- Creates project directory with template Cargo.toml
- Deploys placeholder facilitator program
- Generates `.env` file with configuration

**Example output:**
```
[ACTION] Initializing project: my-payment-agent
[LOG] Generated Agent/Payer keypair: 5Xr7...Abc
[LOG] Generated Facilitator/Receiver keypair: 9Yt3...Def
[LOG] Created project directory
[LOG] Generated .env configuration
[SUCCESS] Project initialized successfully!
```

#### Fund Command

Fund wallets and mint test tokens:

```bash
xforth fund
```

**Options:**
- `--rpc <URL>`: Override default Devnet RPC endpoint
- `--local`: Use local Solana validator
- `--json`: Output results in JSON format
- `--no-color`: Disable colored output

**What it does:**
- Airdrops 0.5 SOL to Agent/Payer wallet (with retries)
- Airdrops 0.5 SOL to Facilitator/Receiver wallet (with retries)
- Mints test SPL tokens to Agent wallet
- Verifies all balances

**Example output:**
```
[ACTION] Funding wallets...
[LOG] Airdropping 0.5 SOL to Agent: 5Xr7...Abc
[LOG] Retry 1/5 due to rate limit...
[LOG] Airdrop successful! Balance: 0.5 SOL
[LOG] Minting test tokens...
[SUCCESS] All wallets funded successfully!
```

#### Test Command

Validate setup with test transaction:

```bash
xforth test
```

**Options:**
- `--rpc <URL>`: Override default Devnet RPC endpoint
- `--local`: Use local Solana validator
- `--json`: Output results in JSON format
- `--no-color`: Disable colored output

**What it does:**
- Checks wallet balances
- Executes test payment transaction
- Verifies transaction success
- Provides transaction explorer link

**Example output:**
```
[ACTION] Running test payment...
[LOG] Agent balance: 0.5 SOL
[LOG] Facilitator balance: 0.5 SOL
[LOG] Executing payment transaction...
[TX] 3Kp9...Xyz
[SUCCESS] Test payment completed successfully!
View transaction: https://explorer.solana.com/tx/3Kp9...Xyz?cluster=devnet
```

### Advanced Usage

#### Using Custom RPC Endpoint

```bash
xforth init my-project --rpc https://my-custom-rpc.com
xforth fund --rpc https://my-custom-rpc.com
xforth test --rpc https://my-custom-rpc.com
```

#### Using Local Validator

```bash
# Start local Solana validator in another terminal
solana-test-validator

# Use xforth with local validator
xforth init my-project --local
xforth fund --local
xforth test --local
```

#### CI/CD Integration

```bash
# Use JSON output for parsing in scripts
xforth init my-project --json --no-color > init-result.json
xforth fund --json --no-color > fund-result.json
xforth test --json --no-color > test-result.json
```

### Project Structure

After running `xforth init my-project`, you'll have:

```
my-payment-agent/
├── Cargo.toml          # Rust project configuration
├── .env                # Environment variables (keypairs, program IDs)
└── src/
    └── main.rs         # Template for your x402 agent
```

### Environment Variables

The `.env` file contains:

```bash
AGENT_KEYPAIR=<base58-encoded-keypair>
FACILITATOR_KEYPAIR=<base58-encoded-keypair>
PROGRAM_ID=<deployed-program-address>
RPC_URL=https://api.devnet.solana.com
```

**Security Note:** Never commit your `.env` file to version control. Add it to `.gitignore` immediately.

## Documentation

Comprehensive documentation is available at [docs.rs/xforth](https://docs.rs/xforth), including:
- Complete API reference
- Detailed command options
- Troubleshooting guide
- Integration examples
- Best practices for x402 development

## Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a>

Keybase Verification:
https://keybase.io/cryptopatrick/sigs/8epNh5h2FtIX1UNNmf8YQ-k33M8J-Md4LnAN

## Support
I'm just a lone dev, silently hacking away, in a place far far away.
Please don't hesitate to tweet at me <a>https://x.com/cryptopatrick/</a> 
if you found this project helpful in any way.
You can also support me with Solana: 8uCGFmXRQPJQ8r16d9a1DroRcmmtzus3SFfTkteSt2Xj
or Ethereum: 0x52a361c89ebF05C8ea3fA72fc43aDF30f53e2D21

Cheers!

## Contributing

Found a bug? Missing a specific feature?
Contributions are welcome! Please see our [contributing guidelines](CONTRIBUTING.md) for details on:
- Code style and testing requirements
- Submitting bug reports and feature requests
- Development setup and workflow

## License
This project is licensed under MIT. See [LICENSE](LICENSE) for details.
