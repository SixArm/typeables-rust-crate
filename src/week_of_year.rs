//! # WeekOfYear
//!
//! 
//! ## WeekOfYear As Number
//! 
//! Example:
//!
//! ```rust
//! # use typeables::week_of_year::*;
//! let x = WeekOfYearAsStructI8(1); // First week of the year
//! ```
//! 
//! 
//! ## WeekOfYear As "WW" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::week_of_year::*;
//! let x = WeekOfYearAsWWAsStructStr("01");
//! ```
//! 
//! ## WeekOfYear As Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::week_of_year::*;
//! let x = WeekOfYearAsNameAsStructStr("First week of the year");
//! ```

//// WeekOfYear

pub struct WeekOfYearAsStructI8(pub i8);
pub struct WeekOfYearAsStructI16(pub i16);
pub struct WeekOfYearAsStructI32(pub i32);
pub struct WeekOfYearAsStructI64(pub i64);
pub struct WeekOfYearAsStructI128(pub i128);
pub struct WeekOfYearAsStructISize(pub isize);
pub struct WeekOfYearAsStructU8(pub u8);
pub struct WeekOfYearAsStructU16(pub u16);
pub struct WeekOfYearAsStructU32(pub u32);
pub struct WeekOfYearAsStructU64(pub u64);
pub struct WeekOfYearAsStructU128(pub u128);
pub struct WeekOfYearAsStructUSize(pub usize);
pub struct WeekOfYearAsStructF32(pub f32);
pub struct WeekOfYearAsStructF64(pub f64);

pub type WeekOfYearAsTypeI8 = i8;
pub type WeekOfYearAsTypeI16 = i16;
pub type WeekOfYearAsTypeI32 = i32;
pub type WeekOfYearAsTypeI64 = i64;
pub type WeekOfYearAsTypeI128 = i128;
pub type WeekOfYearAsTypeISize = isize;
pub type WeekOfYearAsTypeU8 = u8;
pub type WeekOfYearAsTypeU16 = u16;
pub type WeekOfYearAsTypeU32 = u32;
pub type WeekOfYearAsTypeU64 = u64;
pub type WeekOfYearAsTypeU128 = u128;
pub type WeekOfYearAsTypeUSize = usize;
pub type WeekOfYearAsTypeF32 = f32;
pub type WeekOfYearAsTypeF64 = f64;

//// WeekOfYearAsWW

pub struct WeekOfYearAsWWAsStructStr(pub &'static str);
pub struct WeekOfYearAsWWAsStructString(pub String);

pub type WeekOfYearAsWWAsTypeStr = str;
pub type WeekOfYearAsWWAsTypeString = String;

//// WeekOfYearAsName

pub struct WeekOfYearAsNameAsStructStr(pub &'static str);
pub struct WeekOfYearAsNameAsStructString(pub String);

pub type WeekOfYearAsNameAsTypeStr = str;
pub type WeekOfYearAsNameAsTypeString = String;
