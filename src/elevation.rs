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
//!   * Airport tower elevation is 90 meters above ground level.
//! 
//!   * Airport runway elevation is 1635 meters mean sea level.
//! 
//!   * Aircraft takeoff altitude is up to 300 meters above ground level.
//!
//!   * Aircraft cruising altitude is up to 13000 meters mean sea level.
//! 
//! 
//! ## Elevation As Above Ground Level (AGL)
//! 
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::elevation::*;
//! let tower: ElevationAsAboveGroundLevelAsMeterAsTypeI16 = 90;
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::elevation::*;
//! let tower = ElevationAsAboveGroundLevelAsMeterAsStructI16(300);
//! ```
//! 
//! 
//! ## Elevation As Mean Sea Level (MSL)
//! 
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::elevation::*;
//! let runway: ElevationAsMeanSeaLevelAsMeterAsTypeI16 = 1635;
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::elevation::*;
//! let runway = ElevationAsMeanSeaLevelAsMeterAsStructI16(8848);
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

//// Elevation as Above Ground Level (AGL) as Meter

pub type ElevationAsAboveGroundLevelAsMeterAsTypeI8 = i8;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeI16 = i16;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeI32 = i32;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeI64 = i64;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeI128 = i128;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeISize = isize;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeU8 = u8;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeU16 = u16;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeU32 = u32;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeU64 = u64;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeU128 = u128;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeUSize = usize;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeF32 = f32;
pub type ElevationAsAboveGroundLevelAsMeterAsTypeF64 = f64;

pub struct ElevationAsAboveGroundLevelAsMeterAsStructI8(pub i8);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructI16(pub i16);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructI32(pub i32);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructI64(pub i64);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructI128(pub i128);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructISize(pub isize);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructU8(pub u8);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructU16(pub u16);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructU32(pub u32);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructU64(pub u64);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructU128(pub u128);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructUSize(pub usize);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructF32(pub f32);
pub struct ElevationAsAboveGroundLevelAsMeterAsStructF64(pub f64);

//// Elevation as Mean Sea Level (MSL) as Meter

pub type ElevationAsMeanSeaLevelAsMeterAsTypeI8 = i8;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeI16 = i16;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeI32 = i32;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeI64 = i64;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeI128 = i128;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeISize = isize;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeU8 = u8;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeU16 = u16;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeU32 = u32;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeU64 = u64;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeU128 = u128;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeUSize = usize;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeF32 = f32;
pub type ElevationAsMeanSeaLevelAsMeterAsTypeF64 = f64;

pub struct ElevationAsMeanSeaLevelAsMeterAsStructI8(pub i8);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructI16(pub i16);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructI32(pub i32);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructI64(pub i64);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructI128(pub i128);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructISize(pub isize);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructU8(pub u8);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructU16(pub u16);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructU32(pub u32);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructU64(pub u64);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructU128(pub u128);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructUSize(pub usize);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructF32(pub f32);
pub struct ElevationAsMeanSeaLevelAsMeterAsStructF64(pub f64);
