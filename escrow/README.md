# Escrow Program

This Anchor Programs allows for the trustless swapping of tokens (of different mint) between two parties.

## Take Account Struct Explained
- taker: Account bringing token b to swap for already collected token a
- maker: Account that brought the token and initiated the escrow process
- mint_a: Mint Account for Token A
  - needed to confirm that token received by taker is of the expected mint
- mint_b: Mint Account for Token B
  - needed to confirm that token brought by taker is of the expected mint
- taker_ata_a: ATA needed to store Taker token A
- taker_ata_b: ATA needed to store Taker token B
- maker_ata_b: ATA needed to store Maker token B
- escrow: Account that holds configuration for the Escrow
- vault: Account that stores Token gotten from the maker
- token_program: Needed to carry out transfer on ATA
- system_program: Needed to create Account
- associated_token_program: Needed to Createa ATA

## Notes
- It is not good practice to close ATAs (Associated Token Accounts)
- 
