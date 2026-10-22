use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::AmmError;
use crate::math::integer_sqrt;
use crate::state::Pool;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
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
    ctx: Context<AddLiquidity>,
    amount_a_desired: u64,
    amount_b_desired: u64,
    min_lp_out: u64,
) -> Result<()> {
    require!(
        amount_a_desired > 0 && amount_b_desired > 0,
        AmmError::ZeroAmount
    );

    let reserve_a = ctx.accounts.vault_a.amount as u128;
    let reserve_b = ctx.accounts.vault_b.amount as u128;
    let supply = ctx.accounts.lp_mint.supply as u128;

    // Decide the actual amounts deposited and the LP tokens to mint.
    let (amount_a, amount_b, lp_to_mint) = if supply == 0 {
        // First deposit sets the initial price. LP minted = sqrt(a * b).
        let a = amount_a_desired as u128;
        let b = amount_b_desired as u128;
        let lp = integer_sqrt(a.checked_mul(b).ok_or(AmmError::MathOverflow)?);
        (a, b, lp)
    } else {
        // Keep the existing ratio: find the optimal counterpart amount.
        let amount_b_optimal = (amount_a_desired as u128)
            .checked_mul(reserve_b)
            .ok_or(AmmError::MathOverflow)?
            / reserve_a;

        let (a, b) = if amount_b_optimal <= amount_b_desired as u128 {
            (amount_a_desired as u128, amount_b_optimal)
        } else {
            let amount_a_optimal = (amount_b_desired as u128)
                .checked_mul(reserve_a)
                .ok_or(AmmError::MathOverflow)?
                / reserve_b;
            (amount_a_optimal, amount_b_desired as u128)
        };

        let lp_from_a = a.checked_mul(supply).ok_or(AmmError::MathOverflow)? / reserve_a;
        let lp_from_b = b.checked_mul(supply).ok_or(AmmError::MathOverflow)? / reserve_b;
        (a, b, lp_from_a.min(lp_from_b))
    };

    require!(lp_to_mint > 0, AmmError::ZeroLiquidity);
    require!(lp_to_mint >= min_lp_out as u128, AmmError::SlippageExceeded);

    let amount_a_u64 = u64::try_from(amount_a).map_err(|_| AmmError::MathOverflow)?;
    let amount_b_u64 = u64::try_from(amount_b).map_err(|_| AmmError::MathOverflow)?;
    let lp_u64 = u64::try_from(lp_to_mint).map_err(|_| AmmError::MathOverflow)?;

    // Transfer token A and token B from the user into the vaults.
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_a.to_account_info(),
                to: ctx.accounts.vault_a.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_a_u64,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.vault_b.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_b_u64,
    )?;

    // Mint LP tokens to the user, signed by the pool PDA.
    let mint_a = ctx.accounts.pool.mint_a;
    let mint_b = ctx.accounts.pool.mint_b;
    let bump = [ctx.accounts.pool.bump];
    let seeds = Pool::signer_seeds(&mint_a, &mint_b, &bump);
    let signer_seeds: &[&[&[u8]]] = &[&seeds];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        lp_u64,
    )?;

    msg!(
        "Added liquidity: a={}, b={}, lp_minted={}",
        amount_a_u64,
        amount_b_u64,
        lp_u64
    );
    Ok(())
}
