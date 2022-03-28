//! # Media type
//!
//! Example: The media type code "text/plain" means that content (e.g. a
//! software file) is the media type supertype "text" (e.g. not an image, not a
//! movie, etc.) and media type subtype "plain" (e.g. not hypertext, not
//! markdown, etc.).
//!
//! Examples of common media type codes:
//!
//!   * text/plain is a typical note file
//!
//!   * image/jpeg is a typical photo file
//!
//!   * audio/ogg is a typical sound file
//!
//!   * video/mpeg is a typical movie file
//!
//!   * application/json is a typical data file
//!
//!
//! ## Media Type Code
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeCodeAsStructStr("text/plain");
//! let x = MediaTypeCodeAsStructStr("image/jpeg");
//! let x = MediaTypeCodeAsStructStr("audio/ogg");
//! let x = MediaTypeCodeAsStructStr("video/mpeg");
//! let x = MediaTypeCodeAsStructStr("application/json");
//! ```
//!
//!
//! ## Media Type Supertype
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeSupertypeAsStructStr("text");        // e.g. "text/plain"
//! let x = MediaTypeSupertypeAsStructStr("image");       // e.g. "image/jpeg"
//! let x = MediaTypeSupertypeAsStructStr("audio");       // e.g. "audio/ogg"
//! let x = MediaTypeSupertypeAsStructStr("video");       // e.g. "video/mpeg"
//! let x = MediaTypeSupertypeAsStructStr("application"); // e.g. "application/json"
//! ```
//!
//!
//! ## Media Type Subtype
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeSubtypeAsStructStr("plain"); // e.g. "text/plain"
//! let x = MediaTypeSubtypeAsStructStr("jpeg");  // e.g. "image/jpeg"
//! let x = MediaTypeSubtypeAsStructStr("ogg");   // e.g. "audio/ogg"
//! let x = MediaTypeSubtypeAsStructStr("mpeg");  // e.g. "video/mpeg"
//! let x = MediaTypeSubtypeAsStructStr("json");  // e.g. "application/json"
//! ```
//!
//!
//! ## Media Type Suffix
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeSuffixAsStructStr("+json");
//! let x = MediaTypeSuffixAsStructStr("+zip");
//! ```
//!
//!
//! ## Media Type Parameter
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeParameterAsStructStr("charset=UTF-8");
//! let x = MediaTypeParameterAsStructStr("charset=UTF-16");
//! let x = MediaTypeParameterAsStructStr("charset=ASCII");
//! let x = MediaTypeParameterAsStructStr("boundary=alpha");
//! let x = MediaTypeParameterAsStructStr("boundary=bravo");
//! let x = MediaTypeParameterAsStructStr("boundary=charlie");
//! ```
//!
//!
//! Media Type Tree
//!
//! Examples:
//!
//! ```rust
//! # use typeables::media_type::*;
//! let x = MediaTypeTreeAsStructStr("x."); // x means unregistered
//! let x = MediaTypeTreeAsStructStr("vnd."); // vnd means vendor
//! ```

//// Media Type Code

pub struct MediaTypeCodeAsStructStr(pub &'static str);
pub struct MediaTypeCodeAsStructString(pub String);

pub type MediaTypeCodeAsTypeStr = str;
pub type MediaTypeCodeAsTypeString = String;

//// Media Type Supertype

pub struct MediaTypeSupertypeAsStructStr(pub &'static str);
pub struct MediaTypeSupertypeAsStructString(pub String);

pub type MediaTypeSupertypeAsTypeStr = str;
pub type MediaTypeSupertypeAsTypeString = String;

//// Media Type Subtype

pub struct MediaTypeSubtypeAsStructStr(pub &'static str);
pub struct MediaTypeSubtypeAsStructString(pub String);

pub type MediaTypeSubtypeAsTypeStr = str;
pub type MediaTypeSubtypeAsTypeString = String;

//// Media Type Suffix

pub struct MediaTypeSuffixAsStructStr(pub &'static str);
pub struct MediaTypeSuffixAsStructString(pub String);

pub type MediaTypeSuffixAsTypeStr = str;
pub type MediaTypeSuffixAsTypeString = String;

//// Media Type Parameter

pub struct MediaTypeParameterAsStructStr(pub &'static str);
pub struct MediaTypeParameterAsStructString(pub String);

pub type MediaTypeParameterAsTypeStr = str;
pub type MediaTypeParameterAsTypeString = String;

//// Media Type Tree

pub struct MediaTypeTreeAsStructStr(pub &'static str);
pub struct MediaTypeTreeAsStructString(pub String);

pub type MediaTypeTreeAsTypeStr = str;
pub type MediaTypeTreeAsTypeString = String;
