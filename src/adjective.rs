//! # Adjective
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::adjective::*;
//! let x = AdjectiveAsStructStr("large"); // size
//! let x = AdjectiveAsStructStr("round"); // shape
//! let x = AdjectiveAsStructStr("green"); // color
//! ```
//!
//! https://wikipedia.org/wiki/Adjective

pub struct AdjectiveAsStructStr(pub &'static str);
pub struct AdjectiveAsStructString(pub String);

pub type AdjectiveAsTypeStr = str;
pub type AdjectiveAsTypeString = String;
