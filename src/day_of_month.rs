//! # DayOfMonth
//!
//! 
//! ## DayOfMonth As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::day_of_month::*;
//! let x = DayOfMonthAsStructI8(1); // First day of the month
//! ```
//! 
//! 
//! ## DayOfMonth As "DD" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::day_of_month::*;
//! let x = DayOfMonthAsDDAsStructStr("01"); // First day of the month
//! ```
//! 
//! ## DayOfMonth As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::day_of_month::*;
//! let x = DayOfMonthAsNameAsStructStr("First day of the month");
//! ```

//// DayOfMonth

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

//// DayOfMonthAsDD

pub struct DayOfMonthAsDDAsStructStr(pub &'static str);
pub struct DayOfMonthAsDDAsStructString(pub String);

pub type DayOfMonthAsDDAsTypeStr = str;
pub type DayOfMonthAsDDAsTypeString = String;

//// DayOfMonthAsName

pub struct DayOfMonthAsNameAsStructStr(pub &'static str);
pub struct DayOfMonthAsNameAsStructString(pub String);

pub type DayOfMonthAsNameAsTypeStr = str;
pub type DayOfMonthAsNameAsTypeString = String;
