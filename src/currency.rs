//! Currency

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CurrencyNameAsStr(str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CurrencyNameAsString(String);

/// Currency name as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: &CurrencyNameStr = "United States Dollar";
/// let x: &CurrencyNameStr = "Chinese Yuan";
/// let x: &CurrencyNameStr = "Indian Rupee";
/// let x: &CurrencyNameStr = "European Euro";
/// let x: &CurrencyNameStr = "Egyptian Pound";
/// let x: &CurrencyNameStr = "Indonesian Rupiah";
/// ```
pub type CurrencyNameStr = str;

/// Currency name as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: CurrencyNameString = "United States Dollar".into();
/// let x: CurrencyNameString = "Chinese Yuan".into();
/// let x: CurrencyNameString = "Indian Rupee".into();
/// let x: CurrencyNameString = "European Euro".into();
/// let x: CurrencyNameString = "Egyptian Pound".into();
/// let x: CurrencyNameString = "Indonesian Rupiah".into();
/// ```
pub type CurrencyNameString = String;

/// Currency symbol as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: &CurrencySymbolStr = "$"; // United States Dollar
/// let x: &CurrencySymbolStr = "¥"; // Chinese Yuan
/// let x: &CurrencySymbolStr = "₹"; // Indian Rupee
/// let x: &CurrencySymbolStr = "€"; // European Euro
/// let x: &CurrencySymbolStr = "ج.م"; // Egyptian Pound
/// let x: &CurrencySymbolStr = "Rp"; // Indonesian Rupiah
/// ```
pub type CurrencySymbolStr = str;

/// Currency symbol as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::currency::*;
/// let x: CurrencySymbolString = "$".into(); // United States Dollar
/// let x: CurrencySymbolString = "¥".into(); // Chinese Yuan
/// let x: CurrencySymbolString = "₹".into(); // Indian Rupee
/// let x: CurrencySymbolString = "€".into(); // European Euro
/// let x: CurrencySymbolString = "ج.م".into(); // Egyptian Pound
/// let x: CurrencySymbolString = "Rp".into(); // Indonesian Rupiah
/// ```
pub type CurrencySymbolString = String;

