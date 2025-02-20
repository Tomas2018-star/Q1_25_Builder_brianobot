# Dice Game

## Note on Sysvar
Sysvars in Solana are read-only accounts that store blockchain-related information, such as recent block hashes, slot history, rent data, and clock (timestamp) details. These are built-in accounts provided by the Solana runtime and are accessible to all programs without requiring explicit account inclusion.

### Why Use Sysvars?
- Efficiently access on-chain state (e.g., current slot, epoch, rent exemption info).
- Reduce transaction size by avoiding extra accounts.
- Read-only & immutable, ensuring security.

Most Sysvars can be accessed by 
- accessing their ```get()``` function.
```rust 
let clock = Clock::get()?.unix_timestamp;
let epoch = EpochSchedule::get()?;
let fees = Fee::get()?;
let epoch_rewards = EpochRewards::get()?;
```

- The second is to pass the sysvar to the program as an account by including its address as one of the accounts in the Instruction and then deserializing the data during execution. Access to sysvars accounts is always readonly.
```rust
let clock_sysvar_info = next_account_info(account_info_iter)?;
let clock = Clock::from_account_info(&clock_sysvar_info)?;
```

