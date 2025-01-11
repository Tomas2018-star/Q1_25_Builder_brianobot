mod programs; 
    
#[allow(dead_code)]
const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;    
    use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
    use std::str::FromStr; 
    use solana_sdk::message::Message;
    use solana_client::rpc_client::RpcClient; 
    use solana_sdk::system_program;
    use solana_sdk::transaction::Transaction;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer}; 

    #[allow(unused_imports)]
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};


    #[test]
    fn keygen() {
        let kp = Keypair::new();

        println!("You have generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("✅ Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            
            },
            Err(e) => println!("❌ Oops, something went wrong: {}", e.to_string()) };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json")
            .expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("4gTWiPwC7AHdsu6BtySRd9KvEZVJmhQJRkB9rNH2P1Kj")
            .unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        // Get balance of dev wallet 
        let balance = rpc_client 
            .get_balance(&keypair.pubkey()) 
            .expect("Failed to get balance"); 


        // Get recent blockhash 
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create a test transaction to calculate fees 
        let message = Message::new_with_blockhash(
            &[
                transfer( &keypair.pubkey(), 
                    &to_pubkey, 
                    balance, 
                )
            ], Some(&keypair.pubkey()), &recent_blockhash 
        ); 

        let fee = rpc_client.get_fee_for_message(&message) .expect("Failed to get fee calculator"); 

    
        // create a transaction with the payer
        let transaction = Transaction::new_signed_with_payer( 
            &[transfer( 
                &keypair.pubkey(), 
                &to_pubkey, 
                balance - fee
            )], 
            Some(&keypair.pubkey()), 
            &vec![&keypair], 
            recent_blockhash 
        ); 

        // Send the transaction 
        let signature = rpc_client 
            .send_and_confirm_transaction(&transaction) 
            .expect("❌ Failed to send transaction"); 


        println!("✅ Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature); 
                
                            
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file"); 
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        // Define our instruction data 
        let args = CompleteArgs { github: b"brianobot".to_vec() };

        // Get recent blockhash 
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash"); 

        // Now we can invoke the "complete" function 
        let transaction = Turbin3PrereqProgram::complete( 
            &[
                &signer.pubkey(), 
                &prereq, 
                &system_program::id()
            ], 
            &args, 
            Some(&signer.pubkey()), 
            &[&signer], 
            blockhash
        ); 

        let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("❌ Failed to send transaction"); 
        
        // Print our transaction out 
        println!("✅ Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature); 

    }
}
