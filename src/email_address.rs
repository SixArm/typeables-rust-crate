//! # Email Address
//!
//! Email Address is for one email delivery address, which has an optional email
//! address name, and a required email address addr.
//!
//! Example:
//! ```
//! # use typeables::email_address::*;
//! let x = EmailAddressAsStructStr("Alice Adams <alice@example.com>");
//! ```
//!
//! Compare:
//!
//!   * "Alice Adams <alice@example.com>" is a `EmailAddress`
//!
//!   * "Alice Adams" is a `EmailAddressName`
//!
//!   * "alice@example.com" is a `EmailAddressAddr`

pub struct EmailAddressAsStructStr(pub &'static str);
pub struct EmailAddressAsStructString(pub String);

pub type EmailAddressAsTypeStr = str;
pub type EmailAddressAsTypeString = String;
