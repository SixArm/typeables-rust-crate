//! # Joule
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::joule::*;
//! let x = JouleAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Joule
//! 
//! The joule is a unit of energy.
//! 
//! It is a derived unit in the International System of Units (SI).
//! 
//! The SI unit symbol is J.
//!
//! It is equal to the amount of work done when a force of 1 Newton displaces a
//! body through a distance of 1 metre in the direction of the force applied. It
//! is also the energy dissipated as heat when an electric current of one ampere
//! passes through a resistance of one ohm for one second. It is named after the
//! English physicist James Prescott Joule (1818–1889).

//// Joule

pub struct JouleAsStructI8(pub i8);
pub struct JouleAsStructI16(pub i16);
pub struct JouleAsStructI32(pub i32);
pub struct JouleAsStructI64(pub i64);
pub struct JouleAsStructI128(pub i128);
pub struct JouleAsStructISize(pub isize);
pub struct JouleAsStructU8(pub u8);
pub struct JouleAsStructU16(pub u16);
pub struct JouleAsStructU32(pub u32);
pub struct JouleAsStructU64(pub u64);
pub struct JouleAsStructU128(pub u128);
pub struct JouleAsStructUSize(pub usize);
pub struct JouleAsStructF32(pub f32);
pub struct JouleAsStructF64(pub f64);

pub type JouleAsTypeI8 = i8;
pub type JouleAsTypeI16 = i16;
pub type JouleAsTypeI32 = i32;
pub type JouleAsTypeI64 = i64;
pub type JouleAsTypeI128 = i128;
pub type JouleAsTypeISize = isize;
pub type JouleAsTypeU8 = u8;
pub type JouleAsTypeU16 = u16;
pub type JouleAsTypeU32 = u32;
pub type JouleAsTypeU64 = u64;
pub type JouleAsTypeU128 = u128;
pub type JouleAsTypeUSize = usize;
pub type JouleAsTypeF32 = f32;
pub type JouleAsTypeF64 = f64;
