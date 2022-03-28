//! # SecondText
//!
//! Examples:
//!
//! ```rust
//! # use typeables::second_unit::*;
//! let x = SecondTextAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Second>
//!
//// Second as "SS" format from "00" to "59"

pub struct SecondTextAsSSAsStructStr(pub &'static str);
pub struct SecondTextAsSSAsStructString(pub String);

pub type SecondTextAsSSAsTypeStr = str;
pub type SecondTextAsSSAsTypeString = String;
