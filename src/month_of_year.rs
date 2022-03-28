//! # MonthOfYear
//!
//! 
//! ## MonthOfYear As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::month_of_year::*;
//! let x = MonthOfYearAsStructI8(1); // First month of the year means January
//! ```
//! 
//! 
//! ## MonthOfYear As "MM" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::month_of_year::*;
//! let x = MonthOfYearAsMMAsStructStr("01");
//! ```
//! 
//! ## MonthOfYear As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::month_of_year::*;
//! let x = MonthOfYearAsNameAsStructStr("January");
//! ```

//// MonthOfYear

pub struct MonthOfYearAsStructI8(pub i8);
pub struct MonthOfYearAsStructI16(pub i16);
pub struct MonthOfYearAsStructI32(pub i32);
pub struct MonthOfYearAsStructI64(pub i64);
pub struct MonthOfYearAsStructI128(pub i128);
pub struct MonthOfYearAsStructISize(pub isize);
pub struct MonthOfYearAsStructU8(pub u8);
pub struct MonthOfYearAsStructU16(pub u16);
pub struct MonthOfYearAsStructU32(pub u32);
pub struct MonthOfYearAsStructU64(pub u64);
pub struct MonthOfYearAsStructU128(pub u128);
pub struct MonthOfYearAsStructUSize(pub usize);
pub struct MonthOfYearAsStructF32(pub f32);
pub struct MonthOfYearAsStructF64(pub f64);

pub type MonthOfYearAsTypeI8 = i8;
pub type MonthOfYearAsTypeI16 = i16;
pub type MonthOfYearAsTypeI32 = i32;
pub type MonthOfYearAsTypeI64 = i64;
pub type MonthOfYearAsTypeI128 = i128;
pub type MonthOfYearAsTypeISize = isize;
pub type MonthOfYearAsTypeU8 = u8;
pub type MonthOfYearAsTypeU16 = u16;
pub type MonthOfYearAsTypeU32 = u32;
pub type MonthOfYearAsTypeU64 = u64;
pub type MonthOfYearAsTypeU128 = u128;
pub type MonthOfYearAsTypeUSize = usize;
pub type MonthOfYearAsTypeF32 = f32;
pub type MonthOfYearAsTypeF64 = f64;

//// MonthOfYearAsMM

pub struct MonthOfYearAsMMAsStructStr(pub &'static str);
pub struct MonthOfYearAsMMAsStructString(pub String);

pub type MonthOfYearAsMMAsTypeStr = str;
pub type MonthOfYearAsMMAsTypeString = String;

//// MonthOfYearAsName

pub struct MonthOfYearAsNameAsStructStr(pub &'static str);
pub struct MonthOfYearAsNameAsStructString(pub String);

pub type MonthOfYearAsNameAsTypeStr = str;
pub type MonthOfYearAsNameAsTypeString = String;
