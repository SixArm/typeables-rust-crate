//! # Katal
//!
//! Examples:
//!
//! ```rust
//! # use typeables::katal::*;
//! let x = KatalAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Katal>
//!
//! The katal is a unit of catalytic activity.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is kat.

//// Katal

pub struct KatalAsStructI8(pub i8);
pub struct KatalAsStructI16(pub i16);
pub struct KatalAsStructI32(pub i32);
pub struct KatalAsStructI64(pub i64);
pub struct KatalAsStructI128(pub i128);
pub struct KatalAsStructISize(pub isize);
pub struct KatalAsStructU8(pub u8);
pub struct KatalAsStructU16(pub u16);
pub struct KatalAsStructU32(pub u32);
pub struct KatalAsStructU64(pub u64);
pub struct KatalAsStructU128(pub u128);
pub struct KatalAsStructUSize(pub usize);
pub struct KatalAsStructF32(pub f32);
pub struct KatalAsStructF64(pub f64);

pub type KatalAsTypeI8 = i8;
pub type KatalAsTypeI16 = i16;
pub type KatalAsTypeI32 = i32;
pub type KatalAsTypeI64 = i64;
pub type KatalAsTypeI128 = i128;
pub type KatalAsTypeISize = isize;
pub type KatalAsTypeU8 = u8;
pub type KatalAsTypeU16 = u16;
pub type KatalAsTypeU32 = u32;
pub type KatalAsTypeU64 = u64;
pub type KatalAsTypeU128 = u128;
pub type KatalAsTypeUSize = usize;
pub type KatalAsTypeF32 = f32;
pub type KatalAsTypeF64 = f64;
