import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { randomBytes } from 'node:crypto';
import { confirmTransaction } from "@solana-developers/helpers";

const programId = new PublicKey("ASnvhxNh4U9fwETxahiCCGa18LXjRNUDfvJnNAC5tRyg");


describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.Escrow as Program<Escrow>;

  let maker = anchor.web3.Keypair.generate();
  let taker = anchor.web3.Keypair.generate();

  const seed = new BN(randomBytes(8));
  const amount = new BN(1_000_000);

  let [escrow, escrow_bump] = PublicKey.findProgramAddressSync([
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, 'le', 8),
    ],
    programId
  );

  let vault;
  let mint_a_PublicKey;
  let mint_b_PublicKey;
  let maker_ata_a;
  let taker_ata_b;

  before(
    "Create Accounts",
    async () => {
      let airdrop1 = await provider.connection.requestAirdrop(maker.publicKey, 2 * LAMPORTS_PER_SOL);
      let airdrop1_tx = await confirmTransaction(connection, airdrop1, "confirmed");
      console.log("✅ Airdrop 1: ", airdrop1_tx);

      let airdrop2 = await provider.connection.requestAirdrop(taker.publicKey, 2 * LAMPORTS_PER_SOL);
      let airdrop2_tx = await confirmTransaction(connection, airdrop2, "confirmed");
      console.log("✅ Airdrop 2: ", airdrop2_tx);

      // create token mint that would be used to create escrow
      mint_a_PublicKey = await createMint(
          connection,
          maker,
          maker.publicKey,
          null,
          6,
      );
      console.log("✅ Mint A Address: ", mint_a_PublicKey);
      
      mint_b_PublicKey = await createMint(
          connection,
          taker,
          taker.publicKey,
          null,
          6,
      );
      console.log("✅ Mint B Address: ", mint_b_PublicKey);

      maker_ata_a = await getOrCreateAssociatedTokenAccount(
        connection,
        maker,
        mint_a_PublicKey,
        maker.publicKey,
      );
      console.log("✅ Maker ATA A: ", maker_ata_a.address);
      
      taker_ata_b = await getOrCreateAssociatedTokenAccount(
        connection,
        taker,
        mint_b_PublicKey,
        taker.publicKey,
      );
      console.log("✅ Taker ATA B: ", taker_ata_b.address);

      // mint token a to maker and token b to taker
      let mint1_tx = await mintTo(connection, maker, mint_a_PublicKey, maker_ata_a.address, maker, 10000 * 10 ** 6);
      console.log("✅ Mint 1 Tx: ", mint1_tx);

      let mint2_tx = await mintTo(connection, 
      taker, mint_b_PublicKey, taker_ata_b.address, taker, 20000 * 10 ** 6);
      console.log("✅ Mint 2 Tx: ", mint2_tx);
    }
  );

  it("Make Escrow!", async () => {
    vault = getAssociatedTokenAddressSync(
        mint_a_PublicKey,
        escrow,
        true,
        TOKEN_PROGRAM_ID,
    );
    console.log("✅ Vault Address: ", vault);

    try {
      const tx = await program.methods.make(seed, amount).accountsPartial({
        maker: maker.publicKey,
        mintA: mint_a_PublicKey,
        mintB: mint_b_PublicKey,
        makerAtaA: maker_ata_a,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([maker]).rpc();

    } catch(error) {
      console.log("Error Ocurred: ", error)
    }

  });
});
