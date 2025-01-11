use solana_idlgen::idlgen; 
idlgen!({
    "version": "0.1.0",
    "name": "turbin3_prereq",
    "metadata": {
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
    },
    "instructions": [
        {
        "name": "complete",
        "accounts": [
            {
                "name": "signer",
                "isMut": true,
                "isSigner": true
            },
            {
                "name": "prereq",
                "isMut": true,
                "isSigner": false
            },
            {
                "name": "systemProgram",
                "isMut": false,
                "isSigner": false
            }
        ],
        "args": [
            {
                "name": "github",
                "type": "bytes"
            }
        ]
        },
        {
        "name": "update",
        "accounts": [
            {
                "name": "signer",
                "isMut": true,
                "isSigner": true
            },
            {
                "name": "prereq",
                "isMut": true,
                "isSigner": false
            },
            {
                "name": "systemProgram",
                "isMut": false,
                "isSigner": false
            }
        ],
        "args": [
            {
                "name": "github",
                "type": "bytes"
            }
        ]
        }
    ],
    "accounts": [
        {
        "name": "PrereqAccount",
        "type": {
            "kind": "struct",
            "fields": [
            {
                "name": "github",
                "type": "bytes"
            },
            {
                "name": "key",
                "type": "publicKey"
            }
            ]
        }
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "InvalidGithubAccount",
            "msg": "Invalid Github account"
        }
    ]
}); 
    