/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/amm_cp.json`.
 */
export type AmmCp = {
  "address": "AdvuNWNdKBhLx3FqAaHeHcMoXKV7Jxw5Ez9pw2Xns5Vo",
  "metadata": {
    "name": "ammCp",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Constant-product AMM (Uniswap-v2 style) on Solana, built with Anchor"
  },
  "docs": [
    "A minimal constant-product (x * y = k) automated market maker.",
    "",
    "Each pool is a PDA that owns two token vaults and an LP mint. Liquidity",
    "providers deposit both tokens and receive LP tokens; traders swap one token",
    "for the other paying a configurable fee that accrues to the pool."
  ],
  "instructions": [
    {
      "name": "addLiquidity",
      "docs": [
        "Deposit token A and token B and receive LP tokens."
      ],
      "discriminator": [
        181,
        157,
        89,
        67,
        143,
        182,
        52,
        72
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint_a",
                "account": "pool"
              },
              {
                "kind": "account",
                "path": "pool.mint_b",
                "account": "pool"
              }
            ]
          }
        },
        {
          "name": "vaultA",
          "writable": true
        },
        {
          "name": "vaultB",
          "writable": true
        },
        {
          "name": "lpMint",
          "writable": true
        },
        {
          "name": "userTokenA",
          "writable": true
        },
        {
          "name": "userTokenB",
          "writable": true
        },
        {
          "name": "userLp",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amountADesired",
          "type": "u64"
        },
        {
          "name": "amountBDesired",
          "type": "u64"
        },
        {
          "name": "minLpOut",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializePool",
      "docs": [
        "Create a new pool for an ordered pair of mints with a given swap fee."
      ],
      "discriminator": [
        95,
        180,
        10,
        172,
        84,
        174,
        232,
        40
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "mintA"
        },
        {
          "name": "mintB"
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "mintA"
              },
              {
                "kind": "account",
                "path": "mintB"
              }
            ]
          }
        },
        {
          "name": "vaultA",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              }
            ]
          }
        },
        {
          "name": "vaultB",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  98
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              }
            ]
          }
        },
        {
          "name": "lpMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  112,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "feeBps",
          "type": "u16"
        }
      ]
    },
    {
      "name": "removeLiquidity",
      "docs": [
        "Burn LP tokens and withdraw the proportional share of both reserves."
      ],
      "discriminator": [
        80,
        85,
        209,
        72,
        24,
        206,
        177,
        108
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint_a",
                "account": "pool"
              },
              {
                "kind": "account",
                "path": "pool.mint_b",
                "account": "pool"
              }
            ]
          }
        },
        {
          "name": "vaultA",
          "writable": true
        },
        {
          "name": "vaultB",
          "writable": true
        },
        {
          "name": "lpMint",
          "writable": true
        },
        {
          "name": "userTokenA",
          "writable": true
        },
        {
          "name": "userTokenB",
          "writable": true
        },
        {
          "name": "userLp",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "lpAmount",
          "type": "u64"
        },
        {
          "name": "minAmountA",
          "type": "u64"
        },
        {
          "name": "minAmountB",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swap",
      "docs": [
        "Swap `amount_in` of one token for the other along the constant product."
      ],
      "discriminator": [
        248,
        198,
        158,
        145,
        225,
        117,
        135,
        200
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint_a",
                "account": "pool"
              },
              {
                "kind": "account",
                "path": "pool.mint_b",
                "account": "pool"
              }
            ]
          }
        },
        {
          "name": "vaultA",
          "writable": true
        },
        {
          "name": "vaultB",
          "writable": true
        },
        {
          "name": "userTokenA",
          "writable": true
        },
        {
          "name": "userTokenB",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amountIn",
          "type": "u64"
        },
        {
          "name": "minAmountOut",
          "type": "u64"
        },
        {
          "name": "aToB",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "pool",
      "discriminator": [
        241,
        154,
        109,
        4,
        17,
        177,
        109,
        188
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "identicalMints",
      "msg": "The two mints must be different"
    },
    {
      "code": 6001,
      "name": "invalidMintOrder",
      "msg": "Mints must be supplied in canonical order (mint_a < mint_b)"
    },
    {
      "code": 6002,
      "name": "feeTooHigh",
      "msg": "Fee exceeds the maximum allowed"
    },
    {
      "code": 6003,
      "name": "invalidMint",
      "msg": "A supplied token account has the wrong mint"
    },
    {
      "code": 6004,
      "name": "invalidOwner",
      "msg": "A supplied token account has the wrong owner"
    },
    {
      "code": 6005,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6006,
      "name": "zeroLiquidity",
      "msg": "Resulting liquidity would be zero"
    },
    {
      "code": 6007,
      "name": "emptyReserves",
      "msg": "Pool reserves are empty"
    },
    {
      "code": 6008,
      "name": "slippageExceeded",
      "msg": "Output below the requested minimum (slippage)"
    },
    {
      "code": 6009,
      "name": "mathOverflow",
      "msg": "Arithmetic overflow"
    }
  ],
  "types": [
    {
      "name": "pool",
      "docs": [
        "On-chain state for a single constant-product liquidity pool.",
        "",
        "The pool account is itself the authority (signer, via its PDA) over the two",
        "token vaults and the LP mint, so no separate authority account is needed."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintA",
            "docs": [
              "Mint of token A (canonical ordering: mint_a < mint_b)."
            ],
            "type": "pubkey"
          },
          {
            "name": "mintB",
            "docs": [
              "Mint of token B."
            ],
            "type": "pubkey"
          },
          {
            "name": "vaultA",
            "docs": [
              "Vault holding the pool's reserves of token A."
            ],
            "type": "pubkey"
          },
          {
            "name": "vaultB",
            "docs": [
              "Vault holding the pool's reserves of token B."
            ],
            "type": "pubkey"
          },
          {
            "name": "lpMint",
            "docs": [
              "LP mint representing a share of the pool."
            ],
            "type": "pubkey"
          },
          {
            "name": "feeBps",
            "docs": [
              "Swap fee charged on the input amount, in basis points."
            ],
            "type": "u16"
          },
          {
            "name": "bump",
            "docs": [
              "Bump of the pool PDA (used when the pool signs CPIs)."
            ],
            "type": "u8"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "feeDenominator",
      "docs": [
        "Fee denominator. Fees are expressed in basis points (1 bp = 1/10_000)."
      ],
      "type": "u64",
      "value": "10000"
    },
    {
      "name": "lpMintDecimals",
      "docs": [
        "Decimals used by every pool's LP mint."
      ],
      "type": "u8",
      "value": "9"
    },
    {
      "name": "maxFeeBps",
      "docs": [
        "Maximum swap fee the pool may be created with (10%)."
      ],
      "type": "u16",
      "value": "1000"
    }
  ]
};
