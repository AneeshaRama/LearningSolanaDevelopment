# Solana Balance Checker
A simple command-line application written in Rust to fetch and display the balance of a Solana account. This tool allows you to check the balance of any Solana account on different Solana networks, including the development, test, and mainnet networks.

### Networks

The solana_balance_checker tool supports three Solana networks:

  - **Development Network (dev)**: This network is used for testing and development purposes. The RPC URL for the development network is https://api.devnet.solana.com.

  - **Test Network (test)**: This is the Solana testnet, ideal for testing your Solana applications. The RPC URL for the test network is https://api.testnet.solana.com.

  - **Main Network (main)**: This is the mainnet-beta network, where real SOL transactions occur. The RPC URL for the main network is https://api.mainnet-beta.solana.com.

### Installation

Before using the solana_balance_balance tool, make sure you have Rust and Cargo installed. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install.

Once Rust is installed, follow these steps to build and run the tool:

1. Clone the repository to your local machine:
  ```
  git clone https://github.com/AneeshaRama/LearningSolanaDevelopment.git

  ```

2. Navigate to the project directory:
  ```
  cd LearningSolanaDevelopment/projects/solana-balance-checker/

  ```
3. Build the application:
  ```
   cargo build

  ```
4. Run the application using the cargo run command, specifying the network and account address:
  ```
  cargo run -q <netwrok> <solana_account_address>

  ```
Replace **network** and **address** with the desired Solana network and account address.
For example you can run the command like this:
  ```
    // dev-net
    cargo run dev 7bBPWFvUkgthBW5ozV237YrWAuu2Byfp8aW6awTNDEEP

    // test-net
    cargo run test 7bBPWFvUkgthBW5ozV237YrWAuu2Byfp8aW6awTNDEEP

    // main-net
    cargo run main 7bBPWFvUkgthBW5ozV237YrWAuu2Byfp8aW6awTNDEEP

  ```
The output will look like:
```
The balance of 7bBPWFvUkgthBW5ozV237YrWAuu2Byfp8aW6awTNDEEP on dev-net is 21.718800401 SOL

```
