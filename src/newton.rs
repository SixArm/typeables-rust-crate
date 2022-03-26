//! # Newton
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::newton::*;
//! let x = NewtonAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Newton
//!
//! The newton is a unit of mass.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is N.

//// Newton

pub struct NewtonAsStructI8(pub i8);
pub struct NewtonAsStructI16(pub i16);
pub struct NewtonAsStructI32(pub i32);
pub struct NewtonAsStructI64(pub i64);
pub struct NewtonAsStructI128(pub i128);
pub struct NewtonAsStructISize(pub isize);
pub struct NewtonAsStructU8(pub u8);
pub struct NewtonAsStructU16(pub u16);
pub struct NewtonAsStructU32(pub u32);
pub struct NewtonAsStructU64(pub u64);
pub struct NewtonAsStructU128(pub u128);
pub struct NewtonAsStructUSize(pub usize);
pub struct NewtonAsStructF32(pub f32);
pub struct NewtonAsStructF64(pub f64);

pub type NewtonAsTypeI8 = i8;
pub type NewtonAsTypeI16 = i16;
pub type NewtonAsTypeI32 = i32;
pub type NewtonAsTypeI64 = i64;
pub type NewtonAsTypeI128 = i128;
pub type NewtonAsTypeISize = isize;
pub type NewtonAsTypeU8 = u8;
pub type NewtonAsTypeU16 = u16;
pub type NewtonAsTypeU32 = u32;
pub type NewtonAsTypeU64 = u64;
pub type NewtonAsTypeU128 = u128;
pub type NewtonAsTypeUSize = usize;
pub type NewtonAsTypeF32 = f32;
pub type NewtonAsTypeF64 = f64;
