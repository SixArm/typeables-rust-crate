//! # HTML Text
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::html_text::*;
//! let x = HTMLTextAsStructStr("This is <b>bold</b> text");
//! ```

pub struct HTMLTextAsStructStr(pub &'static str);
pub struct HTMLTextAsStructString(pub String);

pub type HTMLTextAsTypeStr = str;
pub type HTMLTextAsTypeString = String;
