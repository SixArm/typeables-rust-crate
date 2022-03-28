//! # DayOfYear
//!
//! 
//! ## DayOfYear As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::day_of_year::*;
//! let x = DayOfYearAsStructI16(1); // First day of the year
//! ```
//! 
//! 
//! ## DayOfYear As "DD" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::day_of_year::*;
//! let x = DayOfYearAsDDAsStructStr("01"); // First day of the year
//! ```
//! 
//! 
//! ## DayOfYear As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::day_of_year::*;
//! let x = DayOfYearAsNameAsStructStr("New Year's Day");
//! ```

//// DayOfYear
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

//// DayOfYearAsDD

pub struct DayOfYearAsDDAsStructStr(pub &'static str);
pub struct DayOfYearAsDDAsStructString(pub String);

pub type DayOfYearAsDDAsTypeStr = str;
pub type DayOfYearAsDDAsTypeString = String;

//// DayOfYearAsName

pub struct DayOfYearAsNameAsStructStr(pub &'static str);
pub struct DayOfYearAsNameAsStructString(pub String);

pub type DayOfYearAsNameAsTypeStr = str;
pub type DayOfYearAsNameAsTypeString = String;
