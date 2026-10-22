use anchor_lang::prelude::*;

/// PDA seed for the pool state account.
pub const POOL_SEED: &[u8] = b"pool";
/// PDA seed for the pool's token-A vault.
pub const VAULT_A_SEED: &[u8] = b"vault_a";
/// PDA seed for the pool's token-B vault.
pub const VAULT_B_SEED: &[u8] = b"vault_b";
/// PDA seed for the pool's LP mint.
pub const LP_MINT_SEED: &[u8] = b"lp_mint";

/// Fee denominator. Fees are expressed in basis points (1 bp = 1/10_000).
#[constant]
pub const FEE_DENOMINATOR: u64 = 10_000;
/// Maximum swap fee the pool may be created with (10%).
#[constant]
pub const MAX_FEE_BPS: u16 = 1_000;
/// Decimals used by every pool's LP mint.
#[constant]
pub const LP_MINT_DECIMALS: u8 = 9;
