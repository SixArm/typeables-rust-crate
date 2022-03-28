//! # Email Address Name
//!
//! Email Address Name is the email address specification for one display name.
//!
//! Example:
//! ```
//! # use typeables::email_address_name::*;
//! let x = EmailAddressNameAsStructStr("Alice Adams");
//! ```
//!
//! Compare:
//!
//!   * "Alice Adams <alice@example.com>" is a `EmailAddress`
//!
//!   * "Alice Adams" is a `EmailAddressName`
//!
//!   * "alice@example.com" is a `EmailAddressAddr`

pub struct EmailAddressNameAsStructStr(pub &'static str);
pub struct EmailAddressNameAsStructString(pub String);

pub type EmailAddressNameAsTypeStr = str;
pub type EmailAddressNameAsTypeString = String;

