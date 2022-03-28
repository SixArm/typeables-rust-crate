//! # Longitude
//!
//! Examples with New York City Grand Central Terminal:
//! ```
//! # use typeables::latitude::*;
//! # use typeables::longitude::*;
//! let latitude: LongitudeAsDecimalDegreeAsTypeF64 = 40.652687;
//! let longitude: LongitudeAsDecimalDegreeAsTypeF64 = -73.877188;
//! ```
//!
//! ## Representations of longitude and longitude
//!
//! Geolocation of longitude and longitude can use a variety of representations
//! such as:
//!
//!  * Decimal Degree (DD). This is the most common representation.
//!
//!  * Degree and Minute (DM). This has degree as an integer, and minute as an
//!    integer (rarely) or decimal (typically). This representation is sometimes
//!    used for historical data or for legal reasons.
//!
//!  * Degree and Minute and Second (DMS). This has degree as an integer, minute
//!    as an integer, and second is an integer (rarely) or decimal (typically).
//!    This representation is sometimes used for historical data or for legal
//!    reasons.
//!
//! Example as Decimal Degree (DD) representation:
//!
//! ```rust
//! # use typeables::longitude::*;
//! let longitude_dd: LongitudeAsDecimalDegreeAsTypeF64 = 40.652687;
//! ```
//!
//! Example as Degree Minute Second (DMS) representation:
//!
//! ```rust
//! # use typeables::longitude::*;
//! let longitude_dm_degree: LongitudeAsDegreeMinuteWithDegreeAsTypeI16 = 40;
//! let longitude_dm_minute: LongitudeAsDegreeMinuteWithMinuteAsTypeF32 = 39.16122;
//! ```
//!
//! Example as Degree Minute Second (DMS) representation:
//!
//! ```rust
//! # use typeables::longitude::*;
//! let longitude_dms_degree: LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI16 = 40;
//! let longitude_dms_minute: LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI8 = 39;
//! let longitude_dms_second: LongitudeAsDegreeMinuteSecondWithSecondAsTypeF32 = 9.6732;
//! ```

//// Longitude as Decimal Degree (DD)

pub struct LongitudeAsDecimalDegreeAsStructI8(pub i8);
pub struct LongitudeAsDecimalDegreeAsStructI16(pub i16);
pub struct LongitudeAsDecimalDegreeAsStructI32(pub i32);
pub struct LongitudeAsDecimalDegreeAsStructI64(pub i64);
pub struct LongitudeAsDecimalDegreeAsStructI128(pub i128);
pub struct LongitudeAsDecimalDegreeAsStructISize(pub isize);
pub struct LongitudeAsDecimalDegreeAsStructU8(pub u8);
pub struct LongitudeAsDecimalDegreeAsStructU16(pub u16);
pub struct LongitudeAsDecimalDegreeAsStructU32(pub u32);
pub struct LongitudeAsDecimalDegreeAsStructU64(pub u64);
pub struct LongitudeAsDecimalDegreeAsStructU128(pub u128);
pub struct LongitudeAsDecimalDegreeAsStructUSize(pub usize);
pub struct LongitudeAsDecimalDegreeAsStructF32(pub f32);
pub struct LongitudeAsDecimalDegreeAsStructF64(pub f64);

pub type LongitudeAsDecimalDegreeAsTypeI8 = i8;
pub type LongitudeAsDecimalDegreeAsTypeI16 = i16;
pub type LongitudeAsDecimalDegreeAsTypeI32 = i32;
pub type LongitudeAsDecimalDegreeAsTypeI64 = i64;
pub type LongitudeAsDecimalDegreeAsTypeI128 = i128;
pub type LongitudeAsDecimalDegreeAsTypeISize = isize;
pub type LongitudeAsDecimalDegreeAsTypeU8 = u8;
pub type LongitudeAsDecimalDegreeAsTypeU16 = u16;
pub type LongitudeAsDecimalDegreeAsTypeU32 = u32;
pub type LongitudeAsDecimalDegreeAsTypeU64 = u64;
pub type LongitudeAsDecimalDegreeAsTypeU128 = u128;
pub type LongitudeAsDecimalDegreeAsTypeUSize = usize;
pub type LongitudeAsDecimalDegreeAsTypeF32 = f32;
pub type LongitudeAsDecimalDegreeAsTypeF64 = f64;

//// Longitude as Degree Minute (DM) with Degree

