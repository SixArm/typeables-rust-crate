//! # Day
//!
//! This module provides:
//! 
//!   * Day Name
//! 
//!   * Day Of Week
//! 
//!   * Day Of Month
//! 
//!   * Day Of Year
//! 
//!
//! ## Day Name
//! 
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x: &DayNameAsTypeStr = "Friday";
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x = DayNameAsStructStr("Friday");
//! ```
//! 
//! 
//! ## Day Of Week
//! 
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x: DayOfWeekAsTypeI8 = 0; // Monday
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x = DayOfWeekAsStructI8(0); // Monday
//! ```
//!
//! 
//! ## Day Of Month
//! 
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x: DayOfMonthAsTypeI8 = 31;
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x = DayOfMonthAsStructI8(31);
//! ```
//! 
//! 
//! ## Day Of Year
//!
//! Example type alias nickname:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x: DayOfYearAsTypeI16 = 365;
//! ```
//! 
//! Example struct tuple wrapper:
//! 
//! ```rust
//! # use ::typeables::day::*;
//! let x = DayOfYearAsStructI16(365);
//! ```

//// Day Name

pub type DayNameAsTypeStr = str;
pub type DayNameAsTypeString = String;

pub struct DayNameAsStructStr(pub &'static str);
pub struct DayNameAsStructString(pub String);

//// Day Of Week

pub type DayOfWeekAsTypeI8 = i8;
pub type DayOfWeekAsTypeI16 = i16;
pub type DayOfWeekAsTypeI32 = i32;
pub type DayOfWeekAsTypeI64 = i64;
pub type DayOfWeekAsTypeI128 = i128;
pub type DayOfWeekAsTypeISize = isize;
pub type DayOfWeekAsTypeU8 = u8;
pub type DayOfWeekAsTypeU16 = u16;
pub type DayOfWeekAsTypeU32 = u32;
pub type DayOfWeekAsTypeU64 = u64;
pub type DayOfWeekAsTypeU128 = u128;
pub type DayOfWeekAsTypeUSize = usize;
pub type DayOfWeekAsTypeF32 = f32;
pub type DayOfWeekAsTypeF64 = f64;

pub struct DayOfWeekAsStructI8(pub i8);
pub struct DayOfWeekAsStructI16(pub i16);
pub struct DayOfWeekAsStructI32(pub i32);
pub struct DayOfWeekAsStructI64(pub i64);
pub struct DayOfWeekAsStructI128(pub i128);
pub struct DayOfWeekAsStructISize(pub isize);
pub struct DayOfWeekAsStructU8(pub u8);
pub struct DayOfWeekAsStructU16(pub u16);
pub struct DayOfWeekAsStructU32(pub u32);
pub struct DayOfWeekAsStructU64(pub u64);
pub struct DayOfWeekAsStructU128(pub u128);
pub struct DayOfWeekAsStructUSize(pub usize);
pub struct DayOfWeekAsStructF32(pub f32);
pub struct DayOfWeekAsStructF64(pub f64);

//// Day Of Month

pub type DayOfMonthAsTypeI8 = i8;
pub type DayOfMonthAsTypeI16 = i16;
pub type DayOfMonthAsTypeI32 = i32;
pub type DayOfMonthAsTypeI64 = i64;
pub type DayOfMonthAsTypeI128 = i128;
pub type DayOfMonthAsTypeISize = isize;
pub type DayOfMonthAsTypeU8 = u8;
pub type DayOfMonthAsTypeU16 = u16;
pub type DayOfMonthAsTypeU32 = u32;
pub type DayOfMonthAsTypeU64 = u64;
pub type DayOfMonthAsTypeU128 = u128;
pub type DayOfMonthAsTypeUSize = usize;
pub type DayOfMonthAsTypeF32 = f32;
pub type DayOfMonthAsTypeF64 = f64;

pub struct DayOfMonthAsStructI8(pub i8);
pub struct DayOfMonthAsStructI16(pub i16);
pub struct DayOfMonthAsStructI32(pub i32);
pub struct DayOfMonthAsStructI64(pub i64);
pub struct DayOfMonthAsStructI128(pub i128);
pub struct DayOfMonthAsStructISize(pub isize);
pub struct DayOfMonthAsStructU8(pub u8);
pub struct DayOfMonthAsStructU16(pub u16);
pub struct DayOfMonthAsStructU32(pub u32);
pub struct DayOfMonthAsStructU64(pub u64);
pub struct DayOfMonthAsStructU128(pub u128);
pub struct DayOfMonthAsStructUSize(pub usize);
pub struct DayOfMonthAsStructF32(pub f32);
pub struct DayOfMonthAsStructF64(pub f64);

//// Day Of Year

pub type DayOfYearAsTypeI8 = i8;
pub type DayOfYearAsTypeI16 = i16;
pub type DayOfYearAsTypeI32 = i32;
pub type DayOfYearAsTypeI64 = i64;
pub type DayOfYearAsTypeI128 = i128;
pub type DayOfYearAsTypeISize = isize;
pub type DayOfYearAsTypeU8 = u8;
pub type DayOfYearAsTypeU16 = u16;
pub type DayOfYearAsTypeU32 = u32;
pub type DayOfYearAsTypeU64 = u64;
pub type DayOfYearAsTypeU128 = u128;
pub type DayOfYearAsTypeUSize = usize;
pub type DayOfYearAsTypeF32 = f32;
pub type DayOfYearAsTypeF64 = f64;

pub struct DayOfYearAsStructI8(pub i8);
pub struct DayOfYearAsStructI16(pub i16);
pub struct DayOfYearAsStructI32(pub i32);
pub struct DayOfYearAsStructI64(pub i64);
pub struct DayOfYearAsStructI128(pub i128);
pub struct DayOfYearAsStructISize(pub isize);
pub struct DayOfYearAsStructU8(pub u8);
pub struct DayOfYearAsStructU16(pub u16);
pub struct DayOfYearAsStructU32(pub u32);
pub struct DayOfYearAsStructU64(pub u64);
pub struct DayOfYearAsStructU128(pub u128);
pub struct DayOfYearAsStructUSize(pub usize);
pub struct DayOfYearAsStructF32(pub f32);
pub struct DayOfYearAsStructF64(pub f64);
