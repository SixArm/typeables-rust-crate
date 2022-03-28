//! # Metre
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::metre::*;
//! let x = MetreAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Metre>
//!
//! The metre is the SI base unit of length.
//!
//! The symbol is m.
//!
//! Metre is the standard spelling of the metric unit for length in nearly all
//! English-speaking nations.
//!
//! Compare:
//!
//!   * Metre is for length
//!
//!   * Metre^2 is for area
//!
//!   * Metre^3 is for volume
//!
//!   * Metre Per Second is for speed

pub struct MetreAsStructI8(pub i8);
pub struct MetreAsStructI16(pub i16);
pub struct MetreAsStructI32(pub i32);
pub struct MetreAsStructI64(pub i64);
pub struct MetreAsStructI128(pub i128);
pub struct MetreAsStructISize(pub isize);
pub struct MetreAsStructU8(pub u8);
pub struct MetreAsStructU16(pub u16);
pub struct MetreAsStructU32(pub u32);
pub struct MetreAsStructU64(pub u64);
pub struct MetreAsStructU128(pub u128);
pub struct MetreAsStructUSize(pub usize);
pub struct MetreAsStructF32(pub f32);
pub struct MetreAsStructF64(pub f64);

pub type MetreAsTypeI8 = i8;
pub type MetreAsTypeI16 = i16;
pub type MetreAsTypeI32 = i32;
pub type MetreAsTypeI64 = i64;
pub type MetreAsTypeI128 = i128;
pub type MetreAsTypeISize = isize;
pub type MetreAsTypeU8 = u8;
pub type MetreAsTypeU16 = u16;
pub type MetreAsTypeU32 = u32;
pub type MetreAsTypeU64 = u64;
pub type MetreAsTypeU128 = u128;
pub type MetreAsTypeUSize = usize;
pub type MetreAsTypeF32 = f32;
pub type MetreAsTypeF64 = f64;
