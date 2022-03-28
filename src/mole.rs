//! # Mole
//!
//! Examples:
//!
//! ```rust
//! # use typeables::mole::*;
//! let x = MoleAsStructF64(1.0);
//! ```
//!
//! <https://en.wikipedia.org/wiki/Mole_(unit)>
//!
//! The mole is a unit of amount of substance.
//!
//! It is a base unit in the International System of Units (SI).
//
//! The SI unit symbol is mol.
//!
//! The quantity amount of substance is a measure of how many elementary
//! entities of a given substance are in an object or sample. Depending on what
//! the substance is, an elementary entity may be an atom, a molecule, an ion,
//! an ion pair, or a subatomic particle such as an electron.

//// Mole

pub struct MoleAsStructI8(pub i8);
pub struct MoleAsStructI16(pub i16);
pub struct MoleAsStructI32(pub i32);
pub struct MoleAsStructI64(pub i64);
pub struct MoleAsStructI128(pub i128);
pub struct MoleAsStructISize(pub isize);
pub struct MoleAsStructU8(pub u8);
pub struct MoleAsStructU16(pub u16);
pub struct MoleAsStructU32(pub u32);
pub struct MoleAsStructU64(pub u64);
pub struct MoleAsStructU128(pub u128);
pub struct MoleAsStructUSize(pub usize);
pub struct MoleAsStructF32(pub f32);
pub struct MoleAsStructF64(pub f64);

pub type MoleAsTypeI8 = i8;
pub type MoleAsTypeI16 = i16;
pub type MoleAsTypeI32 = i32;
pub type MoleAsTypeI64 = i64;
pub type MoleAsTypeI128 = i128;
pub type MoleAsTypeISize = isize;
pub type MoleAsTypeU8 = u8;
pub type MoleAsTypeU16 = u16;
pub type MoleAsTypeU32 = u32;
pub type MoleAsTypeU64 = u64;
pub type MoleAsTypeU128 = u128;
pub type MoleAsTypeUSize = usize;
pub type MoleAsTypeF32 = f32;
pub type MoleAsTypeF64 = f64;
