//! # Currency

/// Currency name as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: &CurrencyNameAsTypeStr = "United States Dollar";
/// let x: &CurrencyNameAsTypeStr = "Chinese Yuan";
/// let x: &CurrencyNameAsTypeStr = "Indian Rupee";
/// let x: &CurrencyNameAsTypeStr = "European Euro";
/// let x: &CurrencyNameAsTypeStr = "Egyptian Pound";
/// let x: &CurrencyNameAsTypeStr = "Indonesian Rupiah";
/// ```
pub type CurrencyNameAsTypeStr = str;

/// Currency name as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: CurrencyNameAsTypeString = String::from("United States Dollar");
/// let x: CurrencyNameAsTypeString = String::from("Chinese Yuan");
/// let x: CurrencyNameAsTypeString = String::from("Indian Rupee");
/// let x: CurrencyNameAsTypeString = String::from("European Euro");
/// let x: CurrencyNameAsTypeString = String::from("Egyptian Pound");
/// let x: CurrencyNameAsTypeString = String::from("Indonesian Rupiah");
/// ```
pub type CurrencyNameAsTypeString = String;

/// Currency name as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x = CurrencyNameAsStructStr("United States Dollar");
/// let x = CurrencyNameAsStructStr("Chinese Yuan");
/// let x = CurrencyNameAsStructStr("Indian Rupee");
/// let x = CurrencyNameAsStructStr("European Euro");
/// let x = CurrencyNameAsStructStr("Egyptian Pound");
/// let x = CurrencyNameAsStructStr("Indonesian Rupiah");
/// ```
pub struct CurrencyNameAsStructStr(pub &'static str);

/// Currency name as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x = CurrencyNameAsStructString(String::from("United States Dollar"));
/// let x = CurrencyNameAsStructString(String::from("Chinese Yuan"));
/// let x = CurrencyNameAsStructString(String::from("Indian Rupee"));
/// let x = CurrencyNameAsStructString(String::from("European Euro"));
/// let x = CurrencyNameAsStructString(String::from("Egyptian Pound"));
/// let x = CurrencyNameAsStructString(String::from("Indonesian Rupiah"));
/// ```
pub struct CurrencyNameAsStructString(pub String);

/// Currency symbol as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: &CurrencySymbolAsTypeStr = "$"; // United States Dollar
/// let x: &CurrencySymbolAsTypeStr = "¥"; // Chinese Yuan
/// let x: &CurrencySymbolAsTypeStr = "₹"; // Indian Rupee
/// let x: &CurrencySymbolAsTypeStr = "€"; // European Euro
/// let x: &CurrencySymbolAsTypeStr = "ج.م"; // Egyptian Pound
/// let x: &CurrencySymbolAsTypeStr = "Rp"; // Indonesian Rupiah
/// ```
pub type CurrencySymbolAsTypeStr = str;

/// Currency symbol as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: CurrencySymbolAsTypeString = "$".into(); // United States Dollar
/// let x: CurrencySymbolAsTypeString = "¥".into(); // Chinese Yuan
/// let x: CurrencySymbolAsTypeString = "₹".into(); // Indian Rupee
/// let x: CurrencySymbolAsTypeString = "€".into(); // European Euro
/// let x: CurrencySymbolAsTypeString = "ج.م".into(); // Egyptian Pound
/// let x: CurrencySymbolAsTypeString = "Rp".into(); // Indonesian Rupiah
/// ```
pub type CurrencySymbolAsTypeString = String;

/// Currency symbol as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x = CurrencySymbolAsStructStr("$"); // United States Dollar
/// let x = CurrencySymbolAsStructStr("¥"); // Chinese Yuan
/// let x = CurrencySymbolAsStructStr("₹"); // Indian Rupee
/// let x = CurrencySymbolAsStructStr("€"); // European Euro
/// let x = CurrencySymbolAsStructStr("ج.م"); // Egyptian Pound
/// let x = CurrencySymbolAsStructStr("Rp"); // Indonesian Rupiah
/// ```
pub struct CurrencySymbolAsStructStr(pub &'static str);

/// Currency symbol as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x = CurrencySymbolAsStructString(String::from("$")); // United States Dollar
/// let x = CurrencySymbolAsStructString(String::from("¥")); // Chinese Yuan
/// let x = CurrencySymbolAsStructString(String::from("₹")); // Indian Rupee
/// let x = CurrencySymbolAsStructString(String::from("€")); // European Euro
/// let x = CurrencySymbolAsStructString(String::from("ج.م")); // Egyptian Pound
/// let x = CurrencySymbolAsStructString(String::from("Rp")); // Indonesian Rupiah
/// ```
pub struct CurrencySymbolAsStructString(pub String);
