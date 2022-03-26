//! # Elevation
//!
//! This module provides:
//!
//!   * Elevation As Above Ground Level (AGL)
//!
//!   * Elevation As Mean Sea Level (MSL)
//!
//! See below for:
//!
//!   * Altitude v. Elevation.
//!
//!   * Above Ground Level (AGL) v. Mean Sea Level (MSL)
//!
//! Examples here are about an airport and aircraft:
//!
//!   * Airport tower elevation is 90 metres above ground level.
//!
//!   * Airport runway elevation is 1635 metres mean sea level.
//!
//!   * Aircraft takeoff altitude is up to 300 metres above ground level.
//!
//!   * Aircraft cruising altitude is up to 13000 metres mean sea level.
//!
//!
//! ## Elevation As Above Ground Level (AGL)
//!
//! Example:
//!
//! ```rust
//! # use ::typeables::elevation::*;
//! let tower = ElevationAsAboveGroundLevelAsMetreAsStructI32(300);
//! ```
//!
//!
//! ## Elevation As Mean Sea Level (MSL)
//!
//! Example:
//!
//! ```rust
//! # use ::typeables::elevation::*;
//! let runway = ElevationAsMeanSeaLevelAsMetreAsStructI32(8848);
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

//// Elevation as Above Ground Level (AGL) as Metre

pub struct ElevationAsAboveGroundLevelAsMetreAsStructI8(pub i8);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructI16(pub i16);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructI32(pub i32);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructI64(pub i64);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructI128(pub i128);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructISize(pub isize);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructU8(pub u8);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructU16(pub u16);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructU32(pub u32);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructU64(pub u64);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructU128(pub u128);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructUSize(pub usize);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructF32(pub f32);
pub struct ElevationAsAboveGroundLevelAsMetreAsStructF64(pub f64);

pub type ElevationAsAboveGroundLevelAsMetreAsTypeI8 = i8;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeI16 = i16;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeI32 = i32;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeI64 = i64;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeI128 = i128;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeISize = isize;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeU8 = u8;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeU16 = u16;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeU32 = u32;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeU64 = u64;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeU128 = u128;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeUSize = usize;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeF32 = f32;
pub type ElevationAsAboveGroundLevelAsMetreAsTypeF64 = f64;

//// Elevation as Mean Sea Level (MSL) as Metre

pub struct ElevationAsMeanSeaLevelAsMetreAsStructI8(pub i8);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructI16(pub i16);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructI32(pub i32);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructI64(pub i64);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructI128(pub i128);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructISize(pub isize);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructU8(pub u8);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructU16(pub u16);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructU32(pub u32);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructU64(pub u64);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructU128(pub u128);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructUSize(pub usize);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructF32(pub f32);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructF64(pub f64);

pub type ElevationAsMeanSeaLevelAsMetreAsTypeI8 = i8;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeI16 = i16;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeI32 = i32;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeI64 = i64;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeI128 = i128;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeISize = isize;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeU8 = u8;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeU16 = u16;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeU32 = u32;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeU64 = u64;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeU128 = u128;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeUSize = usize;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeF32 = f32;
pub type ElevationAsMeanSeaLevelAsMetreAsTypeF64 = f64;
