//! # Email Address
//!
//! Email Address is the email address for one
//! delivery address, with an optional display name.
//! 
//! Example: 
//! 
//!   * "Alice Adams <alice@example.com>"
//! 
//! Example:
//! ```
//! # use ::typeables::email::*;
//! let x = EmailAddressAsStructStr("alice@example.com");
//! ```

pub struct EmailAddressAsStructStr(pub &'static str);
pub struct EmailAddressAsStructString(pub String);

pub type EmailAddressAsTypeStr = str;
pub type EmailAddressAsTypeString = String;

