//! # Locale Language Code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale_language_code::*;
//! let x = LocaleLanguageCodeAsStructStr("en"); // English
//! let x = LocaleLanguageCodeAsStructStr("zh"); // Chinese
//! let x = LocaleLanguageCodeAsStructStr("hi"); // Hindi
//! let x = LocaleLanguageCodeAsStructStr("es"); // Spanish
//! let x = LocaleLanguageCodeAsStructStr("ar"); // Arabic
//! let x = LocaleLanguageCodeAsStructStr("ms"); // Malay
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

pub struct LocaleLanguageCodeAsStructStr(pub &'static str);
pub struct LocaleLanguageCodeAsStructString(pub String);

pub type LocaleLanguageCodeAsTypeStr = str;
pub type LocaleLanguageCodeAsTypeString = String;
