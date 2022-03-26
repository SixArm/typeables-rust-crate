//! # Email Addr
//!
//! Email Addr is the email addr specification for one delivery address, without
//! a display name.
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
//!
//! Comparison: if you want to handle the delivery address, and an optional
//! display name, then see `EmailAddress`.

pub struct EmailAddrAsStructStr(pub &'static str);
pub struct EmailAddrAsStructString(pub String);

pub type EmailAddrAsTypeStr = str;
pub type EmailAddrAsTypeString = String;

