import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token"
import { confirmTransaction } from "@solana-developers/helpers";
import { Metadata, MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";


describe("marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;
  const program = anchor.workspace.Marketplace as Program<Marketplace>;

  let admin = anchor.web3.Keypair.generate();
  let maker = anchor.web3.Keypair.generate();
  let taker = anchor.web3.Keypair.generate();

  let maker_ata;
  let taker_ata;

  let marketplace: PublicKey;
  let marketplaceBump;
  let treasury;
  let treasuryBump;
  let rewardMint;
  let rewardMintBump;
  let makerMint;
  let takerMint;
  let metadata;
  let metadataBump;
  let masterEdition;
  let masterEditionBump;
  
  before(async () => {
    [marketplace, marketplaceBump] = PublicKey.findProgramAddressSync([
      Buffer.from("marketplace"),
      Buffer.from("Marketplace Test Name"),
    ], program.programId);
    console.log("✅ Marketplace PDA: ", marketplace);

    [treasury, treasuryBump] = PublicKey.findProgramAddressSync([
      Buffer.from("treasury"),
      marketplace.toBuffer(),
    ], program.programId);
    console.log("✅ Treasury PDA: ", treasury);
    
    [rewardMint, rewardMintBump] = PublicKey.findProgramAddressSync([
      Buffer.from("rewards_mint"),
      marketplace.toBuffer(),
    ], program.programId);
    console.log("✅ Reward Mint PDA: ", rewardMint);

    await airdrop(connection, maker.publicKey, 100);
    await airdrop(connection, taker.publicKey, 100);

    makerMint = await createMint(connection, maker, maker.publicKey, null, 6);
    takerMint = await createMint(connection, taker, taker.publicKey, null, 6);

    console.log("✅ Maker Mint: ", makerMint);
    console.log("✅ Taker Mint: ", takerMint);
    
    // NOTE: it is very easy to assume that the return value 
    // is of the type Public, this is not the case, the type is an Account
    maker_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      makerMint,
      maker.publicKey
    );
    console.log("✅ Maker Ata: ", maker_ata);

    taker_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      takerMint,
      taker.publicKey
    );
    console.log("✅ Taker Ata: ", taker_ata);

    await mintTo(connection, maker, makerMint, maker_ata.address, maker, 1000);
    console.log("✅ Minted to Maker Ata: ");
    await mintTo(connection, taker, takerMint, taker_ata.address, taker, 1000);
    console.log("✅ Minted to Taker Ata: ");

    // [metadata, metadataBump] = PublicKey.findProgramAddressSync([
    //   Buffer.from("metadata"),
    //   makerMint.toBuffer(),
    //   MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    // ], MPL_TOKEN_METADATA_PROGRAM_ID);
    // console.log("✅ Metadata account PDA: ", metadata);
    
    // [masterEdition, masterEditionBump] = PublicKey.findProgramAddressSync([
    //   Buffer.from("metadata"),
    //   Buffer.from("edition"),
    //   makerMint.toBuffer(),
    //   MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    // ], MPL_TOKEN_METADATA_PROGRAM_ID);
    // console.log("✅ Master Edition account PDA: ", masterEdition);

  });
  

  it("Marketplace Is initialized!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .initialize(
        "Marketplace Test Name",
        10 // TODO: Research on When exactly to use BN for numeric types in ts
      )
      .accountsPartial({
        admin: admin.publicKey,
        marketplace: marketplace,
        // treasury: treasury,
        // rewardsMint: rewardMint,
        tokenProgram: TOKEN_2022_PROGRAM_ID // this is needed to know which token program to use
      })
      .signers([admin])
      .rpc();
    console.log("Your Marketplace Initialization transaction signature", tx);
  });
  
  it("List is Created", async () => {
    await airdrop(program.provider.connection, maker.publicKey, 100)

    const tx = await program.methods
      .list(new BN(1000))
      .accountsPartial({
        maker: maker.publicKey,
        makerMint: makerMint,
        makerAta: maker_ata,
        marketplace: marketplace,
        // metadata: metadata,
        // collectionMint: 
        // masterEdition: masterEdition,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    console.log("Your List transaction signature", tx);
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