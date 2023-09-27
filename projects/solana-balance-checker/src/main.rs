use std::{env, str::FromStr};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

fn main() {
    // Get the Solana account address and network from command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: solana_balance <network> <address>");
        std::process::exit(1);
    }
    
    let network = &args[1];
    let address = &args[2];

    // Define a mapping from network names to their corresponding RPC URLs.
    let rpc_url = match network.as_str() {
        "dev" => "https://api.devnet.solana.com",
        "test" => "https://api.testnet.solana.com",
        "main" => "https://api.mainnet-beta.solana.com",
        _ => {
            eprintln!("Invalid network specified. Use 'dev', 'test', or 'main'.");
            std::process::exit(1);
        }
    };

    // Convert the Solana account address string to a Pubkey object.
    let pubkey = Pubkey::from_str(address).expect("Invalid address format");

    // Create a new RpcClient using the specified RPC URL.
    let client = RpcClient::new(rpc_url);

    // Fetch the balance of the account in lamports.
    let balance_lamports = client.get_balance(&pubkey).expect("Failed to fetch balance");

    // Convert the balance from lamports to SOL.
    let balance_sol = balance_lamports as f64 / 1_000_000_000.0;

    // Print the balance to the console.
    println!("The balance of {} on {}-net is {:.9} SOL", address, network, balance_sol);
}
