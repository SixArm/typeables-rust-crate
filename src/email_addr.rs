//! # Email Addr
//!
//! Email Addr is the email addr specification for one
//! delivery address, without the display name.
//! 
//! Example: 
//! 
//!   * "alice@example.com"
//! 
//! Example:
//! ```
//! # use ::typeables::email::*;
//! let x = EmailAddrAsStructStr("alice@example.com");
//! ```

pub struct EmailAddrAsStructStr(pub &'static str);
pub struct EmailAddrAsStructString(pub String);

pub type EmailAddrAsTypeStr = str;
pub type EmailAddrAsTypeString = String;

