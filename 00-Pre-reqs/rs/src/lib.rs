use bs58; 
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use solana_sdk;
    
    #[test]
    fn keygen() {
        use solana_sdk::signature::{Keypair, Signer};        
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey());
        println!("To save your wallet, copy this into a file:");
        println!("{:?}", kp.to_bytes());
    }
    
    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
    
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
    
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => println!("Success: https://explorer.solana.com/tx/{}?cluster=devnet", sig),
            Err(err) => println!("Airdrop failed: {}", err),
        }
    }
    
    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
    
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        
        let keypair = read_keypair_file("dev-wallet.json").unwrap();
        let to_pubkey = Pubkey::from_str("FXtkuJGP5Zmwu3QUgvhm3MuzcxUV3AMyd6WHdwnT8rxr").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
    
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    
        let tx = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000)],  // 0.1 SOL = 100_000 lamports
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );
    
        let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
        println!("TX Success: https://explorer.solana.com/tx/{}/?cluster=devnet", sig);
    }
    
    #[test]
    fn empty_wallet() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            message::Message,
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
    
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    
        let keypair = read_keypair_file("dev-wallet.json").unwrap();
        let to_pubkey = Pubkey::from_str("FXtkuJGP5Zmwu3QUgvhm3MuzcxUV3AMyd6WHdwnT8rxr").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
    
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let balance = rpc_client.get_balance(&keypair.pubkey()).unwrap();
    
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
    
        let fee = rpc_client.get_fee_for_message(&message).unwrap();
    
        let tx = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );
    
        let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
        println!("Emptied wallet: https://explorer.solana.com/tx/{}/?cluster=devnet", sig);
    }
    
    #[test]
    fn submit_rs() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            signature::{read_keypair_file, Keypair, Signer},
            system_program,
            transaction::Transaction,
        };
        use std::str::FromStr;
    
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Turbin3-wallet.json not found");
        let signer_pubkey = signer.pubkey();
        let rpc_client = RpcClient::new(RPC_URL);
        let mint = Keypair::new();
    
        let program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    
        // PDA for 'account'
        let (account_pda, _) = Pubkey::find_program_address(
            &[b"prereqs", signer_pubkey.as_ref()],
            &program_id,
        );
    
        // PDA for 'authority'
        let (authority_pda, _) = Pubkey::find_program_address(
            &[b"collection", collection.as_ref()],
            &program_id,
        );
    
        // Discriminator for 'submit_rs'
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
    
        let accounts = vec![
            AccountMeta::new(signer_pubkey, true),            // user
            AccountMeta::new(account_pda, false),             // account
            AccountMeta::new(mint.pubkey(), true),            // mint
            AccountMeta::new(collection, false),              // collection
            AccountMeta::new_readonly(authority_pda, false),  // authority PDA
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
    
        let instruction = Instruction {
            program_id,
            accounts,
            data,
        };
    
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
    
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer_pubkey),
            &[&signer, &mint],
            blockhash,
        );
    
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .expect("submit_rs transaction failed");
    
        println!(
            "✅ submit_rs success: https://explorer.solana.com/tx/{}/?cluster=devnet",
            sig
        );
    }
    
    
    #[test]
fn base58_to_wallet() {
    use bs58;
    use std::io::{self, BufRead};

    println!("Input your private key as a base58 string:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    let wallet = bs58::decode(base58).into_vec().unwrap();

    println!("Your wallet file format is:\n{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    use bs58;
    use std::io::{self, BufRead};

    println!("Input your private key as a JSON byte array:");
    let stdin = io::stdin();
    let wallet = stdin.lock().lines().next().unwrap().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    println!("Your Base58-encoded private key is:\n{}", bs58::encode(wallet).into_string());
}

#[test]
fn show_wallet_pubkey() {
    use solana_sdk::signature::{read_keypair_file, Signer};
    
    let keypair = read_keypair_file("dev-wallet.json").unwrap();
    println!("Dev wallet public key: {}", keypair.pubkey());
}

}

