//! # Latitude
//!
//! Examples with New York City Grand Central Terminal:
//! ```
//! # use typeables::latitude::*;
//! # use typeables::longitude::*;
//! let latitude: LatitudeAsDecimalDegreeAsTypeF64 = 40.652687;
//! let longitude: LongitudeAsDecimalDegreeAsTypeF64 = -73.877188;
//! ```
//!
//! ## Representations of latitude and longitude
//!
//! Geolocation of latitude and longitude can use a variety of representations
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
//! # use typeables::latitude::*;
//! let latitude_dd: LatitudeAsDecimalDegreeAsTypeF64 = 40.652687;
//! ```
//!
//! Example as Degree Minute Second (DMS) representation:
//!
//! ```rust
//! # use typeables::latitude::*;
//! let latitude_dm_degree: LatitudeAsDegreeMinuteWithDegreeAsTypeI16 = 40;
//! let latitude_dm_minute: LatitudeAsDegreeMinuteWithMinuteAsTypeF32 = 39.16122;
//! ```
//!
//! Example as Degree Minute Second (DMS) representation:
//!
//! ```rust
//! # use typeables::latitude::*;
//! let latitude_dms_degree: LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI16 = 40;
//! let latitude_dms_minute: LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI8 = 39;
//! let latitude_dms_second: LatitudeAsDegreeMinuteSecondWithSecondAsTypeF32 = 9.6732;
//! ```

//// Latitude as Decimal Degree (DD)

pub struct LatitudeAsDecimalDegreeAsStructI8(pub i8);
pub struct LatitudeAsDecimalDegreeAsStructI16(pub i16);
pub struct LatitudeAsDecimalDegreeAsStructI32(pub i32);
pub struct LatitudeAsDecimalDegreeAsStructI64(pub i64);
pub struct LatitudeAsDecimalDegreeAsStructI128(pub i128);
pub struct LatitudeAsDecimalDegreeAsStructISize(pub isize);
pub struct LatitudeAsDecimalDegreeAsStructU8(pub u8);
pub struct LatitudeAsDecimalDegreeAsStructU16(pub u16);
pub struct LatitudeAsDecimalDegreeAsStructU32(pub u32);
pub struct LatitudeAsDecimalDegreeAsStructU64(pub u64);
pub struct LatitudeAsDecimalDegreeAsStructU128(pub u128);
pub struct LatitudeAsDecimalDegreeAsStructUSize(pub usize);
pub struct LatitudeAsDecimalDegreeAsStructF32(pub f32);
pub struct LatitudeAsDecimalDegreeAsStructF64(pub f64);

pub type LatitudeAsDecimalDegreeAsTypeI8 = i8;
pub type LatitudeAsDecimalDegreeAsTypeI16 = i16;
pub type LatitudeAsDecimalDegreeAsTypeI32 = i32;
pub type LatitudeAsDecimalDegreeAsTypeI64 = i64;
pub type LatitudeAsDecimalDegreeAsTypeI128 = i128;
pub type LatitudeAsDecimalDegreeAsTypeISize = isize;
pub type LatitudeAsDecimalDegreeAsTypeU8 = u8;
pub type LatitudeAsDecimalDegreeAsTypeU16 = u16;
pub type LatitudeAsDecimalDegreeAsTypeU32 = u32;
pub type LatitudeAsDecimalDegreeAsTypeU64 = u64;
pub type LatitudeAsDecimalDegreeAsTypeU128 = u128;
pub type LatitudeAsDecimalDegreeAsTypeUSize = usize;
pub type LatitudeAsDecimalDegreeAsTypeF32 = f32;
pub type LatitudeAsDecimalDegreeAsTypeF64 = f64;

//// Latitude as Degree Minute (DM) with Degree

pub struct LatitudeAsDegreeMinuteWithDegreeAsStructI8(pub i8);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructI16(pub i16);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructI32(pub i32);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructI64(pub i64);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructI128(pub i128);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructISize(pub isize);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructU8(pub u8);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructU16(pub u16);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructU32(pub u32);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructU64(pub u64);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructU128(pub u128);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructUSize(pub usize);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructF32(pub f32);
pub struct LatitudeAsDegreeMinuteWithDegreeAsStructF64(pub f64);

