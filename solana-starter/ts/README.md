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
- All tokens on Solana are effectively data accounts owned by the Token Program.
  - Mint Account are owned by the token programs too!
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


