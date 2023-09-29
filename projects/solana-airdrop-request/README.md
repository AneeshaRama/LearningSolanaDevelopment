# Solana Airdrop Request

For development purposes, we require simulated Solana tokens. These tokens can be obtained through an airdrop mechanism specifically designed for testing and development.
As part of the learning solana development journey, I have created the simple command line application using Rust which takes solana account address and SOL amount as command line arguments and requests
the airdrop.


### Installation

Before using the solana_airdrop_request tool, make sure you have Rust and Cargo installed. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install.

Once Rust is installed, follow these steps to build and run the tool:

1. Clone the repository to your local machine:
  ```
  git clone https://github.com/AneeshaRama/LearningSolanaDevelopment.git

  ```

2. Navigate to the project directory:
  ```
  cd LearningSolanaDevelopment/projects/solana-airdrop-request/

  ```
3. Build the application:
  ```
   cargo build

  ```
4. Run the application using the cargo run command, specifying the account address and SOL amount:
  ```
  cargo run <solana_account_address> <SOL amount>

  ```
Replace **address** and **SOL amount** with the desired Solana network and the SOL amount to you want to request(Let it be less than 3 for greater succeess).
For example you can run the command like this:
  ```
    
  cargo run 7bBPWFvUkgthBW5ozV237YrWAuu2Byfp8aW6awTNDEEP 1

  ```
The output will look like:
```
Successfully credited 1 SOL to your account. Airdrop transcation hash: 3BKMuzHAtuFQiC5yJqzXiuwKdixkKL4U71TH9mLVkdxyMcPZRH28ZiErwznkRFvrN8At2ghKCb3BJFa7ZCFcGxUa

```
