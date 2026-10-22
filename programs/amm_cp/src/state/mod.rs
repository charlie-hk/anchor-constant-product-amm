use anchor_lang::prelude::*;

/// On-chain state for a single constant-product liquidity pool.
///
/// The pool account is itself the authority (signer, via its PDA) over the two
/// token vaults and the LP mint, so no separate authority account is needed.
#[account]
#[derive(InitSpace)]
pub struct Pool {
    /// Mint of token A (canonical ordering: mint_a < mint_b).
    pub mint_a: Pubkey,
    /// Mint of token B.
    pub mint_b: Pubkey,
    /// Vault holding the pool's reserves of token A.
    pub vault_a: Pubkey,
    /// Vault holding the pool's reserves of token B.
    pub vault_b: Pubkey,
    /// LP mint representing a share of the pool.
    pub lp_mint: Pubkey,
    /// Swap fee charged on the input amount, in basis points.
    pub fee_bps: u16,
    /// Bump of the pool PDA (used when the pool signs CPIs).
    pub bump: u8,
}

impl Pool {
    /// Seeds used by the pool PDA to sign CPIs that move vault funds or mint LP.
    pub fn signer_seeds<'a>(
        mint_a: &'a Pubkey,
        mint_b: &'a Pubkey,
        bump: &'a [u8; 1],
    ) -> [&'a [u8]; 4] {
        [crate::POOL_SEED, mint_a.as_ref(), mint_b.as_ref(), bump]
    }
}
