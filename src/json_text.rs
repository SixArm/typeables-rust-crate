//! # JSON Text
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::json_text::*;
//! let x = JSONTextAsStructStr("{\"alpha\":\"bravo\"}");
//! ```

pub struct JSONTextAsStructStr(pub &'static str);
pub struct JSONTextAsStructString(pub String);

pub type JSONTextAsTypeStr = str;
pub type JSONTextAsTypeString = String;
