//! # Email Address Addr
//!
//! Email Address Addr is the email address "addr specification", which means
//! one email delivery string with an "@" sign, without any display name.
//!
//! Example:
//! ```
//! # use ::typeables::email_address_addr::*;
//! let x = EmailAddressAddrAsStructStr("alice@example.com");
//! ```
//!
//! Compare:
//!
//!   * "Alice Adams <alice@example.com>" is a `EmailAddress`
//!
//!   * "Alice Adams" is a `EmailAddressName`
//!
//!   * "alice@example.com" is a `EmailAddressAddr`

pub struct EmailAddressAddrAsStructStr(pub &'static str);
pub struct EmailAddressAddrAsStructString(pub String);

pub type EmailAddressAddrAsTypeStr = str;
pub type EmailAddressAddrAsTypeString = String;

