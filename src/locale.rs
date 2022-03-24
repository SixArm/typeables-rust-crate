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

/// Locale code as type `str`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleCodeAsTypeStr = "en-US"; // English - United States
/// let x: &LocaleCodeAsTypeStr = "zh-CN"; // Chinese - China (Simplified variant)
/// let x: &LocaleCodeAsTypeStr = "hi-IN"; // Hindi - India
/// let x: &LocaleCodeAsTypeStr = "es-ES"; // Spanish - Spain
/// let x: &LocaleCodeAsTypeStr = "ar-EG"; // Arabic - Egypt
/// let x: &LocaleCodeAsTypeStr = "ms-ID"; // Malay - Indonesia
/// ```
pub type LocaleCodeAsTypeStr = str;

/// Locale code as type `String`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleCodeAsTypeString = "en-US".into(); // English - United States
/// let x: LocaleCodeAsTypeString = "zh-CN".into(); // Chinese - China (Simplified variant)
/// let x: LocaleCodeAsTypeString = "hi-IN".into(); // Hindi - India
/// let x: LocaleCodeAsTypeString = "es-ES".into(); // Spanish - Spain
/// let x: LocaleCodeAsTypeString = "ar-EG".into(); // Arabic - Egypt
/// let x: LocaleCodeAsTypeString = "ms-ID".into(); // Malay - Indonesia
/// ```
pub type LocaleCodeAsTypeString = String;

