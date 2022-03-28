//! # Adverb
//!
//! Examples:
//!
//! ```rust
//! # use typeables::adverb::*;
//! let x = AdverbAsStructStr("quickly"); // timings
//! let x = AdverbAsStructStr("quietly"); // sensings
//! let x = AdverbAsStructStr("happily"); // feelings
//! ```
//!
//! <https://wikipedia.org/wiki/Adverb>

pub struct AdverbAsStructStr(pub &'static str);
pub struct AdverbAsStructString(pub String);

pub type AdverbAsTypeStr = str;
pub type AdverbAsTypeString = String;