pub type LatitudeAsDegreeMinuteWithDegreeAsTypeI8 = i8;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeI16 = i16;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeI32 = i32;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeI64 = i64;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeI128 = i128;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeISize = isize;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeU8 = u8;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeU16 = u16;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeU32 = u32;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeU64 = u64;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeU128 = u128;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeUSize = usize;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeF32 = f32;
pub type LatitudeAsDegreeMinuteWithDegreeAsTypeF64 = f64;

//// Latitude as Degree Minute (DM) with Minute

pub struct LatitudeAsDegreeMinuteWithMinuteAsStructI8(pub i8);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructI16(pub i16);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructI32(pub i32);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructI64(pub i64);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructI128(pub i128);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructISize(pub isize);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructU8(pub u8);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructU16(pub u16);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructU32(pub u32);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructU64(pub u64);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructU128(pub u128);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructUSize(pub usize);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructF32(pub f32);
pub struct LatitudeAsDegreeMinuteWithMinuteAsStructF64(pub f64);

pub type LatitudeAsDegreeMinuteWithMinuteAsTypeI8 = i8;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeI16 = i16;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeI32 = i32;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeI64 = i64;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeI128 = i128;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeISize = isize;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeU8 = u8;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeU16 = u16;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeU32 = u32;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeU64 = u64;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeU128 = u128;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeUSize = usize;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeF32 = f32;
pub type LatitudeAsDegreeMinuteWithMinuteAsTypeF64 = f64;

//// Latitude as Degree Minute Second (DMS) with Degree

pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructI8(pub i8);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructI16(pub i16);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructI32(pub i32);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructI64(pub i64);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructI128(pub i128);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructISize(pub isize);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructU8(pub u8);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructU16(pub u16);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructU32(pub u32);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructU64(pub u64);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructU128(pub u128);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructUSize(pub usize);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructF32(pub f32);
pub struct LatitudeAsDegreeMinuteSecondWithDegreeAsStructF64(pub f64);

pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI8 = i8;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI16 = i16;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI32 = i32;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI64 = i64;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeI128 = i128;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeISize = isize;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeU8 = u8;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeU16 = u16;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeU32 = u32;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeU64 = u64;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeU128 = u128;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeUSize = usize;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeF32 = f32;
pub type LatitudeAsDegreeMinuteSecondWithDegreeAsTypeF64 = f64;

//// Latitude as Degree Minute Second (DMS) with Minute

pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructI8(pub i8);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructI16(pub i16);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructI32(pub i32);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructI64(pub i64);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructI128(pub i128);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructISize(pub isize);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructU8(pub u8);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructU16(pub u16);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructU32(pub u32);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructU64(pub u64);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructU128(pub u128);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructUSize(pub usize);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructF32(pub f32);
pub struct LatitudeAsDegreeMinuteSecondWithMinuteAsStructF64(pub f64);

pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI8 = i8;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI16 = i16;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI32 = i32;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI64 = i64;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeI128 = i128;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeISize = isize;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeU8 = u8;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeU16 = u16;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeU32 = u32;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeU64 = u64;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeU128 = u128;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeUSize = usize;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeF32 = f32;
pub type LatitudeAsDegreeMinuteSecondWithMinuteAsTypeF64 = f64;

//// Latitude as Degree Minute Second (DMS) with Second

pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructI8(pub i8);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructI16(pub i16);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructI32(pub i32);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructI64(pub i64);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructI128(pub i128);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructISize(pub isize);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructU8(pub u8);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructU16(pub u16);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructU32(pub u32);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructU64(pub u64);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructU128(pub u128);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructUSize(pub usize);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructF32(pub f32);
pub struct LatitudeAsDegreeMinuteSecondWithSecondAsStructF64(pub f64);

pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeI8 = i8;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeI16 = i16;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeI32 = i32;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeI64 = i64;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeI128 = i128;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeISize = isize;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeU8 = u8;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeU16 = u16;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeU32 = u32;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeU64 = u64;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeU128 = u128;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeUSize = usize;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeF32 = f32;
pub type LatitudeAsDegreeMinuteSecondWithSecondAsTypeF64 = f64;
