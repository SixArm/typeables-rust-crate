//! # Grouping Separator a.k.a. thousands separator for numbers
//!
//! Grouping Separator e.g. numeric thousands separator
//!
//! Examples:
//!
//! ```rust
//! # use typeables::grouping_separator::*;
//! let x = GroupingSeparatorAsStructStr(","); // English
//! let x = GroupingSeparatorAsStructStr("."); // Spanish
//! let x = GroupingSeparatorAsStructStr(" "); // French
//! ```

pub struct GroupingSeparatorAsStructStr(pub &'static str);
pub struct GroupingSeparatorAsStructString(pub String);

pub type GroupingSeparatorAsTypeStr = str;
pub type GroupingSeparatorAsTypeString = String;