/// Locale code as struct `str`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleCodeAsStructStr("en-US"); // English - United States
/// let x = LocaleCodeAsStructStr("zh-CN"); // Chinese - China (Simplified variant)
/// let x = LocaleCodeAsStructStr("hi-IN"); // Hindi - India
/// let x = LocaleCodeAsStructStr("es-ES"); // Spanish - Spain
/// let x = LocaleCodeAsStructStr("ar-EG"); // Arabic - Egypt
/// let x = LocaleCodeAsStructStr("ms-ID"); // Malay - Indonesia
/// ```
pub struct LocaleCodeAsStructStr(pub &'static str);

/// Locale code as struct `String`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleCodeAsStructString(String::from("en-US")); // English - United States
/// let x = LocaleCodeAsStructString(String::from("zh-CN")); // Chinese - China (Simplified variant)
/// let x = LocaleCodeAsStructString(String::from("hi-IN")); // Hindi - India
/// let x = LocaleCodeAsStructString(String::from("es-ES")); // Spanish - Spain
/// let x = LocaleCodeAsStructString(String::from("ar-EG")); // Arabic - Egypt
/// let x = LocaleCodeAsStructString(String::from("ms-ID")); // Malay - Indonesia
/// ```
pub struct LocaleCodeAsStructString(pub String);

/// Locale country code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleCountryCodeAsTypeStr = "US"; // United States
/// let x: &LocaleCountryCodeAsTypeStr = "CN"; // China
/// let x: &LocaleCountryCodeAsTypeStr = "IN"; // India
/// let x: &LocaleCountryCodeAsTypeStr = "ES"; // Spain
/// let x: &LocaleCountryCodeAsTypeStr = "EG"; // Egypt
/// let x: &LocaleCountryCodeAsTypeStr = "ID"; // Indonesia
/// ```
pub type LocaleCountryCodeAsTypeStr = str;

/// Locale country code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleCountryCodeAsTypeString = "US".into(); // United States
/// let x: LocaleCountryCodeAsTypeString = "CN".into(); // China
/// let x: LocaleCountryCodeAsTypeString = "IN".into(); // India
/// let x: LocaleCountryCodeAsTypeString = "ES".into(); // Spain
/// let x: LocaleCountryCodeAsTypeString = "EG".into(); // Egypt
/// let x: LocaleCountryCodeAsTypeString = "ID".into(); // Indonesia
/// ```
pub type LocaleCountryCodeAsTypeString = String;

/// Locale country code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleCountryCodeAsStructStr("US"); // United States
/// let x = LocaleCountryCodeAsStructStr("CN"); // China
/// let x = LocaleCountryCodeAsStructStr("IN"); // India
/// let x = LocaleCountryCodeAsStructStr("ES"); // Spain
/// let x = LocaleCountryCodeAsStructStr("EG"); // Egypt
/// let x = LocaleCountryCodeAsStructStr("ID"); // Indonesia
/// ```
pub struct LocaleCountryCodeAsStructStr(pub &'static str);

/// Locale country code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleCountryCodeAsStructString(String::from("US")); // United States
/// let x = LocaleCountryCodeAsStructString(String::from("CN")); // China
/// let x = LocaleCountryCodeAsStructString(String::from("IN")); // India
/// let x = LocaleCountryCodeAsStructString(String::from("ES")); // Spain
/// let x = LocaleCountryCodeAsStructString(String::from("EG")); // Egypt
/// let x = LocaleCountryCodeAsStructString(String::from("ID")); // Indonesia
/// ```
pub struct LocaleCountryCodeAsStructString(pub String);

/// Locale decimal separator as type `str`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleDecimalSeparatorAsTypeStr = "."; // English
/// let x: &LocaleDecimalSeparatorAsTypeStr = ","; // French
/// let x: &LocaleDecimalSeparatorAsTypeStr = "~"; // Some made up language
/// ```
pub type LocaleDecimalSeparatorAsTypeStr = str;

/// Locale decimal separator as type `String`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleDecimalSeparatorAsTypeString = ".".into(); // English
/// let x: LocaleDecimalSeparatorAsTypeString = ",".into(); // Spanish
/// let x: LocaleDecimalSeparatorAsTypeString = "~".into(); // Some made up language
/// ```
pub type LocaleDecimalSeparatorAsTypeString = String;

/// Locale decimal separator as struct `str`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleDecimalSeparatorAsStructStr("."); // English
/// let x = LocaleDecimalSeparatorAsStructStr(","); // French
/// let x = LocaleDecimalSeparatorAsStructStr("~"); // Some made up language
/// ```
pub struct LocaleDecimalSeparatorAsStructStr(pub &'static str);

/// Locale decimal separator as struct `String`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleDecimalSeparatorAsStructString(String::from(".")); // English
/// let x = LocaleDecimalSeparatorAsStructString(String::from(",")); // Spanish
/// let x = LocaleDecimalSeparatorAsStructString(String::from("~")); // Some made up language
/// ```
pub struct LocaleDecimalSeparatorAsStructString(pub String);

/// Locale grouping separator as type `str`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleGroupingSeparatorAsTypeStr = ","; // English
/// let x: &LocaleGroupingSeparatorAsTypeStr = "."; // Spanish
/// let x: &LocaleGroupingSeparatorAsTypeStr = " "; // French
/// ```
pub type LocaleGroupingSeparatorAsTypeStr = str;

/// Locale grouping separator as type `String`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleGroupingSeparatorAsTypeString = ",".into(); // English
/// let x: LocaleGroupingSeparatorAsTypeString = ".".into(); // Spanish
/// let x: LocaleGroupingSeparatorAsTypeString = " ".into(); // French
/// ```
pub type LocaleGroupingSeparatorAsTypeString = String;

/// Locale grouping separator as struct `str`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleGroupingSeparatorAsStructStr(","); // English
/// let x = LocaleGroupingSeparatorAsStructStr("."); // Spanish
/// let x = LocaleGroupingSeparatorAsStructStr(" "); // French
/// ```
pub struct LocaleGroupingSeparatorAsStructStr(pub &'static str);

/// Locale grouping separator as struct `String`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleGroupingSeparatorAsStructString(String::from(",")); // English
/// let x = LocaleGroupingSeparatorAsStructString(String::from(".")); // Spanish
/// let x = LocaleGroupingSeparatorAsStructString(String::from(" ")); // French
/// ```
pub struct LocaleGroupingSeparatorAsStructString(pub String);

/// Locale language code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleLanguageCodeAsTypeStr = "en"; // English
/// let x: &LocaleLanguageCodeAsTypeStr = "zh"; // Chinese
/// let x: &LocaleLanguageCodeAsTypeStr = "hi"; // Hindi
/// let x: &LocaleLanguageCodeAsTypeStr = "es"; // Spanish
/// let x: &LocaleLanguageCodeAsTypeStr = "ar"; // Arabic
/// let x: &LocaleLanguageCodeAsTypeStr = "ms"; // Malay
/// ```
pub type LocaleLanguageCodeAsTypeStr = str;

/// Locale language code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleLanguageCodeAsTypeString = "en".into(); // English
/// let x: LocaleLanguageCodeAsTypeString = "zh".into(); // Chinese
/// let x: LocaleLanguageCodeAsTypeString = "hi".into(); // Hindi
/// let x: LocaleLanguageCodeAsTypeString = "es".into(); // Spanish
/// let x: LocaleLanguageCodeAsTypeString = "ar".into(); // Arabic
/// let x: LocaleLanguageCodeAsTypeString = "ms".into(); // Malay
/// ```
pub type LocaleLanguageCodeAsTypeString = String;

/// Locale language code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleLanguageCodeAsStructStr("en"); // English
/// let x = LocaleLanguageCodeAsStructStr("zh"); // Chinese
/// let x = LocaleLanguageCodeAsStructStr("hi"); // Hindi
/// let x = LocaleLanguageCodeAsStructStr("es"); // Spanish
/// let x = LocaleLanguageCodeAsStructStr("ar"); // Arabic
/// let x = LocaleLanguageCodeAsStructStr("ms"); // Malay
/// ```
pub struct LocaleLanguageCodeAsStructStr(pub &'static str);

/// Locale language code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleLanguageCodeAsStructString(String::from("en")); // English
/// let x = LocaleLanguageCodeAsStructString(String::from("zh")); // Chinese
/// let x = LocaleLanguageCodeAsStructString(String::from("hi")); // Hindi
/// let x = LocaleLanguageCodeAsStructString(String::from("es")); // Spanish
/// let x = LocaleLanguageCodeAsStructString(String::from("ar")); // Arabic
/// let x = LocaleLanguageCodeAsStructString(String::from("ms")); // Malay
/// ```
pub struct LocaleLanguageCodeAsStructString(pub String);

/// Locale region code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleRegionCodeAsTypeStr = "QO"; // Outlying Oceania
/// let x: &LocaleRegionCodeAsTypeStr = "EU"; // European Union
/// let x: &LocaleRegionCodeAsTypeStr = "ZZ"; // Unknown or Invalid Territory
/// ```
pub type LocaleRegionCodeAsTypeStr = str;

/// Locale region code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleRegionCodeAsTypeString = "QO".into(); // Outlying Oceania
/// let x: LocaleRegionCodeAsTypeString = "EU".into(); // European Union
/// let x: LocaleRegionCodeAsTypeString = "ZZ".into(); // Unknown or Invalid Territory
/// ```
pub type LocaleRegionCodeAsTypeString = String;

/// Locale region code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleRegionCodeAsStructStr("QO"); // Outlying Oceania
/// let x = LocaleRegionCodeAsStructStr("EU"); // European Union
/// let x = LocaleRegionCodeAsStructStr("ZZ"); // Unknown or Invalid Territory
/// ```
pub struct LocaleRegionCodeAsStructStr(pub &'static str);

/// Locale region code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleRegionCodeAsStructString(String::from("QO")); // Outlying Oceania
/// let x = LocaleRegionCodeAsStructString(String::from("EU")); // European Union
/// let x = LocaleRegionCodeAsStructString(String::from("ZZ")); // Unknown or Invalid Territory
/// ```
pub struct LocaleRegionCodeAsStructString(pub String);

/// Locale script code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleScriptCodeAsTypeStr = "Latn"; // Latin
/// let x: &LocaleScriptCodeAsTypeStr = "Hans"; // Han (simplified script)
/// let x: &LocaleScriptCodeAsTypeStr = "Deva"; // Devanagari (Nagari)
/// let x: &LocaleScriptCodeAsTypeStr = "Lina"; // Linear A
/// let x: &LocaleScriptCodeAsTypeStr = "Arab"; // Arabic
/// let x: &LocaleScriptCodeAsTypeStr = "Mlym"; // Malayalam
/// ```
pub type LocaleScriptCodeAsTypeStr  = str;

/// Locale script code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleScriptCodeAsTypeString = "Latn".into(); // Latin
/// let x: LocaleScriptCodeAsTypeString = "Hans".into(); // Han (simplified script)
/// let x: LocaleScriptCodeAsTypeString = "Deva".into(); // Devanagari (Nagari)
/// let x: LocaleScriptCodeAsTypeString = "Lina".into(); // Linear A
/// let x: LocaleScriptCodeAsTypeString = "Arab".into(); // Arabic
/// let x: LocaleScriptCodeAsTypeString = "Mlym".into(); // Malayalam
/// ```
pub type LocaleScriptCodeAsTypeString  = String;

/// Locale script code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleScriptCodeAsStructStr("Latn"); // Latin
/// let x = LocaleScriptCodeAsStructStr("Hans"); // Han (simplified script)
/// let x = LocaleScriptCodeAsStructStr("Deva"); // Devanagari (Nagari)
/// let x = LocaleScriptCodeAsStructStr("Lina"); // Linear A
/// let x = LocaleScriptCodeAsStructStr("Arab"); // Arabic
/// let x = LocaleScriptCodeAsStructStr("Mlym"); // Malayalam
/// ```
pub struct LocaleScriptCodeAsStructStr (pub &'static str);

/// Locale script code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleScriptCodeAsStructString(String::from("Latn")); // Latin
/// let x = LocaleScriptCodeAsStructString(String::from("Hans")); // Han (simplified script)
/// let x = LocaleScriptCodeAsStructString(String::from("Deva")); // Devanagari (Nagari)
/// let x = LocaleScriptCodeAsStructString(String::from("Lina")); // Linear A
/// let x = LocaleScriptCodeAsStructString(String::from("Arab")); // Arabic
/// let x = LocaleScriptCodeAsStructString(String::from("Mlym")); // Malayalam
/// ```
pub struct LocaleScriptCodeAsStructString (pub String);

/// Locale variant code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleVariantCodeAsTypeStr = "arevela";  // Eastern dialect of Armenian
/// let x: &LocaleVariantCodeAsTypeStr = "biscayan"; // Biscayan dialect of Basque
/// let x: &LocaleVariantCodeAsTypeStr = "cisaup";   // Cisalpine dialect of Italian
/// let x: &LocaleVariantCodeAsTypeStr = "dajnko";   // Slovene in Dajnko alphabet
/// let x: &LocaleVariantCodeAsTypeStr = "ekavsk";   // Serbian with Ekavian pronunciation
/// let x: &LocaleVariantCodeAsTypeStr = "fonipa";   // International Phonetic Alphabet
/// ```
pub type LocaleVariantCodeAsTypeStr = str;

/// Locale variant code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleVariantCodeAsTypeString = "arevela".into();  // Eastern dialect of Armenian
/// let x: LocaleVariantCodeAsTypeString = "biscayan".into(); // Biscayan dialect of Basque
/// let x: LocaleVariantCodeAsTypeString = "cisaup".into();   // Cisalpine dialect of Italian
/// let x: LocaleVariantCodeAsTypeString = "dajnko".into();   // Slovene in Dajnko alphabet
/// let x: LocaleVariantCodeAsTypeString = "ekavsk".into();   // Serbian with Ekavian pronunciation
/// let x: LocaleVariantCodeAsTypeString = "fonipa".into();   // International Phonetic Alphabet
/// ```
pub type LocaleVariantCodeAsTypeString = String;

/// Locale variant code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleVariantCodeAsStructStr("arevela");  // Eastern dialect of Armenian
/// let x = LocaleVariantCodeAsStructStr("biscayan"); // Biscayan dialect of Basque
/// let x = LocaleVariantCodeAsStructStr("cisaup");   // Cisalpine dialect of Italian
/// let x = LocaleVariantCodeAsStructStr("dajnko");   // Slovene in Dajnko alphabet
/// let x = LocaleVariantCodeAsStructStr("ekavsk");   // Serbian with Ekavian pronunciation
/// let x = LocaleVariantCodeAsStructStr("fonipa");   // International Phonetic Alphabet
/// ```
pub struct LocaleVariantCodeAsStructStr(pub &'static str);

/// Locale variant code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x = LocaleVariantCodeAsStructString(String::from("arevela"));  // Eastern dialect of Armenian
/// let x = LocaleVariantCodeAsStructString(String::from("biscayan")); // Biscayan dialect of Basque
/// let x = LocaleVariantCodeAsStructString(String::from("cisaup"));   // Cisalpine dialect of Italian
/// let x = LocaleVariantCodeAsStructString(String::from("dajnko"));   // Slovene in Dajnko alphabet
/// let x = LocaleVariantCodeAsStructString(String::from("ekavsk"));   // Serbian with Ekavian pronunciation
/// let x = LocaleVariantCodeAsStructString(String::from("fonipa"));   // International Phonetic Alphabet
/// ```
pub struct LocaleVariantCodeAsStructString(pub String);
