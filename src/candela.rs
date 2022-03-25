//! # Candela
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::candela::*;
//! let x = CandelaAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Candela
//! 
//! The candela is a unit of luminous intensity.
//! 
//! It is a base unit in the International System of Units (SI).
//! 
//! The SI unit symbol is cd.
//!
//! A candela is the luminous power per unit solid angle emitted by a point
//! light source in a particular direction. Luminous intensity is analogous to
//! radiant intensity, but instead of simply adding up the contributions of
//! every wavelength of light in the source's spectrum, the contribution of each
//! wavelength is weighted by the standard luminosity function (a model of the
//! sensitivity of the human eye to different wavelengths). 
//! 
//! A common wax candle emits light with a luminous intensity of roughly one
//! candela. If emission in some directions is blocked by an opaque barrier, the
//! emission would still be approximately one candela in the directions that are
//! not obscured.
//!
//! The word candela is Latin for candle. The old name "candle" is still
//! sometimes used, as in foot-candle and the modern definition of candlepower.

//// Candela

pub struct CandelaAsStructI8(pub i8);
pub struct CandelaAsStructI16(pub i16);
pub struct CandelaAsStructI32(pub i32);
pub struct CandelaAsStructI64(pub i64);
pub struct CandelaAsStructI128(pub i128);
pub struct CandelaAsStructISize(pub isize);
pub struct CandelaAsStructU8(pub u8);
pub struct CandelaAsStructU16(pub u16);
pub struct CandelaAsStructU32(pub u32);
pub struct CandelaAsStructU64(pub u64);
pub struct CandelaAsStructU128(pub u128);
pub struct CandelaAsStructUSize(pub usize);
pub struct CandelaAsStructF32(pub f32);
pub struct CandelaAsStructF64(pub f64);

pub type CandelaAsTypeI8 = i8;
pub type CandelaAsTypeI16 = i16;
pub type CandelaAsTypeI32 = i32;
pub type CandelaAsTypeI64 = i64;
pub type CandelaAsTypeI128 = i128;
pub type CandelaAsTypeISize = isize;
pub type CandelaAsTypeU8 = u8;
pub type CandelaAsTypeU16 = u16;
pub type CandelaAsTypeU32 = u32;
pub type CandelaAsTypeU64 = u64;
pub type CandelaAsTypeU128 = u128;
pub type CandelaAsTypeUSize = usize;
pub type CandelaAsTypeF32 = f32;
pub type CandelaAsTypeF64 = f64;
