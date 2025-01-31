import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token"

describe("marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;
  const program = anchor.workspace.Marketplace as Program<Marketplace>;

  let maker = anchor.web3.Keypair.generate();
  let taker = anchor.web3.Keypair.generate();

  let mint;
  let maker_ata;
  
  before(async () => {
    // maker and taker account
    // airdrop the accounts
    const makerAirdrop = await connection.requestAirdrop(maker.publicKey, 7 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: makerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    
    const takerAirdrop = await connection.requestAirdrop(taker.publicKey, 7 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction({
      signature: takerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    // create mint account and nft
    mint = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      0 // zero decimals for ntf mints
    );

    // create maker ATA
    maker_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mint,
      maker.publicKey
    );

    // MintTo nft to maker ATA
    mintTo(connection, maker, mint, maker_ata.address, maker, 0); // decimals must alwaus be 0 for nft
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
