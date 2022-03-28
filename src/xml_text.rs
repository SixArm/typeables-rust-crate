//! # XML Text
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::xml_text::*;
//! let x = XMLTextAsStructStr("<alpha>bravo</alpha>");
//! ```

pub struct XMLTextAsStructStr(pub &'static str);
pub struct XMLTextAsStructString(pub String);

pub type XMLTextAsTypeStr = str;
pub type XMLTextAsTypeString = String;
