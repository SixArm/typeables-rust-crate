//! # Open Location Code
//!
//! Open Location Code geolocation text.
//!
//! See: <https://github.com/google/open-location-code>
//!
//! Example:
//!
//!   * "6GCRPR6C+24" is the Parliament Buildings in Nairobi, Kenya.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::open_location_code::*;
//! let x = OpenLocationCodeAsStructStr("6GCRPR6C+24");
//! ```

pub struct OpenLocationCodeAsStructStr(pub &'static str);
pub struct OpenLocationCodeAsStructString(pub String);

pub type OpenLocationCodeAsTypeStr = str;
pub type OpenLocationCodeAsTypeString = String;
