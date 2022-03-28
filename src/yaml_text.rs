//! # YAML Text
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::yaml_text::*;
//! let x = YAMLTextAsStructStr("\"alpha\":\"bravo\"");
//! ```

pub struct YAMLTextAsStructStr(pub &'static str);
pub struct YAMLTextAsStructString(pub String);

pub type YAMLTextAsTypeStr = str;
pub type YAMLTextAsTypeString = String;
