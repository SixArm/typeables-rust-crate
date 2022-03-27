//! # Unit Interval
//!
//! A number between 0 inclusive and 1 inclusive.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::unit_interval::*;
//! let x = UnitIntervalAsStructF64(0.5);
//! ```
//!
//! Compare:
//!
//!   * Unit Interval is 0..1
//!
//!   * Dual Interval is -1..1

pub struct UnitIntervalAsStructF32(pub f32);
pub struct UnitIntervalAsStructF64(pub f64);

pub type UnitIntervalAsTypeF32 = f32;
pub type UnitIntervalAsTypeF64 = f64;
