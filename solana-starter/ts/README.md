# Concepts Convered in the Samples

- PDA (Program Derived Address)
- SPL Token (Solana Program Library Token)
- Mint Account 
- Token Account
- (ATA) Associated Token Account
- IDL (Interface Design Language)


## Notes:
- PDAs are signed for by the program that created them (their owner)
- PDAs require seeds, a program id and a bump to be created 
- All SPL Tokens require a Mint Account before they are created 
  - the ```spl-token library``` provides a function to create and initialize a mint account
  - all new tokens are minted from a mint account
- Token account holds the balance for each token a user owns, it is linked to a single mint account
  - technically if i own 5 different tokens, i would need atleast 5 token accounts to hold their balance
  - this means a user can have more than 1 token account for any 1 token,
- Associated Token account is a deterministic token account tha can be derived from the mint account and the wallet address
  - some useful tyepscript spl-token library function for interacting with ATA are;
    - ```getAssociatedTokenAccount```
    - ```createAssociatedTokenAccount```
    - ```getOrCreateAssociatedTokenAccount```
- 


## Random Stuffs that might make you see stuffs differently
- Think of Tokens as just numbers in a Token Account
- All tokens on Solana are effectively data accounts owned by the Token Program.
  - Mint Account are owned by the token programs too!
  - The account that created the mint account is the mint authority behind the mint, but the owner is the token program
- Tokens are uniquiely identified by the adderss of the Mint Account
- Mint Account is effectively the global counter for a specific token stores data like 
  - supply
  - decimal
  - mint authority
  - freeze authority
  - is_initialized
- Token Account is used to hold the unit balance of a particular token, some it attributes are;
  - mint
  - owner 
  - amount
- Associated Token Account are deterministic token account derived from the wallet address and the token mint
  inputs needed to create an ATA are
  - wallet address of the owner
  - token mint address
  - token program address, since it owns the token account
- ATA can be created for Programs, other PDAs and anything at all on-chain that has a public key
- ATA is a subset of the Token Account, since both can hold the balance of some token, but ATA are easier to find
- Token Extensions introduce a standard way to extend the standard token functionality; this extensions include;
    - confidential transfer
    - custom tranfer logic
    - extended metadata
- Metadata can be added to the Mint Account directly through the ```MetadataPointer``` and the ```TokenMetadata```

- The ```TokenMetadata Interface``` is designed to serve as a standard way of adding metadata to tokens by 
defining the data structure and set of instructions for handling metadata, with this interface, apps can generically access
token metadata 
  - fields in the token metadata data structure
    - update authority
    - mint
    - name
    - symbol
    - uri
    - additional_metadata


## Creating An NFT
Steps involved
- UPload NFT Media (Picture, Video, etc) to a decentralized storage ()
- Creating and Uploading the NFT Metadata to the blockchain
- Minting the NFT on Solana Blockchain

## Create A Token
Steps involved
- Create a Mint Account (Token Mint)
- Create Token Account / Associated Token Account
- Mint Token into Token Account / Associated Token Account


### Metaplex
This is solana main provider of token standard. they create tools to support creation of open NFT Protocol.

Tokens like ```USDC``` when viewed on the explorer has things like a picture and nice name and symbol, 
these attributes are obtained from the metadata linked to the Token's Mint account and the standard for defining
this attribute are provided by the Metaplex Token standard.

Tokens can be created with other standards too that are not metaplex, there is flexibility to this 
and using a generally acknowledged standard is easier and better for interoperability

#### Metaplex Token Standards
- NonFungible: has a Master Edition
- NonFungibleEdition: Has an Edition Account
- FungibleAsset: Has no Master Edition and uses zerp decimals places
- Fungible: Has no Master Edition and uses at least one decimal palce

#### Metadata Account
- can be set to be mutable
- JSON standards follows closely with other chains

Metadata account for a mint account is a PDA that holds metadata for a mint account and it is derived from 
- 'metadata'
- mint_id
- metadata_program_id

Some the fields expected in the metadata account are;
- name (string): Name of the Asset
- symbol (string): Symbol of the Asset
- uri (string): URI to external JSON representing the asset
- creators (array): public keys of each creator
- update_authority (string): public key of the metadata owner
- primary_sale_happened (boolean): 
- 

## Finding PDAs
Generic way to find PDAs in Typescript library

```typescript

const metadata_seeds = [
  Buffer.from("metadata"),
  TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  mint.toBuffer(),
];

const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
  metadata_seeds,
  TOKEN_METADATA_PROGRAM_ID
);

```