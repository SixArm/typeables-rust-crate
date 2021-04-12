/// Locale code as `str`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleCodeStr = "en-US"; // English - United States
/// let x: &LocaleCodeStr = "zh-CN"; // Chinese - China (Simplified variant)
/// let x: &LocaleCodeStr = "hi-IN"; // Hindi - India
/// let x: &LocaleCodeStr = "es-ES"; // Spanish - Spain
/// let x: &LocaleCodeStr = "ar-EG"; // Arabic - Egypt
/// let x: &LocaleCodeStr = "ms-ID"; // Malay - Indonesia
/// ```
pub type LocaleCodeStr = str;

/// Locale code as `String`, such as for language code and country code.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleCodeString = "en-US".into(); // English - United States
/// let x: LocaleCodeString = "zh-CN".into(); // Chinese - China (Simplified variant)
/// let x: LocaleCodeString = "hi-IN".into(); // Hindi - India
/// let x: LocaleCodeString = "es-ES".into(); // Spanish - Spain
/// let x: LocaleCodeString = "ar-EG".into(); // Arabic - Egypt
/// let x: LocaleCodeString = "ms-ID".into(); // Malay - Indonesia
/// ```
pub type LocaleCodeString = String;

/// Locale country code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleCountryCodeStr = "US"; // United States
/// let x: &LocaleCountryCodeStr = "CN"; // China
/// let x: &LocaleCountryCodeStr = "IN"; // India
/// let x: &LocaleCountryCodeStr = "ES"; // Spain
/// let x: &LocaleCountryCodeStr = "EG"; // Egypt
/// let x: &LocaleCountryCodeStr = "ID"; // Indonesia
/// ```
pub type LocaleCountryCodeStr = str;

/// Locale country code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleCountryCodeString = "US".into(); // United States
/// let x: LocaleCountryCodeString = "CN".into(); // China
/// let x: LocaleCountryCodeString = "IN".into(); // India
/// let x: LocaleCountryCodeString = "ES".into(); // Spain
/// let x: LocaleCountryCodeString = "EG".into(); // Egypt
/// let x: LocaleCountryCodeString = "ID".into(); // Indonesia
/// ```
pub type LocaleCountryCodeString = String;

/// Locale decimal separator as `str`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleDecimalSeparatorStr = "."; // English
/// let x: &LocaleDecimalSeparatorStr = ","; // French
/// let x: &LocaleDecimalSeparatorStr = "~"; // Some made up language
/// ```
pub type LocaleDecimalSeparatorStr = str;

/// Locale decimal separator as `String`, a.k.a. radix character for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleDecimalSeparatorString = ".".into(); // English
/// let x: LocaleDecimalSeparatorString = ",".into(); // Spanish
/// let x: LocaleDecimalSeparatorString = "~".into(); // Some made up language
/// ```
pub type LocaleDecimalSeparatorString = String;

/// Locale grouping separator as `str`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleGroupingSeparatorStr = ","; // English
/// let x: &LocaleGroupingSeparatorStr = "."; // Spanish
/// let x: &LocaleGroupingSeparatorStr = " "; // French
/// ```
pub type LocaleGroupingSeparatorStr = str;

/// Locale grouping separator as `String`, a.k.a. thousands separator for numbers.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleGroupingSeparatorString = ",".into(); // English
/// let x: LocaleGroupingSeparatorString = ".".into(); // Spanish
/// let x: LocaleGroupingSeparatorString = " ".into(); // French
/// ```
pub type LocaleGroupingSeparatorString = String;

/// Locale language code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleLanguageCodeStr = "en"; // English
/// let x: &LocaleLanguageCodeStr = "zh"; // Chinese
/// let x: &LocaleLanguageCodeStr = "hi"; // Hindi
/// let x: &LocaleLanguageCodeStr = "es"; // Spanish
/// let x: &LocaleLanguageCodeStr = "ar"; // Arabic
/// let x: &LocaleLanguageCodeStr = "ms"; // Malay
/// ```
pub type LocaleLanguageCodeStr = str;

/// Locale language code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleLanguageCodeString = "en".into(); // English
/// let x: LocaleLanguageCodeString = "zh".into(); // Chinese
/// let x: LocaleLanguageCodeString = "hi".into(); // Hindi
/// let x: LocaleLanguageCodeString = "es".into(); // Spanish
/// let x: LocaleLanguageCodeString = "ar".into(); // Arabic
/// let x: LocaleLanguageCodeString = "ms".into(); // Malay
/// ```
pub type LocaleLanguageCodeString = String;

/// Locale region code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleRegionCodeStr = "QO"; // Outlying Oceania
/// let x: &LocaleRegionCodeStr = "EU"; // European Union
/// let x: &LocaleRegionCodeStr = "ZZ"; // Unknown or Invalid Territory
/// ```
pub type LocaleRegionCodeStr = str;

/// Locale region code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleRegionCodeString = "QO".into(); // Outlying Oceania
/// let x: LocaleRegionCodeString = "EU".into(); // European Union
/// let x: LocaleRegionCodeString = "ZZ".into(); // Unknown or Invalid Territory
/// ```
pub type LocaleRegionCodeString = String;

/// Locale script code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleScriptCodeStr = "Latn"; // Latin
/// let x: &LocaleScriptCodeStr = "Hans"; // Han (simplified script)
/// let x: &LocaleScriptCodeStr = "Deva"; // Devanagari (Nagari)
/// let x: &LocaleScriptCodeStr = "Lina"; // Linear A
/// let x: &LocaleScriptCodeStr = "Arab"; // Arabic
/// let x: &LocaleScriptCodeStr = "Mlym"; // Malayalam
/// ```
pub type LocaleScriptCodeStr  = str;

/// Locale script code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleScriptCodeString = "Latn".into(); // Latin
/// let x: LocaleScriptCodeString = "Hans".into(); // Han (simplified script)
/// let x: LocaleScriptCodeString = "Deva".into(); // Devanagari (Nagari)
/// let x: LocaleScriptCodeString = "Lina".into(); // Linear A
/// let x: LocaleScriptCodeString = "Arab".into(); // Arabic
/// let x: LocaleScriptCodeString = "Mlym".into(); // Malayalam
/// ```
pub type LocaleScriptCodeString  = String;

/// Locale variant code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: &LocaleVariantCodeStr = "arevela";  // Eastern dialect of Armenian
/// let x: &LocaleVariantCodeStr = "biscayan"; // Biscayan dialect of Basque
/// let x: &LocaleVariantCodeStr = "cisaup";   // Cisalpine dialect of Italian
/// let x: &LocaleVariantCodeStr = "dajnko";   // Slovene in Dajnko alphabet
/// let x: &LocaleVariantCodeStr = "ekavsk";   // Serbian with Ekavian pronunciation
/// let x: &LocaleVariantCodeStr = "fonipa";   // International Phonetic Alphabet
/// ```
pub type LocaleVariantCodeStr = str;

/// Locale variant code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::locale::*;
/// let x: LocaleVariantCodeString = "arevela".into();  // Eastern dialect of Armenian
/// let x: LocaleVariantCodeString = "biscayan".into(); // Biscayan dialect of Basque
/// let x: LocaleVariantCodeString = "cisaup".into();   // Cisalpine dialect of Italian
/// let x: LocaleVariantCodeString = "dajnko".into();   // Slovene in Dajnko alphabet
/// let x: LocaleVariantCodeString = "ekavsk".into();   // Serbian with Ekavian pronunciation
/// let x: LocaleVariantCodeString = "fonipa".into();   // International Phonetic Alphabet
/// ```
pub type LocaleVariantCodeString = String;
