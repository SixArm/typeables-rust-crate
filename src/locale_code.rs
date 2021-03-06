//! # Locale Code
//!
//! Examples:
//!
//! ```rust
//! # use typeables::locale_code::*;
//! let x = LocaleCodeAsStructStr("en-US"); // English - United States
//! let x = LocaleCodeAsStructStr("zh-CN"); // Chinese - China (Simplified variant)
//! let x = LocaleCodeAsStructStr("hi-IN"); // Hindi - India
//! let x = LocaleCodeAsStructStr("es-ES"); // Spanish - Spain
//! let x = LocaleCodeAsStructStr("ar-EG"); // Arabic - Egypt
//! let x = LocaleCodeAsStructStr("ms-ID"); // Malay - Indonesia
//! ```
//!
//! A locale typically uses a language code and country code:
//!
//!   * "en-US" means English - United States.
//!   * "zh-CN" means Chinese - China (Simplified variant)
//!   * "hi-IN" means Hindi - India
//!   * "es-ES" means Spanish - Spain
//!   * "ar-EG" means Arabic - Egypt
//!   * "ms-ID" means Malay - Indonesia
//!
//! Locale concepts:
//!
//!   * [locale_code](../locale_code)
//!   * [locale_langauge_code](../locale_language_code)
//!   * [locale_country_code](../locale_country_code)
//!   * [locale_region_code](../locale_region_code)
//!   * [locale_script_code](../locale_script_code)
//!   * [locale_variant_code](../locale_variant_code)
//!   * [decimal_separator](../decimal_separator)
//!   * [grouping_separator](../grouping_separator)
//!   * [quotation_start_delimiter](../quotation_start_delimiter)
//!   * [quotation_stop_delimiter](../quotation_stop_delimiter)

pub struct LocaleCodeAsStructStr(pub &'static str);
pub struct LocaleCodeAsStructString(pub String);

pub type LocaleCodeAsTypeStr = str;
pub type LocaleCodeAsTypeString = String;
