//! # Litre
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::litre::*;
//! let x = LitreAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Litre>
//!
//! The litre is a metric unit of volume. It is equal to 1 cubic decimetre
//! (dm3), 1000 cubic centimetre (cm3) or 0.001 cubic metre (m3). A cubic
//! decimetre (or litre) occupies a volume of 10 cm × 10 cm × 10 cm (see figure)
//! and is thus equal to one-thousandth of a cubic metre.
//!
//! The symbols are L, l, ℓ.
//!
//! Litre is the standard spelling in nearly all English-speaking areas.

//// Litre

pub struct LitreAsStructI8(pub i8);
pub struct LitreAsStructI16(pub i16);
pub struct LitreAsStructI32(pub i32);
pub struct LitreAsStructI64(pub i64);
pub struct LitreAsStructI128(pub i128);
pub struct LitreAsStructISize(pub isize);
pub struct LitreAsStructU8(pub u8);
pub struct LitreAsStructU16(pub u16);
pub struct LitreAsStructU32(pub u32);
pub struct LitreAsStructU64(pub u64);
pub struct LitreAsStructU128(pub u128);
pub struct LitreAsStructUSize(pub usize);
pub struct LitreAsStructF32(pub f32);
pub struct LitreAsStructF64(pub f64);

pub type LitreAsTypeI8 = i8;
pub type LitreAsTypeI16 = i16;
pub type LitreAsTypeI32 = i32;
pub type LitreAsTypeI64 = i64;
pub type LitreAsTypeI128 = i128;
pub type LitreAsTypeISize = isize;
pub type LitreAsTypeU8 = u8;
pub type LitreAsTypeU16 = u16;
pub type LitreAsTypeU32 = u32;
pub type LitreAsTypeU64 = u64;
pub type LitreAsTypeU128 = u128;
pub type LitreAsTypeUSize = usize;
pub type LitreAsTypeF32 = f32;
pub type LitreAsTypeF64 = f64;