pub struct LongitudeAsDegreeMinuteWithDegreeAsStructI8(pub i8);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructI16(pub i16);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructI32(pub i32);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructI64(pub i64);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructI128(pub i128);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructISize(pub isize);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructU8(pub u8);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructU16(pub u16);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructU32(pub u32);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructU64(pub u64);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructU128(pub u128);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructUSize(pub usize);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructF32(pub f32);
pub struct LongitudeAsDegreeMinuteWithDegreeAsStructF64(pub f64);

pub type LongitudeAsDegreeMinuteWithDegreeAsTypeI8 = i8;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeI16 = i16;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeI32 = i32;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeI64 = i64;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeI128 = i128;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeISize = isize;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeU8 = u8;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeU16 = u16;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeU32 = u32;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeU64 = u64;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeU128 = u128;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeUSize = usize;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeF32 = f32;
pub type LongitudeAsDegreeMinuteWithDegreeAsTypeF64 = f64;

//// Longitude as Degree Minute (DM) with Minute

pub struct LongitudeAsDegreeMinuteWithMinuteAsStructI8(pub i8);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructI16(pub i16);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructI32(pub i32);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructI64(pub i64);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructI128(pub i128);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructISize(pub isize);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructU8(pub u8);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructU16(pub u16);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructU32(pub u32);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructU64(pub u64);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructU128(pub u128);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructUSize(pub usize);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructF32(pub f32);
pub struct LongitudeAsDegreeMinuteWithMinuteAsStructF64(pub f64);

pub type LongitudeAsDegreeMinuteWithMinuteAsTypeI8 = i8;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeI16 = i16;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeI32 = i32;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeI64 = i64;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeI128 = i128;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeISize = isize;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeU8 = u8;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeU16 = u16;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeU32 = u32;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeU64 = u64;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeU128 = u128;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeUSize = usize;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeF32 = f32;
pub type LongitudeAsDegreeMinuteWithMinuteAsTypeF64 = f64;

//// Longitude as Degree Minute Second (DMS) with Degree

pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructI8(pub i8);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructI16(pub i16);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructI32(pub i32);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructI64(pub i64);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructI128(pub i128);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructISize(pub isize);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructU8(pub u8);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructU16(pub u16);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructU32(pub u32);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructU64(pub u64);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructU128(pub u128);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructUSize(pub usize);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructF32(pub f32);
pub struct LongitudeAsDegreeMinuteSecondWithDegreeAsStructF64(pub f64);

pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI8 = i8;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI16 = i16;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI32 = i32;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI64 = i64;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeI128 = i128;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeISize = isize;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeU8 = u8;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeU16 = u16;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeU32 = u32;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeU64 = u64;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeU128 = u128;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeUSize = usize;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeF32 = f32;
pub type LongitudeAsDegreeMinuteSecondWithDegreeAsTypeF64 = f64;

//// Longitude as Degree Minute Second (DMS) with Minute

pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructI8(pub i8);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructI16(pub i16);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructI32(pub i32);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructI64(pub i64);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructI128(pub i128);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructISize(pub isize);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructU8(pub u8);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructU16(pub u16);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructU32(pub u32);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructU64(pub u64);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructU128(pub u128);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructUSize(pub usize);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructF32(pub f32);
pub struct LongitudeAsDegreeMinuteSecondWithMinuteAsStructF64(pub f64);

pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI8 = i8;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI16 = i16;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI32 = i32;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI64 = i64;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeI128 = i128;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeISize = isize;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeU8 = u8;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeU16 = u16;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeU32 = u32;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeU64 = u64;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeU128 = u128;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeUSize = usize;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeF32 = f32;
pub type LongitudeAsDegreeMinuteSecondWithMinuteAsTypeF64 = f64;

//// Longitude as Degree Minute Second (DMS) with Second

pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructI8(pub i8);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructI16(pub i16);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructI32(pub i32);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructI64(pub i64);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructI128(pub i128);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructISize(pub isize);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructU8(pub u8);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructU16(pub u16);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructU32(pub u32);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructU64(pub u64);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructU128(pub u128);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructUSize(pub usize);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructF32(pub f32);
pub struct LongitudeAsDegreeMinuteSecondWithSecondAsStructF64(pub f64);

pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeI8 = i8;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeI16 = i16;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeI32 = i32;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeI64 = i64;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeI128 = i128;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeISize = isize;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeU8 = u8;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeU16 = u16;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeU32 = u32;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeU64 = u64;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeU128 = u128;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeUSize = usize;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeF32 = f32;
pub type LongitudeAsDegreeMinuteSecondWithSecondAsTypeF64 = f64;
