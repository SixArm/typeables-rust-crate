//! # Decimal Separator
//!
//! Decimal Separator a.k.a. radix character for numbers.
//!
//! Examples:
//!
//! ```rust
//! # use typeables::decimal_separator::*;
//! let x = DecimalSeparatorAsStructStr("."); // English
//! let x = DecimalSeparatorAsStructStr(","); // French
//! let x = DecimalSeparatorAsStructStr("~"); // Some made up language
//! ```

pub struct DecimalSeparatorAsStructStr(pub &'static str);
pub struct DecimalSeparatorAsStructString(pub String);

pub type DecimalSeparatorAsTypeStr = str;
pub type DecimalSeparatorAsTypeString = String;
