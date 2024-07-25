mod programs;

#[cfg(test)]
mod tests {
    use std::{
        io::{self, BufRead},
        str::FromStr,
    };

    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        message::Message,
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_instruction::transfer,
        system_program,
        transaction::Transaction,
    };

    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn test_keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file.");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn test_base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{wallet:?}");
    }

    #[test]
    fn test_wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet: Vec<u8> = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{base58:?}");
    }

    /// dev-wallet Pubkey: GDvHTKs9M23Zx2Xbo1tmnzkEYgRQVige47QHopmemra9
    #[test]
    fn test_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2 * LAMPORTS_PER_SOL) {
            Ok(s) => {
                println!("Success! Check out your TX here");
                println!("https://explorer.solana.com/tx/{s}?cluster=devnet");
            }
            Err(e) => {
                println!("Oops, something went wrong: {e}");
            }
        }
    }

    #[test]
    fn test_transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("").unwrap();
        let client = RpcClient::new(RPC_URL);

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transfer_inst = transfer(&keypair.pubkey(), &to_pubkey, 1 * LAMPORTS_PER_SOL);

        let transaction = Transaction::new_signed_with_payer(
            &[transfer_inst],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here");
        println!("https://explorer.solana.com/tx/{signature}?cluster=devnet");
    }

    #[test]
    fn test_transfer_all_remaining_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("").unwrap();
        let client = RpcClient::new(RPC_URL);

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let transfer_inst = transfer(&keypair.pubkey(), &to_pubkey, balance);
        let message = Message::new_with_blockhash(
            &[transfer_inst],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transfer_inst = transfer(&keypair.pubkey(), &to_pubkey, balance - fee);
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_inst],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here");
        println!("https://explorer.solana.com/tx/{signature}?cluster=devnet");
    }

    #[test]
    fn test_wba_prereq_program() {
        let signer = read_keypair_file("dev1-wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"aoikurokawa".to_vec(),
        };

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here");
        println!("https://explorer.solana.com/tx/{signature}?cluster=devnet");
    }
}
