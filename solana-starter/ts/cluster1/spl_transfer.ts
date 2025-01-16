import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("qz93JhpN9KBSWovGCY7Vrkd4hKymzR8qSaogUnyerqk");

// Recipient address
const to = new PublicKey("AYnYh4u4tyANs9KJo1xegohEQcA2pWxeqHFMwUhE15eT");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        console.log("From Wallet: ", fromWallet.address);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to,
        );
        console.log("ToWallet: ", toWallet.address);

        // Transfer the new token to the "toTokenAccount" we just created
        const transferID = await transfer(
            connection,
            keypair,
            fromWallet.address,
            toWallet.address,
            keypair,
            1_000_000n,
        );
        console.log("Transfer ID: ", transferID);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();