//! # Siemens
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::siemens::*;
//! let x = SiemensAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Siemens>
//!
//! The siemens is a unit of electrical conductance.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is S.

//// Siemens

pub struct SiemensAsStructI8(pub i8);
pub struct SiemensAsStructI16(pub i16);
pub struct SiemensAsStructI32(pub i32);
pub struct SiemensAsStructI64(pub i64);
pub struct SiemensAsStructI128(pub i128);
pub struct SiemensAsStructISize(pub isize);
pub struct SiemensAsStructU8(pub u8);
pub struct SiemensAsStructU16(pub u16);
pub struct SiemensAsStructU32(pub u32);
pub struct SiemensAsStructU64(pub u64);
pub struct SiemensAsStructU128(pub u128);
pub struct SiemensAsStructUSize(pub usize);
pub struct SiemensAsStructF32(pub f32);
pub struct SiemensAsStructF64(pub f64);

pub type SiemensAsTypeI8 = i8;
pub type SiemensAsTypeI16 = i16;
pub type SiemensAsTypeI32 = i32;
pub type SiemensAsTypeI64 = i64;
pub type SiemensAsTypeI128 = i128;
pub type SiemensAsTypeISize = isize;
pub type SiemensAsTypeU8 = u8;
pub type SiemensAsTypeU16 = u16;
pub type SiemensAsTypeU32 = u32;
pub type SiemensAsTypeU64 = u64;
pub type SiemensAsTypeU128 = u128;
pub type SiemensAsTypeUSize = usize;
pub type SiemensAsTypeF32 = f32;
pub type SiemensAsTypeF64 = f64;
