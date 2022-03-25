//! # Locale
//!
//! Examples:
//!
//!   * "en-US" means English - United States.
//!   * "zh-CN" means Chinese - China (Simplified variant)
//!   * "hi-IN" means Hindi - India
//!   * "es-ES" means Spanish - Spain
//!   * "ar-EG" means Arabic - Egypt
//!   * "ms-ID" means Malay - Indonesia
//!
//! 
//! ## Locale Code a.k.a. language code and country code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleCodeAsStructStr("en-US"); // English - United States
//! let x = LocaleCodeAsStructStr("zh-CN"); // Chinese - China (Simplified variant)
//! let x = LocaleCodeAsStructStr("hi-IN"); // Hindi - India
//! let x = LocaleCodeAsStructStr("es-ES"); // Spanish - Spain
//! let x = LocaleCodeAsStructStr("ar-EG"); // Arabic - Egypt
//! let x = LocaleCodeAsStructStr("ms-ID"); // Malay - Indonesia
//! ```
//! 
//! 
//! ## Locale Language Code
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
//! 
//! ## Locale Country Code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleCountryCodeAsStructStr("US"); // United States
//! let x = LocaleCountryCodeAsStructStr("CN"); // China
//! let x = LocaleCountryCodeAsStructStr("IN"); // India
//! let x = LocaleCountryCodeAsStructStr("ES"); // Spain
//! let x = LocaleCountryCodeAsStructStr("EG"); // Egypt
//! let x = LocaleCountryCodeAsStructStr("ID"); // Indonesia
//! ```
//!
//!
//! ## Locale Decimal Separator a.k.a. radix character for numbers
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleDecimalSeparatorAsStructStr("."); // English
//! let x = LocaleDecimalSeparatorAsStructStr(","); // French
//! let x = LocaleDecimalSeparatorAsStructStr("~"); // Some made up language
//! ```
//! 
//! 
//! ## Locale Grouping Separator a.k.a. thousands separator for numbers
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleGroupingSeparatorAsStructStr(","); // English
//! let x = LocaleGroupingSeparatorAsStructStr("."); // Spanish
//! let x = LocaleGroupingSeparatorAsStructStr(" "); // French
//! ```
//!
//! 
//! ## Locale Region Code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleRegionCodeAsStructStr("QO"); // Outlying Oceania
//! let x = LocaleRegionCodeAsStructStr("EU"); // European Union
//! let x = LocaleRegionCodeAsStructStr("ZZ"); // Unknown or Invalid Territory
//! ```
//! 
//! 
//! ## Locale Script Code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleScriptCodeAsStructStr("Latn"); // Latin
//! let x = LocaleScriptCodeAsStructStr("Hans"); // Han (simplified script)
//! let x = LocaleScriptCodeAsStructStr("Deva"); // Devanagari (Nagari)
//! let x = LocaleScriptCodeAsStructStr("Lina"); // Linear A
//! let x = LocaleScriptCodeAsStructStr("Arab"); // Arabic
//! let x = LocaleScriptCodeAsStructStr("Mlym"); // Malayalam
//! ```
//!
//! 
//! ## Locale Variant Code
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::locale::*;
//! let x = LocaleVariantCodeAsStructStr("arevela");  // Eastern dialect of Armenian
//! let x = LocaleVariantCodeAsStructStr("biscayan"); // Biscayan dialect of Basque
//! let x = LocaleVariantCodeAsStructStr("cisaup");   // Cisalpine dialect of Italian
//! let x = LocaleVariantCodeAsStructStr("dajnko");   // Slovene in Dajnko alphabet
//! let x = LocaleVariantCodeAsStructStr("ekavsk");   // Serbian with Ekavian pronunciation
//! let x = LocaleVariantCodeAsStructStr("fonipa");   // International Phonetic Alphabet
//! ```

//// Locale Code

pub struct LocaleCodeAsStructStr(pub &'static str);
pub struct LocaleCodeAsStructString(pub String);

pub type LocaleCodeAsTypeStr = str;
pub type LocaleCodeAsTypeString = String;

//// Locale Language Code

pub struct LocaleLanguageCodeAsStructStr(pub &'static str);
pub struct LocaleLanguageCodeAsStructString(pub String);

pub type LocaleLanguageCodeAsTypeStr = str;
pub type LocaleLanguageCodeAsTypeString = String;

//// Locale Country Code

pub struct LocaleCountryCodeAsStructStr(pub &'static str);
pub struct LocaleCountryCodeAsStructString(pub String);

pub type LocaleCountryCodeAsTypeStr = str;
pub type LocaleCountryCodeAsTypeString = String;

//// Locale Decimal Separator a.k.a. radix character for numbers

pub struct LocaleDecimalSeparatorAsStructStr(pub &'static str);
pub struct LocaleDecimalSeparatorAsStructString(pub String);

pub type LocaleDecimalSeparatorAsTypeStr = str;
pub type LocaleDecimalSeparatorAsTypeString = String;

//// Locale Grouping Separator a.k.a. thousands separator for numbers

pub struct LocaleGroupingSeparatorAsStructStr(pub &'static str);
pub struct LocaleGroupingSeparatorAsStructString(pub String);

pub type LocaleGroupingSeparatorAsTypeStr = str;
pub type LocaleGroupingSeparatorAsTypeString = String;

//// Locale Region Code

pub struct LocaleRegionCodeAsStructStr(pub &'static str);
pub struct LocaleRegionCodeAsStructString(pub String);

pub type LocaleRegionCodeAsTypeStr = str;
pub type LocaleRegionCodeAsTypeString = String;

//// Locale script code

pub struct LocaleScriptCodeAsStructStr (pub &'static str);
pub struct LocaleScriptCodeAsStructString (pub String);

pub type LocaleScriptCodeAsTypeStr  = str;
pub type LocaleScriptCodeAsTypeString  = String;

//// Locale Variant Code

pub struct LocaleVariantCodeAsStructString(pub String);
pub struct LocaleVariantCodeAsStructStr(pub &'static str);

pub type LocaleVariantCodeAsTypeStr = str;
pub type LocaleVariantCodeAsTypeString = String;
