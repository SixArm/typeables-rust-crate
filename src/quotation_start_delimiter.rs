//! # Quotation Start Delimiter
//!
//! Currency is for representing cash and equivalents.
//!
//! Examples:
//!
//!   * “ is English (U+201C left double quotation mark)
//!   * 「 is Chinese (U+300C left corner bracket)
//!   * « is French gullemet (U+00AB left angle quotes)
//!   * ( is Unix subcommand (left parenthesis)
//!
//! Examples:
//!
//! ```
//! # use typeables::quotation_start_delimiter::*;
//! let x = QuotationStartDelimiterAsStructStr("“"); // English (U+201C left double quotation mark)
//! let x = QuotationStartDelimiterAsStructStr("「"); // Chinese (U+300C left corner bracket)
//! let x = QuotationStartDelimiterAsStructStr("«"); // French gullemet (U+00AB left angle quotes)
//! let x = QuotationStartDelimiterAsStructStr("("); // Unix subcommand (left parenthesis)
//! ```

pub struct QuotationStartDelimiterAsStructStr(pub &'static str);
pub struct QuotationStartDelimiterAsStructString(pub String);

pub type QuotationStartDelimiterAsTypeStr = str;
pub type QuotationStartDelimiterAsTypeString = String;
