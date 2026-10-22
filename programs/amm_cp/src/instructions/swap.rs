use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::AmmError;
use crate::math::get_amount_out;
use crate::state::Pool;

#[derive(Accounts)]
pub struct Swap<'info> {
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

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Swap>,
    amount_in: u64,
    min_amount_out: u64,
    a_to_b: bool,
) -> Result<()> {
    require!(amount_in > 0, AmmError::ZeroAmount);

    let (reserve_in, reserve_out) = if a_to_b {
        (ctx.accounts.vault_a.amount, ctx.accounts.vault_b.amount)
    } else {
        (ctx.accounts.vault_b.amount, ctx.accounts.vault_a.amount)
    };

    let amount_out = get_amount_out(
        amount_in,
        reserve_in,
        reserve_out,
        ctx.accounts.pool.fee_bps,
        FEE_DENOMINATOR,
    )?;

    require!(amount_out >= min_amount_out, AmmError::SlippageExceeded);

    // Pull the input tokens from the user into the correct vault.
    let (user_in, vault_in) = if a_to_b {
        (
            ctx.accounts.user_token_a.to_account_info(),
            ctx.accounts.vault_a.to_account_info(),
        )
    } else {
        (
            ctx.accounts.user_token_b.to_account_info(),
            ctx.accounts.vault_b.to_account_info(),
        )
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: user_in,
                to: vault_in,
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_in,
    )?;

    // Send the output tokens from the vault to the user, signed by the pool PDA.
    let (vault_out, user_out) = if a_to_b {
        (
            ctx.accounts.vault_b.to_account_info(),
            ctx.accounts.user_token_b.to_account_info(),
        )
    } else {
        (
            ctx.accounts.vault_a.to_account_info(),
            ctx.accounts.user_token_a.to_account_info(),
        )
    };

    let mint_a = ctx.accounts.pool.mint_a;
    let mint_b = ctx.accounts.pool.mint_b;
    let bump = [ctx.accounts.pool.bump];
    let seeds = Pool::signer_seeds(&mint_a, &mint_b, &bump);
    let signer_seeds: &[&[&[u8]]] = &[&seeds];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: vault_out,
                to: user_out,
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_out,
    )?;

    msg!(
        "Swap: a_to_b={}, amount_in={}, amount_out={}",
        a_to_b,
        amount_in,
        amount_out
    );
    Ok(())
}
