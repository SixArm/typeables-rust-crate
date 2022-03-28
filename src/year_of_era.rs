//! # YearOfEra
//!
//! 
//! ## YearOfEra as number
//! 
//! Examples:
//!
//! ```rust
//! # use typeables::year_of_era::*;
//! let x = YearOfEraAsStructI16(2022);
//! ```
//!
//! 
//! ## YearOfEra as "YYYY" format
//!
//! Examples:
//!
//! ```rust
//! # use typeables::year_of_era::*;
//! let x = YearOfEraAsYYYYAsStructStr("2022");
//! ```
//! 
//! ## YearOfEra as Name
//!
//! Example:
//!
//! ```rust
//! # use typeables::year_of_era::*;
//! let x = YearOfEraAsNameAsStructStr("Year Two Thousand Twenty Two");
//! ```

//// YearOfEra as number

pub struct YearOfEraAsStructI8(pub i8);
pub struct YearOfEraAsStructI16(pub i16);
pub struct YearOfEraAsStructI32(pub i32);
pub struct YearOfEraAsStructI64(pub i64);
pub struct YearOfEraAsStructI128(pub i128);
pub struct YearOfEraAsStructISize(pub isize);
pub struct YearOfEraAsStructU8(pub u8);
pub struct YearOfEraAsStructU16(pub u16);
pub struct YearOfEraAsStructU32(pub u32);
pub struct YearOfEraAsStructU64(pub u64);
pub struct YearOfEraAsStructU128(pub u128);
pub struct YearOfEraAsStructUSize(pub usize);
pub struct YearOfEraAsStructF32(pub f32);
pub struct YearOfEraAsStructF64(pub f64);

pub type YearOfEraAsTypeI8 = i8;
pub type YearOfEraAsTypeI16 = i16;
pub type YearOfEraAsTypeI32 = i32;
pub type YearOfEraAsTypeI64 = i64;
pub type YearOfEraAsTypeI128 = i128;
pub type YearOfEraAsTypeISize = isize;
pub type YearOfEraAsTypeU8 = u8;
pub type YearOfEraAsTypeU16 = u16;
pub type YearOfEraAsTypeU32 = u32;
pub type YearOfEraAsTypeU64 = u64;
pub type YearOfEraAsTypeU128 = u128;
pub type YearOfEraAsTypeUSize = usize;
pub type YearOfEraAsTypeF32 = f32;
pub type YearOfEraAsTypeF64 = f64;

//// YearOfEra as "YYYY" format

pub struct YearOfEraAsYYYYAsStructStr(pub &'static str);
pub struct YearOfEraAsYYYYAsStructString(pub String);

pub type YearOfEraAsYYYYAsTypeStr = str;
pub type YearOfEraAsYYYYAsTypeString = String;

//// YearOfEraAsName
pub struct YearOfEraAsNameAsStructStr(pub &'static str);
pub struct YearOfEraAsNameAsStructString(pub String);

pub type YearOfEraAsNameAsTypeStr = str;
pub type YearOfEraAsNameAsTypeString = String;
