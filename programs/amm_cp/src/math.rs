use crate::error::AmmError;
use anchor_lang::prelude::*;

/// Integer square root (floor) using the Babylonian method, on u128.
pub fn integer_sqrt(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let mut x = value;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + value / x) / 2;
    }
    x
}

/// `a * b / denominator` computed in u128, returned as u128, with overflow checks.
pub fn mul_div(a: u128, b: u128, denominator: u128) -> Result<u128> {
    let prod = a.checked_mul(b).ok_or(AmmError::MathOverflow)?;
    let res = prod
        .checked_div(denominator)
        .ok_or(AmmError::MathOverflow)?;
    Ok(res)
}

/// Constant-product output amount for a swap, net of the pool fee.
///
/// `amount_out = reserve_out * amount_in_after_fee / (reserve_in + amount_in_after_fee)`
pub fn get_amount_out(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_bps: u16,
    fee_denominator: u64,
) -> Result<u64> {
    require!(amount_in > 0, AmmError::ZeroAmount);
    require!(reserve_in > 0 && reserve_out > 0, AmmError::EmptyReserves);

    let fee_factor = (fee_denominator as u128)
        .checked_sub(fee_bps as u128)
        .ok_or(AmmError::MathOverflow)?;

    let amount_in_after_fee = mul_div(amount_in as u128, fee_factor, fee_denominator as u128)?;

    let numerator = (reserve_out as u128)
        .checked_mul(amount_in_after_fee)
        .ok_or(AmmError::MathOverflow)?;
    let denominator = (reserve_in as u128)
        .checked_add(amount_in_after_fee)
        .ok_or(AmmError::MathOverflow)?;

    let amount_out = numerator
        .checked_div(denominator)
        .ok_or(AmmError::MathOverflow)?;

    u64::try_from(amount_out).map_err(|_| AmmError::MathOverflow.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_works() {
        assert_eq!(integer_sqrt(0), 0);
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(8), 2);
        assert_eq!(integer_sqrt(9), 3);
        assert_eq!(integer_sqrt(1_000_000), 1000);
    }

    #[test]
    fn amount_out_matches_formula() {
        // reserves 1_000 / 1_000, swap 100 in, 0.3% fee.
        // in_after_fee = 100 * 9970 / 10000 = 99 (floor of 99.7)
        // out = 1000*99 / (1000+99) = 99000/1099 = 90
        let out = get_amount_out(100, 1_000, 1_000, 30, 10_000).unwrap();
        assert_eq!(out, 90);
    }
}
