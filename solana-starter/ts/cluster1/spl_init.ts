import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// the createMint functions works well when your script has access to your secret key as in our case
// but in a case where you want your user to create new Token Mint maybe from a website
// and you do not want them to expose their secret keys to the browser, you would create a transaction
// with the right instruction, under the hood, the createMint function calls two instructions
// - create new account
// - initialize a new mint

(async () => {
    try {
        // Start here 
        // the mint account (Token Mint) is needed to create a token (SPL Token Account)
        // this function returns the public key of the nrely created Token Mint
        const mint = await createMint(
            connection, // the JSON-RPC connection to the cluster
            keypair, // payer of the transaction fees
            keypair.publicKey, // mint authority 
            // the account that is authorized to do the actual minting of tokens from the token mint.
            keypair.publicKey, // freezeAuthority specified here
            6 // decimal place of the 
        ); 
        console.log("✅ Mint Address: ",  mint.toBase58());
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
