//! # Legal Entity Identifier Code
//!
//! See: https://schema.org/leiCode
//! 
//! See: https://wikipedia.org/wiki/Legal_Entity_Identifier
//!
//! Example:
//! 
//!   * "01234567890123456789"
//! 
//! Example:
//! ```
//! # use ::typeables::legal_entity_identifier_code::*;
//! let x = LegalEntityIdentifierCodeAsStructStr("01234567890123456789");
//! ```

pub struct LegalEntityIdentifierCodeAsStructStr(pub &'static str);
pub struct LegalEntityIdentifierCodeAsStructString(pub String);

pub type LegalEntityIdentifierCodeAsTypeStr = str;
pub type LegalEntityIdentifierCodeAsTypeString = String;
