//! # Locale Region Code
//!
//! Examples:
//!
//! ```rust
//! # use typeables::locale_region_code::*;
//! let x = LocaleRegionCodeAsStructStr("QO"); // Outlying Oceania
//! let x = LocaleRegionCodeAsStructStr("EU"); // European Union
//! let x = LocaleRegionCodeAsStructStr("ZZ"); // Unknown or Invalid Territory
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

pub struct LocaleRegionCodeAsStructStr(pub &'static str);
pub struct LocaleRegionCodeAsStructString(pub String);

pub type LocaleRegionCodeAsTypeStr = str;
pub type LocaleRegionCodeAsTypeString = String;
