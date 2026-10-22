use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("The two mints must be different")]
    IdenticalMints,
    #[msg("Mints must be supplied in canonical order (mint_a < mint_b)")]
    InvalidMintOrder,
    #[msg("Fee exceeds the maximum allowed")]
    FeeTooHigh,
    #[msg("A supplied token account has the wrong mint")]
    InvalidMint,
    #[msg("A supplied token account has the wrong owner")]
    InvalidOwner,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Resulting liquidity would be zero")]
    ZeroLiquidity,
    #[msg("Pool reserves are empty")]
    EmptyReserves,
    #[msg("Output below the requested minimum (slippage)")]
    SlippageExceeded,
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
