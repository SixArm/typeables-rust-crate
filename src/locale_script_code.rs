//! # Locale Script Code
//!
//! Examples:
//!
//! ```rust
//! # use typeables::locale_script_code::*;
//! let x = LocaleScriptCodeAsStructStr("Latn"); // Latin
//! let x = LocaleScriptCodeAsStructStr("Hans"); // Han (simplified script)
//! let x = LocaleScriptCodeAsStructStr("Deva"); // Devanagari (Nagari)
//! let x = LocaleScriptCodeAsStructStr("Lina"); // Linear A
//! let x = LocaleScriptCodeAsStructStr("Arab"); // Arabic
//! let x = LocaleScriptCodeAsStructStr("Mlym"); // Malayalam
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

pub struct LocaleScriptCodeAsStructStr (pub &'static str);
pub struct LocaleScriptCodeAsStructString (pub String);

pub type LocaleScriptCodeAsTypeStr  = str;
pub type LocaleScriptCodeAsTypeString  = String;
