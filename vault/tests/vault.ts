import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";


describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  
  anchor.setProvider(provider);

  // the program is accessible on the anchor.workspace attribute
  const program = anchor.workspace.Vault as Program<Vault>;

  const connection = provider.connection;

  let signer;
  let vault;
  let vault_state;
  let bump;

  before(async () => {
      signer = anchor.web3.Keypair.generate();

      [vault_state, bump] = PublicKey.findProgramAddressSync([
          Buffer.from("state"),
          signer.publicKey.toBuffer(),
      ], program.programId);
      
      [vault, bump] = PublicKey.findProgramAddressSync([
        vault_state.toBuffer(),
      ], program.programId);

      await airdrop(connection, signer.publicKey, 5);
      // airdrop some SOL to the vault so it does not disappear (make it rent exempt)

      // let rent_exemption_amount = await calculateRentExemption(connection, vault);
      await airdrop(connection, vault, 1);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
    console.log("✅ Your Initialization transaction signature", tx);
  });

  it("Is Deposited!", async () => {
    const tx  = await program.methods
      .deposit(new BN(1_000_000_000))
      .accounts({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc();
    console.log("✅ Your Deposit transaction signature", tx);
  
    // assert that the vault balance increase after the transaction
    let vaultBalance = await getBalance(connection, vault);
    assert.equal(vaultBalance, 2 * 1_000_000_000);

    // assert that the signer balance decrease after the transaction
    // let signerBalance = await getBalance(connection, signer.publicKey); 
    // assert.isTrue(signerBalance <= 5);
  });

  it("Is Withdraw!", async () => {
    const tx = program.methods
      .withdraw(new BN(1))
      .accounts({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc();

      let vaultBalance = await getBalance(connection, vault);
      // assert that the vault balance increase after the transaction
      assert.equal(vaultBalance, 2 * 1_000_000_000);

      // assert that the signer balance decrease after the transaction
      let signerBalance = await getBalance(connection, signer.publicKey); 
      assert.isTrue(signerBalance >= 4.9);
  });

  it("Is Closed!", async () => {
    const tx = program.methods
      .close()
      .accounts({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc();
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

async function calculateRentExemption(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  let accountSize;

  if (accountInfo === null) {
    accountSize = 1000;
  } else {
    accountSize = accountInfo.data.length;
  }
  
  const rentExemptionAmount = await connection.getMinimumBalanceForRentExemption(accountSize);

  return rentExemptionAmount;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}