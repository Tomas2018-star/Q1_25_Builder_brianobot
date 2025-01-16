import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { createAccount, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../wba-wallet.json"
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address // the value here was gotten after initializing the mint account
const mint = new PublicKey("qz93JhpN9KBSWovGCY7Vrkd4hKymzR8qSaogUnyerqk");

(async () => {
    try {
        // Create an ATA
        // remember before we can create a new token from a Token mint
        // we need a place to store that token, and that is where the Token account comes in
        // we can either use the normal token account or an associated token account for a particular user
        // and a particular token mint

        // the ATA stores token in account made from the owner public address and the token mint
        // most of the times, it is the ATA we would use to store token for an user, 
        // even if the user does not have an ATA, you can find it for them and store tokens there for them
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,

        );
        console.log(`✅ Your ata is: ${ata.address.toBase58()}`);
        
        // techinically the ata is enough to hold the to be created token
        // but as part of my execerise i want to also document the creation of a token account here
        // const tokenAccount = await createAccount(
        //     connection,
        //     keypair,
        //     mint,
        //     keypair.publicKey,
        //     keypair,
        // );

        // console.log("Token Account: ", tokenAccount.toBase58())

        // Mint to ATA
        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair.publicKey,
            token_decimals // notice how the numbers of zeros here must match the decimal in the mint
            // for each to equal one full unit of the token minted from the Mint Account
            // so in our case, since the decimal value was 6, to get a whole unit of the token, 
            // we must set 1e6, (1_000_000n), 
        );
        console.log(`Your mint txid: ${mintTx}`);


        // // mint to Token Account
        // const mintTokenAccountTx = await mintTo(
        //     connection,
        //     keypair,
        //     mint,
        //     tokenAccount,
        //     keypair.publicKey,token_decimals
        // );
        // console.log("Second Mint Txid: ", mintTokenAccountTx);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
