//! # Global Location Number
//!
//! See: <https://schema.org/globalLocationNumber>
//! 
//! See: <https://en.wikipedia.org/wiki/Global_Location_Number>
//!
//! A Global Location Number (GLN) is a 13-digit number used to identify parties
//! and physical locations. A GLN is sometimes also referred to as International
//! Location Number (ILN).
//! 
//! Example:
//! ```
//! # use ::typeables::global_location_number::*;
//! let x = GlobalLocationNumberAsStructStr("1234567890123");
//! ```

pub struct GlobalLocationNumberAsStructStr(pub &'static str);
pub struct GlobalLocationNumberAsStructString(pub String);

pub type GlobalLocationNumberAsTypeStr = str;
pub type GlobalLocationNumberAsTypeString = String;
