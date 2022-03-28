//! # Locale Language Code
//!
//! Examples of locales:
//!
//!   * "en-US" means English - United States.
//!   * "zh-CN" means Chinese - China (Simplified variant)
//!   * "hi-IN" means Hindi - India
//!   * "es-ES" means Spanish - Spain
//!   * "ar-EG" means Arabic - Egypt
//!   * "ms-ID" means Malay - Indonesia
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleLanguageCodeAsStructStr("en"); // English
//! let x = LocaleLanguageCodeAsStructStr("zh"); // Chinese
//! let x = LocaleLanguageCodeAsStructStr("hi"); // Hindi
//! let x = LocaleLanguageCodeAsStructStr("es"); // Spanish
//! let x = LocaleLanguageCodeAsStructStr("ar"); // Arabic
//! let x = LocaleLanguageCodeAsStructStr("ms"); // Malay
//! ```
//! 
//! See also:
//!
//!   * [locale_code](../locale_code)
//!   * [decimal_separator](../decimal_separator)
//!   * [grouping_separator](../grouping_separator)
//!   * [quotation_start_delimiter](../quotation_start_delimiter)
//!   * [quotation_stop_delimiter](../quotation_stop_delimiter)
//!

pub struct LocaleLanguageCodeAsStructStr(pub &'static str);
pub struct LocaleLanguageCodeAsStructString(pub String);

pub type LocaleLanguageCodeAsTypeStr = str;
pub type LocaleLanguageCodeAsTypeString = String;
