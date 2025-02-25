import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";


describe("staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Staking as Program<Staking>;

  const connection = program.provider.connection;

  let admin = anchor.web3.Keypair.generate();

  before(async () => {

  });

  it("Config Is initialized!", async () => {
    await airdrop(connection, admin.publicKey, 100);

    const tx = await program.methods.initialize(
      10,
      10,
      1_000,
    )
      .accountsPartial({
        admin: admin.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  // console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  // console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  // console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}