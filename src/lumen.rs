//! # Lumen
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::lumen::*;
//! let x = LumenAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Lumen>
//!
//! The lumen is a unit of luminous flux.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is lm.

//// Lumen

pub struct LumenAsStructI8(pub i8);
pub struct LumenAsStructI16(pub i16);
pub struct LumenAsStructI32(pub i32);
pub struct LumenAsStructI64(pub i64);
pub struct LumenAsStructI128(pub i128);
pub struct LumenAsStructISize(pub isize);
pub struct LumenAsStructU8(pub u8);
pub struct LumenAsStructU16(pub u16);
pub struct LumenAsStructU32(pub u32);
pub struct LumenAsStructU64(pub u64);
pub struct LumenAsStructU128(pub u128);
pub struct LumenAsStructUSize(pub usize);
pub struct LumenAsStructF32(pub f32);
pub struct LumenAsStructF64(pub f64);

pub type LumenAsTypeI8 = i8;
pub type LumenAsTypeI16 = i16;
pub type LumenAsTypeI32 = i32;
pub type LumenAsTypeI64 = i64;
pub type LumenAsTypeI128 = i128;
pub type LumenAsTypeISize = isize;
pub type LumenAsTypeU8 = u8;
pub type LumenAsTypeU16 = u16;
pub type LumenAsTypeU32 = u32;
pub type LumenAsTypeU64 = u64;
pub type LumenAsTypeU128 = u128;
pub type LumenAsTypeUSize = usize;
pub type LumenAsTypeF32 = f32;
pub type LumenAsTypeF64 = f64;
