import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Account, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { randomBytes } from 'node:crypto';
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";


describe("escrow", () => {
  const provider = anchor.AnchorProvider.env()
  
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const connection = provider.connection;

  let maker: Keypair;
  let taker: Keypair;
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerAtaA: Account;
  let makerAtaB: Account;
  let takerAtaA: Account;
  let takerAtaB: Account;
  let vault: anchor.web3.PublicKey;
  let escrow: anchor.web3.PublicKey;
  let bump: number;

  const seed = new BN(randomBytes(8));

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

      makerAtaB = await getOrCreateAssociatedTokenAccount(
        connection,
        maker,
        mintB,
        maker.publicKey,
      );
      console.log("✅ Maker ATA B: ", makerAtaB.address);
      
      takerAtaA = await getOrCreateAssociatedTokenAccount(
        connection,
        taker,
        mintA,
        taker.publicKey,
      );
      console.log("✅ Taker ATA A: ", takerAtaA.address);
      
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
        seed.toArrayLike(Buffer, "le", 8),
      ], program.programId);
      
      console.log("✅ Escrow Account created: ", escrow);

      vault = getAssociatedTokenAddressSync(
          mintA,
          escrow,
          true,
      );
      console.log("✅ Vault Address: ", vault);

  });

  it("Make Escrow!", async () => {
      const tx = await program.methods
      .make(seed, new BN(1_000_000_000))
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA.address,
        escrow: escrow,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

      console.log("✅ Your Escrow Make transaction signature", tx);
  });

  it("Request Refund!", async () => {
    const tx = await program.methods
      .refund()
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA.address,
        escrow: escrow,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

      console.log("✅ Your Refund transaction signature", tx);
  })

  // doing this again to test the take instruction
  it("Make Escrow Again!", async () => {
      const tx = await program.methods
      .make(seed, new BN(1_000_000_000))
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA.address,
        escrow: escrow,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

      console.log("✅ Your Escrow Make transaction signature", tx);
  });

  it("Take Escrow Worked!", async () => {
    const tx = await program.methods
    .take()
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA: mintA,
      mintB: mintB,
      takerAtaA: takerAtaA.address,
      takerAtaB: takerAtaB.address,
      escrow: escrow,
      vault: vault,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc();

    console.log("✅ Your Escrow Take transaction signature", tx);
});

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