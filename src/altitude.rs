//! # Altitude
//!
//! This module provides:
//!
//!   * Altitude As Above Ground Level (AGL)
//!
//!   * Altitude As Mean Sea Level (MSL)
//!
//! See below for:
//!
//!   * Altitude v. Elevation.
//!
//!   * Above Ground Level (AGL) v. Mean Sea Level (MSL)
//!
//! Examples here are about an aircraft and airport:
//!
//!   * Aircraft takeoff altitude is up to 300 metres above ground level.
//!
//!   * Aircraft cruising altitude is up to 13000 metres mean sea level.
//!
//!   * Airport tower elevation is 90 metres above ground level.
//!
//!   * Airport runway elevation is 1635 metres mean sea level.
//!
//!
//! ## Altitude As Above Ground Level (AGL)
//!
//! Example:
//!
//! ```rust
//! # use ::typeables::altitude::*;
//! let takeoff = AltitudeAsAboveGroundLevelAsMetreAsStructI32(300);
//! ```
//!
//!
//! ## Altitude As Mean Sea Level (MSL)
//!
//! Example:
//!
//! ```rust
//! # use ::typeables::altitude::*;
//! let cruising = AltitudeAsMeanSeaLevelAsMetreAsStructI32(13000);
//! ```
//!
//!
//! ## Altitude v. Elevation
//!
//! Altitude and elevation are similar concepts: they both measure the height of
//! something relative to a reference datum (such as the earth's surface).
//!
//! * Altitude typically is for the distance between a moving object (such as an
//!   aircraft) and a well-known reference datum (such as ground level or sea
//!   level). For example, an aircraft takeoff process goes up to an alititude
//!   of 300 metres above ground level (AGL) i.e. above the airport runway, and
//!   a typical aircraft cruising process goes up to an alititude of 13000
//!   metres mean sea level (MSL).
//!
//! * Elevation typically is for the tallness of a stationary object (such as a
//!   place) compared to a well-known reference point (such as ground level or
//!   seal level).  For example, an airport control tower building has an
//!   elevation of 80 metres above ground level (AGL), and an airport runway in
//!   Denver Colorado has an elevation of 1635 metres mean sea level (MSL).
//!
//!
//! ## Above Ground Level (AGL) v. Mean Sea Level (MSL)
//!
//! Altitude and elevation can use a variety of representations such as:
//!
//! * Above Ground Level (AGL). This is measured from the local ground level.
//!   For example, the Burj Khalifa skyscraper in Dubai is 828 metres tall i.e.
//!   the top is 828 metres above ground level.
//!
//! * Mean Sea Level (MSL). This is measured from a worldwide agreed-upon
//!   standard chosen based on an plausible average of the world's ocean level.

//// Altitude as Above Ground Level (AGL) as Metre

pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI8(pub i8);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI16(pub i16);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI32(pub i32);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI64(pub i64);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI128(pub i128);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructISize(pub isize);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructU8(pub u8);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructU16(pub u16);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructU32(pub u32);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructU64(pub u64);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructU128(pub u128);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructUSize(pub usize);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructF32(pub f32);
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructF64(pub f64);

pub type AltitudeAsAboveGroundLevelAsMetreAsTypeI8 = i8;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeI16 = i16;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeI32 = i32;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeI64 = i64;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeI128 = i128;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeISize = isize;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeU8 = u8;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeU16 = u16;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeU32 = u32;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeU64 = u64;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeU128 = u128;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeUSize = usize;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeF32 = f32;
pub type AltitudeAsAboveGroundLevelAsMetreAsTypeF64 = f64;

//// Altitude as Mean Sea Level (MSL) as Metre
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI8(pub i8);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI16(pub i16);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI32(pub i32);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI64(pub i64);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI128(pub i128);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructISize(pub isize);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructU8(pub u8);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructU16(pub u16);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructU32(pub u32);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructU64(pub u64);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructU128(pub u128);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructUSize(pub usize);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructF32(pub f32);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructF64(pub f64);

pub type AltitudeAsMeanSeaLevelAsMetreAsTypeI8 = i8;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeI16 = i16;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeI32 = i32;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeI64 = i64;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeI128 = i128;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeISize = isize;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeU8 = u8;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeU16 = u16;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeU32 = u32;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeU64 = u64;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeU128 = u128;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeUSize = usize;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeF32 = f32;
pub type AltitudeAsMeanSeaLevelAsMetreAsTypeF64 = f64;
