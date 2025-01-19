# Metaplex Protocol

Write description for the metaplex protocol here

## Metaplex Candy Machine ([docs](https://developers.metaplex.com/candy-machine))
This is solana program (smart contract) designed to allow developers to reliably sell NFTS on the solana blockchain.
This document contains codes and steps for candy machine v3, for other version please check the docs as linked above

> The name refers to the vending machines that dispense candy for coins via a mechanical crank. In this case the candy are NFTs and the payment is SOL or a SPL token.

### Features of the Candy Machine
- Accept payment in SOl, NFTs or any other Solana Token
- Restrict launch via start/end dates, control mint limits etc
- Protect launch against bots via configurable bot taxes and gate keepers like captchas
- Restrict minting to specific NFT/Tokens holders or to a curated list of wallets
- Create multiple minting groups with different set of rules
- Candy Guards: allows for configurable rules to control minting of NFTs

#### Candy Guard
Some guards can be set up to control the minting process of any configured candy machine , some guards are
- Sol Payment
- Start date: controls the time which minting can start
- Mint Limit: controls the mint amount per wallet
- Bot Tax: this guard mutates itself when one of the guards fails, to ensure that retries are discouraged in the case of bots etc

#### Life cycle of a candy machine
- create and configure an instance of candy machine
- Add items to the candy machine 
- Mint: Nfts are minted from the candy machine
- At this point the candy machine can be deleted

#### Code interaction with the candy machine program
- js: we can use the umi framework to interact with the candy machine program in js codes
- 

