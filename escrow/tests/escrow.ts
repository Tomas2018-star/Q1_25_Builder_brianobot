import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { randomBytes } from 'node:crypto';
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";

const programId = new PublicKey("ASnvhxNh4U9fwETxahiCCGa18LXjRNUDfvJnNAC5tRyg");


describe("escrow", () => {
  const provider = anchor.AnchorProvider.env()
  
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.Escrow as Program<Escrow>;

  // accounts involved
  // maker 
  // mint_a
  // mint_b
  // maker_ata_a
  // escrow
  // vault
  // system_program
  // token_program
  // associated_token_program

  let maker;
  let taker;
  let mintA;
  let mintB;
  let makerAtaA;
  let takerAtaB;
  let vault;
  let escrow;
  let bump;

  const seeds = new BN(randomBytes(8));

  before(async () => {
      // create required accounts
      maker = anchor.web3.Keypair.generate();
      taker = anchor.web3.Keypair.generate();

      await airdrop(connection, maker.publicKey, 5);
      await airdrop(connection, taker.publicKey, 5);

      mintA = await createMint(
          connection,
          maker,
          maker.publicKey,
          null,
          6,
      );
      console.log("✅ Mint A Address: ", mintA);
      
      mintB = await createMint(
          connection,
          taker,
          taker.publicKey,
          null,
          6,
      );
      console.log("✅ Mint B Address: ", mintB);


      makerAtaA = await getOrCreateAssociatedTokenAccount(
        connection,
        maker,
        mintA,
        maker.publicKey,
      );
      console.log("✅ Maker ATA A: ", makerAtaA.address);
      
      takerAtaB = await getOrCreateAssociatedTokenAccount(
        connection,
        taker,
        mintB,
        taker.publicKey,
      );
      console.log("✅ Taker ATA B: ", takerAtaB.address);

      // mint token a to maker and token b to taker
      let mint1_tx = await mintTo(connection, maker, mintA, makerAtaA.address, maker, 10000 * 10 ** 6);
      console.log("✅ Mint 1 Tx: ", mint1_tx);

      let mint2_tx = await mintTo(connection, taker, mintB, takerAtaB.address, taker, 20000 * 10 ** 6);
      console.log("✅ Mint 2 Tx: ", mint2_tx);

      [escrow, bump] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        // seeds.toBuffer(),
      ], program.programId);
      
      console.log("✅ Escrow Account created: ", escrow);

      vault = getAssociatedTokenAddressSync(
          mintA,
          escrow,
          true,
          TOKEN_PROGRAM_ID,
      );
      console.log("✅ Vault Address: ", vault);

  });

  it("Make Escrow!", async () => {
      const tx = await program.methods
      .make(seeds, new BN(1_000_000_000))
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA.address,
        escrow: escrow,
        vault: vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

      console.log("✅ Your Make transaction signature", tx);
  });

  it("Request Refund!", async () => {
    const tx = await program.methods
      .refund()
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA.address,
        escrow: escrow,
        vault: vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

      console.log("✅ Your Refund transaction signature", tx);
  })
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}