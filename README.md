# Brian's Turbin3 Q1 2025 Codebase 🦇

<div align="center">
  <img src="https://github.com/solana-turbin3/Q1_25_Builder_daniel-burlacu/blob/main/turbine-logo-text.png" alt="Logo" width="400">
</div>

This is a comprehensive reference to the codes I wrote and edited while learning in the Solana Turbin3 Program (Builder Cohort).

[🌐 Capstone Frontend](https://crowd-fi-frontend.vercel.app/)

## 🗂️ Content/Folders
- [airdrop](/airdrop/): Contain typescript code written and edited for the pre-requisite task
- [amm-program](/amm-program): Contains Anchor program for an Automated Market Maker Smart Contract
- [crowd_fund_arch_diagram](/crowd_fund_arch_diagram): Contains Architectural Diagram for my Capstone project
- [crowdfi_revamped](/crowdfi_revamped/): My Capstone Project
- [dice](/dice/): Contains Anchor program for a Dice Game
- [escrow](/escrow/): Contains Anchor program for an Escrow Smart Contract
- [marketplace](/marketplace/): Contains Anchor program for NFT Marketplace Smart Contract
- [metaplex_core_learning](/metaplex_core_learning/): Contains THings to learn about the Metaplex Core 
- [pre_req_task_rust](/pre_req_task_rust/): Contains rust codes written and edited for the second pre-requisite task
- [solana-starter](/solana-starter/): Contains SPL relateds edited during the first class
- [staking](/staking/): Contains Anchor program for a Token Staking Protocol
- [vault](/vault): Contains Anchor program code for a vault program
- [Capstone_Letter_of_Intent(LOI).pdf](https://docs.google.com/document/d/1e5ZDsHkfKfeBpUr5ikHsVyoQU9KRSfUrk_yVYZ-yrsA/edit?usp=sharing): Capstone Letter of Intent for the Turbin3 2025 Q1 Builders Cohort


## ⚙️ Setup
- [Install Rust Compiler](https://www.rust-lang.org/tools/install)
- [Install Node](https://nodejs.org/en/download)
- [Install Yarn](https://classic.yarnpkg.com/lang/en/docs/install/)
- [Install Anchor Version Manager](https://www.anchor-lang.com/docs/installation)
- [Install Solana Command Line Interface](https://docs.solana.com/cli/install-solana-cli-tools)


## 🔨 Built with:
- 🦀 [Rust](https://www.rust-lang.org/): A general-purpose programming language emphasizing performance, type safety, and concurrency
- <img src="https://www.svgrepo.com/show/374144/typescript.svg" alt="typescript-logo" width="20"/>[Typescript](https://www.typescriptlang.org/): A free and open-source high-level programming language developed by Microsoft that adds static typing with optional type annotations to JavaScript
- ⚓️ [Anchor](https://www.anchor-lang.com/): A tool that simplifies the process of building Solana programs and emphasis safety.
- <img src="https://avatars.githubusercontent.com/u/84874526?s=200&v=4" alt="metaplex" width="20"/>[Metaplex](https://www.metaplex.com/): Metaplex is a decentralized protocol that allows users to create, sell, and manage digital assets on the Solana blockchain. It's one of the most widely used blockchain protocols for NFTs. 
- Grit: Motivation from the Most High and Personal Grit 


## Anchor Stuffs
- you can initialize an anchor project to use rust for testing with ```anchor init --test-template rust <project_name>```
- ```anchor deploy``` and ```anchor test``` would use the cluster speocief in the Anchor.toml file
- Programs are updated by deploying the program to the same address
- you can close a program to reclaim the SOL allocated to the program
  - once a program is closed, the program ID can not be used to deploy a new program
-

## Other Random Stuffs
- When using the ```close``` constraint in the Account struct, the account passed to the close
constraint MUST be the signer of the instruction
- Everything above u32, can not be represented correctly in JS number and therefore we must use BN to represent Big Numbers
  ```js
  import { BN } from "bn.js";

  let bigNumber = new BN(1);
  ```
- Whenever you add ```anchor-spl``` as a dependenc for a program, also add the ```idl-build``` for it in the features section of the 
Cargo.toml file
- CPI calls cost money!
- As the Amount of CU decreases the chances of getting to the block increases

## Anchor CPIs and Errors
- anchor cpi feature generates CPI helper functions for invoking instructions on existign anchor program
- if you do not have access rto CPI helper function, you can still use invoke and invoke_signed directly
- ```error_code``` attribute macro is used to create Custom Anchor Errors

CPIContext are used to create CPI Context which are similar to COntext, 

Fields on the CPIContext include;
- accounts: List of accounts needed for the CPI Call, Type must implements the ```ToAccountMetas``` adn ```ToAccountInfos<'info>``` traits, which are added by ```#[derive(Accounts)]``` attribute macro
- remaining_Accounts
- program
- signer_seeds

To create a new one that passes the current transaction signature to the transaction that would be perform by the cpi use this

```rust
CPIContext::new(cpi_program, cpi_accounts);
```

You use ```CPIContext::new_with_signer``` to construct  new instance when signing on behalf of  PDA for the CPI

```rust
CPIContext::new_with_signer(cpi_program, cpi_accounts, seeds);
```

When calling another anchor program with a published crate, anchor can generate instruction builders and CPI helper functions for you, simple add the program as a dependency to your ```Cargo.toml``` file and specify the ```cpi`` feature

```Toml
[dependencies]
callee = { path = "../callee", features = ["cpi"] }
```
by doing so, you have enable cpu feature and your program gains access to the callee::cpi module.
the cpi modules turns callee's instruction handlers into Rust functions, these functions takes CPIContext and any extra data needed for the instruction, 

### Errors
Ultimately all programs return the same error type ```ProgramError```, however you can use ```AnchorError``` as an abstraction on top of the ProgramError, 

fields on the AnchorError
```rust
pub struct AnchorError {
  pub error_name: String,
  pub error_code_number: u32,
  pub error_msg: String,
  pub error_origin: Option<ErrorOrigin>,
  pub compared_values: Option<ComparedValues>,
}
```

you can add custom errors like so
```rust
#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}
```
you can use the ```require!``` macro to simplify returning errors

## 🤝 Contribution
While using this material as a reference material for your studies or research, if you do see the need to contribute to the 
content of this repository, please you are welcome to do so, 

- Fork the [base repository](https://github.com/brianobot/TURBIN3-Q1-25) which is the actively maintained version of this repository
- Create a branch for your changes.
- Submit a Pull Request for review


## 👨🏽‍🔧 Maintainer
- Brian Obot <brianobot9@gmail.com>