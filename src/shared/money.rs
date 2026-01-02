use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

use crate::shared::errors::AppError;

pub fn scale_factor(minor_unit: u8) -> Decimal {
    // 10^minor_unit (exact, sans powu)
    let pow10 = 10i128.pow(minor_unit as u32);
    Decimal::from_i128_with_scale(pow10, 0)
}

/// Convertit un amount_minor (devise source) vers base_amount_minor (devise base)
/// Convention rate: 1 base = rate quote
/// => si src = quote, base = quote / rate
pub fn convert_to_base_minor(
    amount_minor: i64,
    src_minor: u8,
    base_minor: u8,
    rate_base_to_src: Decimal,
) -> Result<i64, AppError> {
    if rate_base_to_src <= Decimal::ZERO {
        return Err(AppError::BadRequest("invalid fx rate".into()));
    }

    let src_scale = scale_factor(src_minor);
    let base_scale = scale_factor(base_minor);

    let amount_src_major = Decimal::from_i64(amount_minor).ok_or(AppError::Internal)? / src_scale;
    let amount_base_major = amount_src_major / rate_base_to_src;

    let base_minor_decimal = (amount_base_major * base_scale).round();
    base_minor_decimal.to_i64().ok_or(AppError::Internal)
}
