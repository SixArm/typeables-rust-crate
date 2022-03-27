//! # Henry
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::henry::*;
//! let x = HenryAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Henry>
//!
//! The henry is a unit of electrical inductance.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is H.

//// Henry

pub struct HenryAsStructI8(pub i8);
pub struct HenryAsStructI16(pub i16);
pub struct HenryAsStructI32(pub i32);
pub struct HenryAsStructI64(pub i64);
pub struct HenryAsStructI128(pub i128);
pub struct HenryAsStructISize(pub isize);
pub struct HenryAsStructU8(pub u8);
pub struct HenryAsStructU16(pub u16);
pub struct HenryAsStructU32(pub u32);
pub struct HenryAsStructU64(pub u64);
pub struct HenryAsStructU128(pub u128);
pub struct HenryAsStructUSize(pub usize);
pub struct HenryAsStructF32(pub f32);
pub struct HenryAsStructF64(pub f64);

pub type HenryAsTypeI8 = i8;
pub type HenryAsTypeI16 = i16;
pub type HenryAsTypeI32 = i32;
pub type HenryAsTypeI64 = i64;
pub type HenryAsTypeI128 = i128;
pub type HenryAsTypeISize = isize;
pub type HenryAsTypeU8 = u8;
pub type HenryAsTypeU16 = u16;
pub type HenryAsTypeU32 = u32;
pub type HenryAsTypeU64 = u64;
pub type HenryAsTypeU128 = u128;
pub type HenryAsTypeUSize = usize;
pub type HenryAsTypeF32 = f32;
pub type HenryAsTypeF64 = f64;
