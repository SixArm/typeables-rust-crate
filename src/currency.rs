//! # Currency
//!
//! Currency is for representing cash and equivalents.
//!
//!
//! ## Currency Name
//!
//! Examples:
//!
//! ```
//! # use typeables::currency::*;
//! let x = CurrencyNameAsStructStr("United States Dollar");
//! let x = CurrencyNameAsStructStr("Chinese Yuan");
//! let x = CurrencyNameAsStructStr("Indian Rupee");
//! let x = CurrencyNameAsStructStr("European Euro");
//! let x = CurrencyNameAsStructStr("Egyptian Pound");
//! let x = CurrencyNameAsStructStr("Indonesian Rupiah");
//! ```
//!
//! ## Currency Symbol
//!
//! Examples:
//! ```
//! # use typeables::currency::*;
//! let x = CurrencySymbolAsStructStr("$"); // United States Dollar
//! let x = CurrencySymbolAsStructStr("¥"); // Chinese Yuan
//! let x = CurrencySymbolAsStructStr("₹"); // Indian Rupee
//! let x = CurrencySymbolAsStructStr("€"); // European Euro
//! let x = CurrencySymbolAsStructStr("ج.م"); // Egyptian Pound
//! let x = CurrencySymbolAsStructStr("Rp"); // Indonesian Rupiah
//! ```

//// Currency Name

pub struct CurrencyNameAsStructStr(pub &'static str);
pub struct CurrencyNameAsStructString(pub String);

pub type CurrencyNameAsTypeStr = str;
pub type CurrencyNameAsTypeString = String;

//// Currency Symbol

pub struct CurrencySymbolAsStructStr(pub &'static str);
pub struct CurrencySymbolAsStructString(pub String);

pub type CurrencySymbolAsTypeStr = str;
pub type CurrencySymbolAsTypeString = String;

