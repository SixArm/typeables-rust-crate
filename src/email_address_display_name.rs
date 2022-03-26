//! # Email Address Display Name
//!
//! Email Address Display Name is the email addr specification for one delivery
//! address, without a display name.
//! 
//! Example: 
//! 
//!   * The "Alice Adams" in "Alice Adams <alice@example.com>".
//! 
//! Example:
//! ```
//! # use ::typeables::email::*;
//! let x = EmailAddressDisplayNameAsStructStr("Alice Adams");
//! ```

pub struct EmailAddressDisplayNameAsStructStr(pub &'static str);
pub struct EmailAddressDisplayNameAsStructString(pub String);

pub type EmailAddressDisplayNameAsTypeStr = str;
pub type EmailAddressDisplayNameAsTypeString = String;

