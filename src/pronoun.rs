//! # Pronoun
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::pronoun::*;
//! let x = PronounAsStructStr("they"); // people
//! let x = PronounAsStructStr("those"); // things
//! ```
//!
//! <https://wikipedia.org/wiki/Pronoun>

pub struct PronounAsStructStr(pub &'static str);
pub struct PronounAsStructString(pub String);

pub type PronounAsTypeStr = str;
pub type PronounAsTypeString = String;
