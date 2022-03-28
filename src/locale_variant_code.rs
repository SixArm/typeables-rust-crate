//! # Locale Variant Code
//!
//! Examples:
//!
//! ```rust
//! # use typeables::locale_variant_code::*;
//! let x = LocaleVariantCodeAsStructStr("arevela");  // Eastern dialect of Armenian
//! let x = LocaleVariantCodeAsStructStr("biscayan"); // Biscayan dialect of Basque
//! let x = LocaleVariantCodeAsStructStr("cisaup");   // Cisalpine dialect of Italian
//! let x = LocaleVariantCodeAsStructStr("dajnko");   // Slovene in Dajnko alphabet
//! let x = LocaleVariantCodeAsStructStr("ekavsk");   // Serbian with Ekavian pronunciation
//! let x = LocaleVariantCodeAsStructStr("fonipa");   // International Phonetic Alphabet
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

pub struct LocaleVariantCodeAsStructString(pub String);
pub struct LocaleVariantCodeAsStructStr(pub &'static str);

pub type LocaleVariantCodeAsTypeStr = str;
pub type LocaleVariantCodeAsTypeString = String;
