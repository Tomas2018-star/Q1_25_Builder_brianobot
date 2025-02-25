import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { createMint, getOrCreateAssociatedTokenAccount, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";


describe("staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Staking as Program<Staking>;
  const connection = program.provider.connection;

  let admin = anchor.web3.Keypair.generate();
  let user = anchor.web3.Keypair.generate();

  let config: PublicKey;
  let configBump;
  let rewardsMint;
  let rewardsMintBump;
  let userAccount;
  let userAccountBump;
  let nftMint;
  let nftMintATA;

  before(async () => {
    [config, configBump] = PublicKey.findProgramAddressSync([
        Buffer.from("stake_config"),
      ], program.programId);
      console.log("✅ Config PDA: ", config);
    
      [rewardsMint, rewardsMintBump] = PublicKey.findProgramAddressSync([
        Buffer.from("rewards"),
        config.toBuffer(),
      ], program.programId);
      console.log("✅ Reward Mint PDA: ", rewardsMint);
     
      [userAccount, userAccountBump] = PublicKey.findProgramAddressSync([
        Buffer.from("user_account"),
        user.publicKey.toBuffer(),
      ], program.programId);
      console.log("✅ Reward Mint PDA: ", rewardsMint);

      await airdrop(connection, admin.publicKey, 100);
      nftMint = await createMint(connection, admin, user.publicKey, null, 6);

      nftMintATA = await getOrCreateAssociatedTokenAccount(
        connection,
        admin,
        nftMint,
        user.publicKey,
      );
      console.log("NFTMINT: ", nftMintATA);
      
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
        config: config,
        rewardsMint: rewardsMint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
    console.log("Your transaction signature", tx);
  });
  
  it("User is Registered!", async () => {
    await airdrop(connection, user.publicKey, 100);

    const tx = await program.methods.registerUser()
      .accountsPartial({
        user: user.publicKey,
        userAccount: userAccount,
      })
      .signers([user])
      .rpc();
    console.log("Your Register User transaction signature", tx);
  });
  
  it("Stake is Created!", async () => {
    await airdrop(connection, user.publicKey, 100);

    const tx = await program.methods.stake()
      .accountsPartial({
        user: user.publicKey,
        nftMint: nftMint,
        userAccount: userAccount,
      })
      .signers([user])
      .rpc();
    console.log("Your Register User transaction signature", tx);
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