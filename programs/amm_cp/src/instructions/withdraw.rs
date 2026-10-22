use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::AmmError;
use crate::math::mul_div;
use crate::state::Pool;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [POOL_SEED, pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut, address = pool.vault_a)]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(mut, address = pool.vault_b)]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(mut, address = pool.lp_mint)]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_a.mint == pool.mint_a @ AmmError::InvalidMint,
        constraint = user_token_a.owner == user.key() @ AmmError::InvalidOwner,
    )]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_b.mint == pool.mint_b @ AmmError::InvalidMint,
        constraint = user_token_b.owner == user.key() @ AmmError::InvalidOwner,
    )]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_lp.mint == pool.lp_mint @ AmmError::InvalidMint,
        constraint = user_lp.owner == user.key() @ AmmError::InvalidOwner,
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RemoveLiquidity>,
    lp_amount: u64,
    min_amount_a: u64,
    min_amount_b: u64,
) -> Result<()> {
    require!(lp_amount > 0, AmmError::ZeroAmount);

    let reserve_a = ctx.accounts.vault_a.amount as u128;
    let reserve_b = ctx.accounts.vault_b.amount as u128;
    let supply = ctx.accounts.lp_mint.supply as u128;
    require!(supply > 0, AmmError::EmptyReserves);

    // Proportional share of each reserve.
    let amount_a = mul_div(lp_amount as u128, reserve_a, supply)?;
    let amount_b = mul_div(lp_amount as u128, reserve_b, supply)?;

    require!(amount_a > 0 && amount_b > 0, AmmError::ZeroAmount);
    require!(
        amount_a >= min_amount_a as u128 && amount_b >= min_amount_b as u128,
        AmmError::SlippageExceeded
    );

    let amount_a_u64 = u64::try_from(amount_a).map_err(|_| AmmError::MathOverflow)?;
    let amount_b_u64 = u64::try_from(amount_b).map_err(|_| AmmError::MathOverflow)?;

    // Burn the user's LP tokens first.
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        lp_amount,
    )?;

    // Transfer the underlying tokens back to the user, signed by the pool PDA.
    let mint_a = ctx.accounts.pool.mint_a;
    let mint_b = ctx.accounts.pool.mint_b;
    let bump = [ctx.accounts.pool.bump];
    let seeds = Pool::signer_seeds(&mint_a, &mint_b, &bump);
    let signer_seeds: &[&[&[u8]]] = &[&seeds];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_a.to_account_info(),
                to: ctx.accounts.user_token_a.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_a_u64,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_b.to_account_info(),
                to: ctx.accounts.user_token_b.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_b_u64,
    )?;

    msg!(
        "Removed liquidity: lp_burned={}, a_out={}, b_out={}",
        lp_amount,
        amount_a_u64,
        amount_b_u64
    );
    Ok(())
}
