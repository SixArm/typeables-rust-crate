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
//!   * Aircraft takeoff altitude is up to 300 meters above ground level.
//!
//!   * Aircraft cruising altitude is up to 13000 meters mean sea level.
//! 
//!   * Airport tower elevation is 90 meters above ground level.
//! 
//!   * Airport runway elevation is 1635 meters mean sea level.
//! 
//! 
//! ## Altitude As Above Ground Level (AGL)
//! 
//! Example type alias:
//! 
//! ```rust
//! # use ::typeables::altitude::*;
//! let takeoff: AltitudeAsAboveGroundLevelAsMeterAsTypeI16 = 300;
//! ```
//! 
//! Example struct tuple:
//! 
//! ```rust
//! # use ::typeables::altitude::*;
//! let takeoff = AltitudeAsAboveGroundLevelAsMeterAsStructI16(300);
//! ```
//! 
//! //! ## Altitude As Mean Sea Level (MSL)
//! 
//! Example type alias:
//! 
//! ```rust
//! # use ::typeables::altitude::*;
//! let cruising: AltitudeAsMeanSeaLevelAsMeterAsTypeI16 = 13000; // Cruising
//! ```
//! 
//! Example struct tuple:
//! 
//! ```rust
//! # use ::typeables::altitude::*;
//! let cruising = AltitudeAsMeanSeaLevelAsMeterAsStructI16(8848);
//! ```
//!
//!
//! ## Altitude v. Elevation
//! 
//! Altitude and elevation are similar concepts: they both measure the height of
//! something relative to a reference datum (such as the earth's surface).
//! 
//! * Altitude typically is for the distance between a moving object (such as an
//!   aircraft) and a well-known reference datum (such as sea level or ground
//!   level). For example, an aircraft takeoff process goes up to 300 meters
//!   above ground level (AGL) i.e. above the airport runway, and a typical
//!   aircraft cruising process goes up to 13000 meters mean sea level (MSL).
//! 
//! * Elevation typically is for the tallness of a stationary object (such as a
//!   place) compared to a well-known reference point (such as sea level or
//!   ground level). For example, the airport in Denver has a runway elevation
//!   of 1635 meters mean sea level (MSL), and an airport control tower building
//!   elevation of 90 meters above ground level (AGL).
//! 
//! 
//! ## Above Ground Level (AGL) v. Mean Sea Level (MSL)
//! 
//! Altitude and elevation can use a variety of representations such as:
//! 
//! * Above Ground Level (AGL). This is measured from the local ground level.
//!   For example, the Burj Khalifa skyscraper in Dubai is 828 meters tall i.e.
//!   the top is 828 meters above ground level.
//! 
//! * Mean Sea Level (MSL). This is measured from a worldwide agreed-upon
//!   standard chosen based on an plausible average of the world's ocean level.

//// Altitude as Above Ground Level (AGL) as Meter

pub type AltitudeAsAboveGroundLevelAsMeterAsTypeI8 = i8;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeI16 = i16;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeI32 = i32;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeI64 = i64;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeI128 = i128;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeISize = isize;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeU8 = u8;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeU16 = u16;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeU32 = u32;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeU64 = u64;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeU128 = u128;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeUSize = usize;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeF32 = f32;
pub type AltitudeAsAboveGroundLevelAsMeterAsTypeF64 = f64;

pub struct AltitudeAsAboveGroundLevelAsMeterAsStructI8(pub i8);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructI16(pub i16);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructI32(pub i32);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructI64(pub i64);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructI128(pub i128);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructISize(pub isize);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructU8(pub u8);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructU16(pub u16);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructU32(pub u32);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructU64(pub u64);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructU128(pub u128);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructUSize(pub usize);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructF32(pub f32);
pub struct AltitudeAsAboveGroundLevelAsMeterAsStructF64(pub f64);

//// Altitude as Mean Sea Level (MSL) as Meter

pub type AltitudeAsMeanSeaLevelAsMeterAsTypeI8 = i8;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeI16 = i16;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeI32 = i32;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeI64 = i64;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeI128 = i128;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeISize = isize;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeU8 = u8;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeU16 = u16;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeU32 = u32;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeU64 = u64;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeU128 = u128;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeUSize = usize;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeF32 = f32;
pub type AltitudeAsMeanSeaLevelAsMeterAsTypeF64 = f64;

pub struct AltitudeAsMeanSeaLevelAsMeterAsStructI8(pub i8);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructI16(pub i16);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructI32(pub i32);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructI64(pub i64);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructI128(pub i128);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructISize(pub isize);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructU8(pub u8);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructU16(pub u16);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructU32(pub u32);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructU64(pub u64);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructU128(pub u128);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructUSize(pub usize);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructF32(pub f32);
pub struct AltitudeAsMeanSeaLevelAsMeterAsStructF64(pub f64);
