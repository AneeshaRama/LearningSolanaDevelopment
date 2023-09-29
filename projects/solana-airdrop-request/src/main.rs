use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

fn main() {
    // Get the Solana account address and airdrop amount from the command line arguments.
    let address = std::env::args().nth(1).unwrap();
    let sol = std::env::args().nth(2).unwrap().parse::<u64>().unwrap();

    let amount_in_lamports = sol * 1_000_000_000;

    // Create a new RpcClient.
    let client = RpcClient::new("https://api.devnet.solana.com");

    let pubkey = Pubkey::from_str(&address).expect("Invalid address format");

    // Request an airdrop for the specified account address and amount.
    let txhash = match client.request_airdrop(&pubkey, amount_in_lamports){
        Ok(txhash) => txhash,
        Err(err) => {
            println!("OOPS...Something went wrong. Please try again. The following error occurred: {}", err);
            std::process::exit(1);
        }
    };

    println!("Successfully credited {} SOL to your account. Airdrop transcation hash: {}", sol, txhash);
}


