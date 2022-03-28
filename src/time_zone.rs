//! ## Time zone as abbreviation text
//!
//! Examples:
//! ```
//! # use typeables::time_zone::*;
//! let x = TimeZoneAsAbbreviationAsStructStr("ET"); // U.S. Eastern Time
//! let x = TimeZoneAsAbbreviationAsStructStr("BST"); // British Summer Time
//! ```

pub struct TimeZoneAsAbbreviationAsStructStr(pub &'static str);
pub struct TimeZoneAsAbbreviationAsStructString(pub String);

pub type TimeZoneAsAbbreviationAsTypeStr = str;
pub type TimeZoneAsAbbreviationAsTypeString = String;
