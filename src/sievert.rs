//! # Sievert
//!
//! Examples:
//!
//! ```rust
//! # use typeables::sievert::*;
//! let x = SievertAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Sievert>
//!
//! The sievert is a unit of equivalent dose (of ionizing radiation).
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is Sv.

//// Sievert

pub struct SievertAsStructI8(pub i8);
pub struct SievertAsStructI16(pub i16);
pub struct SievertAsStructI32(pub i32);
pub struct SievertAsStructI64(pub i64);
pub struct SievertAsStructI128(pub i128);
pub struct SievertAsStructISize(pub isize);
pub struct SievertAsStructU8(pub u8);
pub struct SievertAsStructU16(pub u16);
pub struct SievertAsStructU32(pub u32);
pub struct SievertAsStructU64(pub u64);
pub struct SievertAsStructU128(pub u128);
pub struct SievertAsStructUSize(pub usize);
pub struct SievertAsStructF32(pub f32);
pub struct SievertAsStructF64(pub f64);

pub type SievertAsTypeI8 = i8;
pub type SievertAsTypeI16 = i16;
pub type SievertAsTypeI32 = i32;
pub type SievertAsTypeI64 = i64;
pub type SievertAsTypeI128 = i128;
pub type SievertAsTypeISize = isize;
pub type SievertAsTypeU8 = u8;
pub type SievertAsTypeU16 = u16;
pub type SievertAsTypeU32 = u32;
pub type SievertAsTypeU64 = u64;
pub type SievertAsTypeU128 = u128;
pub type SievertAsTypeUSize = usize;
pub type SievertAsTypeF32 = f32;
pub type SievertAsTypeF64 = f64;
