//! # Typeables: Rust crate of type aliases
//!
//! Typeables is a Rust crate of type aliases, intended to help improve source code clarity. This helps with literate programming, domain driven design, and developer knowledge.
//!
//!
//! # Examples
//!
//! Geolocation example of New York City Grand Central Terminal:
//!
//! ```rust
//! # use typeables::geolocation::*;
//! let lat: LatitudeDecimalDegreeF64 = 40.75; // Western hemisphere
//! let lng: LongitudeDecimalDegreeF64 = -73.97; // Northern hemisphere
//! let alt: AltitudeMeanSeaLevelMeterF64 = 54.00; // 54 meter elevation
//! ```
//!
//! Date-time example of NASA launching Mars Perseverance Rover:
//!
//! ```rust
//! # use typeables::datetime::*;
//! let date: &DateYYYYXMMXDDStr = "2020-07-30"; // July 30, 2020
//! let time: &TimeHHXMMXSSStr = "07:50:00"; // 7:50 a.m.
//! let zone: &ZoneHHXMMStr = "-05:00"; // Eastern Daylight Time (EDT)
//! ```
//!
//!
//! # Purpose
//!
//! The purpose of this crate is syntax sugar for better readabiliy.
//!
//! The purpose of this library is not any kind type-based coding, such as data encapsulation, or parameter validation, or object oriented programming. If you want these kinds of aspects, we recommend looking at the crate `uom` (unit of measure) and the Rust book examples of the `newtype` pattern.
//!
//!
//! # Implementation
//!
//! The type aliases are all for Rust primitives and standards such as strings
//! (using `str` and `String`) and numbers (using `i64`, `u64`, `f64`, et al.).
//!
//! The type aliases are zero-overhead because they are replaced at compile time.

pub mod datetime;
pub mod geolocation;
pub mod grammar;
pub mod locale;
pub mod media_type;
