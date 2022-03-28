//! # WeekOfMonth
//!
//! 
//! ## WeekOfMonth As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::week_of_month::*;
//! let x = WeekOfMonthAsStructI8(1); // First week of the month
//! ```
//! 
//! 
//! ## WeekOfMonth As "WW" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::week_of_month::*;
//! let x = WeekOfMonthAsWWAsStructStr("01"); // First week of the month
//! ```
//! 
//! ## WeekOfMonth As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::week_of_month::*;
//! let x = WeekOfMonthAsNameAsStructStr("First week of the month");
//! ```

//// WeekOfMonth

pub struct WeekOfMonthAsStructI8(pub i8);
pub struct WeekOfMonthAsStructI16(pub i16);
pub struct WeekOfMonthAsStructI32(pub i32);
pub struct WeekOfMonthAsStructI64(pub i64);
pub struct WeekOfMonthAsStructI128(pub i128);
pub struct WeekOfMonthAsStructISize(pub isize);
pub struct WeekOfMonthAsStructU8(pub u8);
pub struct WeekOfMonthAsStructU16(pub u16);
pub struct WeekOfMonthAsStructU32(pub u32);
pub struct WeekOfMonthAsStructU64(pub u64);
pub struct WeekOfMonthAsStructU128(pub u128);
pub struct WeekOfMonthAsStructUSize(pub usize);
pub struct WeekOfMonthAsStructF32(pub f32);
pub struct WeekOfMonthAsStructF64(pub f64);

pub type WeekOfMonthAsTypeI8 = i8;
pub type WeekOfMonthAsTypeI16 = i16;
pub type WeekOfMonthAsTypeI32 = i32;
pub type WeekOfMonthAsTypeI64 = i64;
pub type WeekOfMonthAsTypeI128 = i128;
pub type WeekOfMonthAsTypeISize = isize;
pub type WeekOfMonthAsTypeU8 = u8;
pub type WeekOfMonthAsTypeU16 = u16;
pub type WeekOfMonthAsTypeU32 = u32;
pub type WeekOfMonthAsTypeU64 = u64;
pub type WeekOfMonthAsTypeU128 = u128;
pub type WeekOfMonthAsTypeUSize = usize;
pub type WeekOfMonthAsTypeF32 = f32;
pub type WeekOfMonthAsTypeF64 = f64;

//// WeekOfMonthAsWW

pub struct WeekOfMonthAsWWAsStructStr(pub &'static str);
pub struct WeekOfMonthAsWWAsStructString(pub String);

pub type WeekOfMonthAsWWAsTypeStr = str;
pub type WeekOfMonthAsWWAsTypeString = String;

//// WeekOfMonthAsName

pub struct WeekOfMonthAsNameAsStructStr(pub &'static str);
pub struct WeekOfMonthAsNameAsStructString(pub String);

pub type WeekOfMonthAsNameAsTypeStr = str;
pub type WeekOfMonthAsNameAsTypeString = String;
