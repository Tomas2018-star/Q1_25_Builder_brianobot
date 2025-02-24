# Token Extension

These are optional features that can be added to Token Mint or Token Accounts provided
through instructions by the Token 2022 Program.

Each extension adds a specific state that must be initialized during the mint or token account
creation, however extensions can not be added after an account has been created. this decision means
the token creator must decide on all token features ahead before token creation.

These helper functions can be found in the ```token_2022_extention``` module of the ```anchor-spl``` crate.

> **_NOTE:_** that while the anchor-spl crate provides helper functions for working with Token Extensions, not all extension instructions have been fully implemented yet. You may need to manually implement CPI calls for some extension instructions.
