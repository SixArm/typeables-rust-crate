//! # Ohm
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::ohm::*;
//! let x = OhmAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Ohm
//! 
//! The ohm is a unit of  of electrical resistance.
//! 
//! It is a derived unit in the International System of Units (SI).
//!
//! The SI unit symbol is Ω.
//!
//! The ohm named after German physicist Georg Ohm. 
//! 
//! Various empirically derived standard units for electrical resistance were
//! developed in connection with early telegraphy practice, and the British
//! Association for the Advancement of Science proposed a unit derived from
//! existing units of mass, length and time, and of a convenient scale for
//! practical work as early as 1861. As of 2020, the definition of the ohm is
//! expressed in terms of the quantum Hall effect. 

//// Ohm

pub struct OhmAsStructI8(pub i8);
pub struct OhmAsStructI16(pub i16);
pub struct OhmAsStructI32(pub i32);
pub struct OhmAsStructI64(pub i64);
pub struct OhmAsStructI128(pub i128);
pub struct OhmAsStructISize(pub isize);
pub struct OhmAsStructU8(pub u8);
pub struct OhmAsStructU16(pub u16);
pub struct OhmAsStructU32(pub u32);
pub struct OhmAsStructU64(pub u64);
pub struct OhmAsStructU128(pub u128);
pub struct OhmAsStructUSize(pub usize);
pub struct OhmAsStructF32(pub f32);
pub struct OhmAsStructF64(pub f64);

pub type OhmAsTypeI8 = i8;
pub type OhmAsTypeI16 = i16;
pub type OhmAsTypeI32 = i32;
pub type OhmAsTypeI64 = i64;
pub type OhmAsTypeI128 = i128;
pub type OhmAsTypeISize = isize;
pub type OhmAsTypeU8 = u8;
pub type OhmAsTypeU16 = u16;
pub type OhmAsTypeU32 = u32;
pub type OhmAsTypeU64 = u64;
pub type OhmAsTypeU128 = u128;
pub type OhmAsTypeUSize = usize;
pub type OhmAsTypeF32 = f32;
pub type OhmAsTypeF64 = f64;
