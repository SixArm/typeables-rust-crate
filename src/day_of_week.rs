//! # DayOfWeek
//!
//! 
//! ## DayOfWeek As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::day_of_week::*;
//! let x = DayOfWeekAsStructI8(1); // First day of the week i.e. Monday
//! ```
//! 
//! 
//! ## DayOfWeek As "DD" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::day_of_week::*;
//! let x = DayOfWeekAsDDAsStructStr("01"); // First day of the week i.e. Monday
//! ```
//! 
//! 
//! ## DayOfWeek As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::day_of_week::*;
//! let x = DayOfWeekAsNameAsStructStr("Monday");
//! ```

//// DayOfWeek

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

//// DayOfWeekAsDD

pub struct DayOfWeekAsDDAsStructStr(pub &'static str);
pub struct DayOfWeekAsDDAsStructString(pub String);

pub type DayOfWeekAsDDAsTypeStr = str;
pub type DayOfWeekAsDDAsTypeString = String;

//// DayOfWeekAsName

pub struct DayOfWeekAsNameAsStructStr(pub &'static str);
pub struct DayOfWeekAsNameAsStructString(pub String);

pub type DayOfWeekAsNameAsTypeStr = str;
pub type DayOfWeekAsNameAsTypeString = String;
