//! # Dual Interval
//!
//! A number between -1 inclusive and 1 inclusive.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::dual_interval::*;
//! let x = DualIntervalAsStructF64(0.5);
//! ```
//!
//! Compare:
//!
//!   * Dual Interval is 0..1
//!
//!   * Dual Interval is -1..1

pub struct DualIntervalAsStructF32(pub f32);
pub struct DualIntervalAsStructF64(pub f64);

pub type DualIntervalAsTypeF32 = f32;
pub type DualIntervalAsTypeF64 = f64;
