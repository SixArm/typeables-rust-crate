//! # Value Added Tax Identification Number 
//! 
//! Value Added Tax Identification Number is a.k.a. VATIN, VAT id.
//!
//! See: https://schema.org/vatID
//! 
//! See: https://wikipedia.org/wiki/VAT_identification_number
//! 
//! Example:
//! 
//!   * "01234567"
//!
//! Note that this type is a text code, not an integer.
//! 
//! Example:
//! ```
//! # use ::typeables::value_added_tax_identification_number::*;
//! let x = ValueAddedTaxIdentificationNumberAsStructStr("Alice Adams <alice@example.com>");
//! ```

pub struct ValueAddedTaxIdentificationNumberAsStructStr(pub &'static str);
pub struct ValueAddedTaxIdentificationNumberAsStructString(pub String);

pub type ValueAddedTaxIdentificationNumberAsTypeStr = str;
pub type ValueAddedTaxIdentificationNumberAsTypeString = String;
