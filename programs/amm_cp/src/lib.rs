pub mod constants;
pub mod error;
pub mod instructions;
pub mod math;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AdvuNWNdKBhLx3FqAaHeHcMoXKV7Jxw5Ez9pw2Xns5Vo");

/// A minimal constant-product (x * y = k) automated market maker.
///
/// Each pool is a PDA that owns two token vaults and an LP mint. Liquidity
/// providers deposit both tokens and receive LP tokens; traders swap one token
/// for the other paying a configurable fee that accrues to the pool.
#[program]
pub mod amm_cp {
    use super::*;

    /// Create a new pool for an ordered pair of mints with a given swap fee.
    pub fn initialize_pool(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
        instructions::initialize::handler(ctx, fee_bps)
    }

    /// Deposit token A and token B and receive LP tokens.
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a_desired: u64,
        amount_b_desired: u64,
        min_lp_out: u64,
    ) -> Result<()> {
        instructions::deposit::handler(ctx, amount_a_desired, amount_b_desired, min_lp_out)
    }

    /// Burn LP tokens and withdraw the proportional share of both reserves.
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_amount: u64,
        min_amount_a: u64,
        min_amount_b: u64,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, lp_amount, min_amount_a, min_amount_b)
    }

    /// Swap `amount_in` of one token for the other along the constant product.
    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
        min_amount_out: u64,
        a_to_b: bool,
    ) -> Result<()> {
        instructions::swap::handler(ctx, amount_in, min_amount_out, a_to_b)
    }
}
