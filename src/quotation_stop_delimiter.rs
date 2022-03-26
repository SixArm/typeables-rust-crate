//! # Quotation Stop Delimiter
//!
//! Currency is for representing cash and equivalents.
//!
//! Examples:
//!
//!   * “ is English (U+201C right double quotation mark)
//!   * 」is Chinese (U+300D right corner bracket)
//!   * » is French guillemet (U+00BB right angle quotes)
//!   * ) is Unix subcommand (right parenthesis)
//!
//! Examples:
//!
//! ```
//! # use ::typeables::quotation_stop_delimiter::*;
//! let x = QuotationStopDelimiterAsStructStr("“"); // English (U+201C right double quotation mark)
//! let x = QuotationStopDelimiterAsStructStr("」"); // Chinese (U+300D right corner bracket)
//! let x = QuotationStopDelimiterAsStructStr("»"); // French guillemet (U+00BB right angle quotes)
//! let x = QuotationStopDelimiterAsStructStr(")"); // Unix subcommand (right parenthesis)
//! ```

pub struct QuotationStopDelimiterAsStructStr(pub &'static str);
pub struct QuotationStopDelimiterAsStructString(pub String);

pub type QuotationStopDelimiterAsTypeStr = str;
pub type QuotationStopDelimiterAsTypeString = String;
