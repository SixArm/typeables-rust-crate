//! # Media type
//!
//! Example: "text/plain" means that some media (e.g. a disk file) 
//! is "text" (e.g. not an image) and plain (e.g. not HTML).

/// Media type text as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeTextAsTypeStr = "text/plain";
/// let x: &MediaTypeTextAsTypeStr = "image/jpeg";
/// let x: &MediaTypeTextAsTypeStr = "audio/ogg";
/// let x: &MediaTypeTextAsTypeStr = "video/mpeg";
/// let x: &MediaTypeTextAsTypeStr = "application/json";
/// ```
pub type MediaTypeTextAsTypeStr = str;

/// Media type text as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeTextAsTypeString = String::from("text/plain");
/// let x: MediaTypeTextAsTypeString = String::from("image/jpeg");
/// let x: MediaTypeTextAsTypeString = String::from("audio/ogg");
/// let x: MediaTypeTextAsTypeString = String::from("video/mpeg");
/// let x: MediaTypeTextAsTypeString = String::from("application/json");
/// ```
pub type MediaTypeTextAsTypeString = String;

/// Media type text as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeTextAsStructStr("text/plain");
/// let x = MediaTypeTextAsStructStr("image/jpeg");
/// let x = MediaTypeTextAsStructStr("audio/ogg");
/// let x = MediaTypeTextAsStructStr("video/mpeg");
/// let x = MediaTypeTextAsStructStr("application/json");
/// ```
pub struct MediaTypeTextAsStructStr(pub &'static str);

/// Media type text as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeTextAsStructString(String::from("text/plain"));
/// let x = MediaTypeTextAsStructString(String::from("image/jpeg"));
/// let x = MediaTypeTextAsStructString(String::from("audio/ogg"));
/// let x = MediaTypeTextAsStructString(String::from("video/mpeg"));
/// let x = MediaTypeTextAsStructString(String::from("application/json"));
/// ```
pub struct MediaTypeTextAsStructString(pub String);

/// Media type supertype as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSupertypeAsTypeStr = "text";        // e.g. "text/plain"
/// let x: &MediaTypeSupertypeAsTypeStr = "image";       // e.g. "image/jpeg"
/// let x: &MediaTypeSupertypeAsTypeStr = "audio";       // e.g. "audio/ogg"
/// let x: &MediaTypeSupertypeAsTypeStr = "video";       // e.g. "video/mpeg"
/// let x: &MediaTypeSupertypeAsTypeStr = "application"; // e.g. "application/json"
/// ```
pub type MediaTypeSupertypeAsTypeStr = str;

/// Media type supertype as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSupertypeAsTypeString = String::from("text");        // e.g. "text/plain"
/// let x: MediaTypeSupertypeAsTypeString = String::from("image");       // e.g. "image/jpeg"
/// let x: MediaTypeSupertypeAsTypeString = String::from("audio");       // e.g. "audio/ogg"
/// let x: MediaTypeSupertypeAsTypeString = String::from("video");       // e.g. "video/mpeg"
/// let x: MediaTypeSupertypeAsTypeString = String::from("application"); // e.g. "application/json"
/// ```
pub type MediaTypeSupertypeAsTypeString = String;

/// Media type supertype as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSupertypeAsStructStr("text");        // e.g. "text/plain"
/// let x = MediaTypeSupertypeAsStructStr("image");       // e.g. "image/jpeg"
/// let x = MediaTypeSupertypeAsStructStr("audio");       // e.g. "audio/ogg"
/// let x = MediaTypeSupertypeAsStructStr("video");       // e.g. "video/mpeg"
/// let x = MediaTypeSupertypeAsStructStr("application"); // e.g. "application/json"
/// ```
pub struct MediaTypeSupertypeAsStructStr(pub &'static str);

/// Media type supertype as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSupertypeAsStructString(String::from("text"));        // e.g. "text/plain"
/// let x = MediaTypeSupertypeAsStructString(String::from("image"));       // e.g. "image/jpeg"
/// let x = MediaTypeSupertypeAsStructString(String::from("audio"));       // e.g. "audio/ogg"
/// let x = MediaTypeSupertypeAsStructString(String::from("video"));       // e.g. "video/mpeg"
/// let x = MediaTypeSupertypeAsStructString(String::from("application")); // e.g. "application/json"
/// ```
pub struct MediaTypeSupertypeAsStructString(pub String);

/// Media type subtype as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSubtypeAsTypeStr = "plain"; // e.g. "text/plain"
/// let x: &MediaTypeSubtypeAsTypeStr = "jpeg";  // e.g. "image/jpeg"
/// let x: &MediaTypeSubtypeAsTypeStr = "ogg";   // e.g. "audio/ogg"
/// let x: &MediaTypeSubtypeAsTypeStr = "mpeg";  // e.g. "video/mpeg"
/// let x: &MediaTypeSubtypeAsTypeStr = "json";  // e.g. "application/json"
/// ```
pub type MediaTypeSubtypeAsTypeStr = str;

/// Media type subtype as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSubtypeAsTypeString = String::from("plain"); // e.g. "text/plain"
/// let x: MediaTypeSubtypeAsTypeString = String::from("jpeg");  // e.g. "image/jpeg"
/// let x: MediaTypeSubtypeAsTypeString = String::from("ogg");   // e.g. "audio/ogg"
/// let x: MediaTypeSubtypeAsTypeString = String::from("mpeg");  // e.g. "video/mpeg"
/// let x: MediaTypeSubtypeAsTypeString = String::from("json");  // e.g. "application/json"
/// ```
pub type MediaTypeSubtypeAsTypeString = String;

/// Media type subtype as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSubtypeAsStructStr("plain"); // e.g. "text/plain"
/// let x = MediaTypeSubtypeAsStructStr("jpeg");  // e.g. "image/jpeg"
/// let x = MediaTypeSubtypeAsStructStr("ogg");   // e.g. "audio/ogg"
/// let x = MediaTypeSubtypeAsStructStr("mpeg");  // e.g. "video/mpeg"
/// let x = MediaTypeSubtypeAsStructStr("json");  // e.g. "application/json"
/// ```
pub struct MediaTypeSubtypeAsStructStr(pub &'static str);

/// Media type subtype as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSubtypeAsStructString(String::from("plain")); // e.g. "text/plain"
/// let x = MediaTypeSubtypeAsStructString(String::from("jpeg"));  // e.g. "image/jpeg"
/// let x = MediaTypeSubtypeAsStructString(String::from("ogg"));   // e.g. "audio/ogg"
/// let x = MediaTypeSubtypeAsStructString(String::from("mpeg"));  // e.g. "video/mpeg"
/// let x = MediaTypeSubtypeAsStructString(String::from("json"));  // e.g. "application/json"
/// ```
pub struct MediaTypeSubtypeAsStructString(pub String);

/// Media type suffix as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSuffixAsTypeStr = "+json";
/// let x: &MediaTypeSuffixAsTypeStr = "+zip";
/// ```
pub type MediaTypeSuffixAsTypeStr = str;

/// Media type suffix as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSuffixAsTypeString = String::from("+json");
/// let x: MediaTypeSuffixAsTypeString = String::from("+zip");
/// ```
pub type MediaTypeSuffixAsTypeString = String;

/// Media type suffix as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSuffixAsStructStr("+json");
/// let x = MediaTypeSuffixAsStructStr("+zip");
/// ```
pub struct MediaTypeSuffixAsStructStr(pub &'static str);

/// Media type suffix as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeSuffixAsStructString(String::from("+json"));
/// let x = MediaTypeSuffixAsStructString(String::from("+zip"));
/// ```
pub struct MediaTypeSuffixAsStructString(pub String);

/// Media type parameter as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeParameterAsTypeStr = "charset=UTF-8";
/// let x: &MediaTypeParameterAsTypeStr = "charset=UTF-16";
/// let x: &MediaTypeParameterAsTypeStr = "charset=ASCII";
/// let x: &MediaTypeParameterAsTypeStr = "boundary=alpha";
/// let x: &MediaTypeParameterAsTypeStr = "boundary=bravo";
/// let x: &MediaTypeParameterAsTypeStr = "boundary=charlie";
/// ```
pub type MediaTypeParameterAsTypeStr = str;

/// Media type parameter as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeParameterAsTypeString = String::from("charset=UTF-8");
/// let x: MediaTypeParameterAsTypeString = String::from("charset=UTF-16");
/// let x: MediaTypeParameterAsTypeString = String::from("charset=ASCII");
/// let x: MediaTypeParameterAsTypeString = String::from("boundary=alpha");
/// let x: MediaTypeParameterAsTypeString = String::from("boundary=bravo");
/// let x: MediaTypeParameterAsTypeString = String::from("boundary=charlie");
/// ```
pub type MediaTypeParameterAsTypeString = String;

/// Media type parameter as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeParameterAsStructStr("charset=UTF-8");
/// let x = MediaTypeParameterAsStructStr("charset=UTF-16");
/// let x = MediaTypeParameterAsStructStr("charset=ASCII");
/// let x = MediaTypeParameterAsStructStr("boundary=alpha");
/// let x = MediaTypeParameterAsStructStr("boundary=bravo");
/// let x = MediaTypeParameterAsStructStr("boundary=charlie");
/// ```
pub struct MediaTypeParameterAsStructStr(pub &'static str);

/// Media type parameter as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeParameterAsStructString(String::from("charset=UTF-8"));
/// let x = MediaTypeParameterAsStructString(String::from("charset=UTF-16"));
/// let x = MediaTypeParameterAsStructString(String::from("charset=ASCII"));
/// let x = MediaTypeParameterAsStructString(String::from("boundary=alpha"));
/// let x = MediaTypeParameterAsStructString(String::from("boundary=bravo"));
/// let x = MediaTypeParameterAsStructString(String::from("boundary=charlie"));
/// ```
pub struct MediaTypeParameterAsStructString(pub String);

/// Media type tree as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeTreeAsTypeStr = "x."; // unregistered
/// let x: &MediaTypeTreeAsTypeStr = "vnd."; // vendor
/// ```
pub type MediaTypeTreeAsTypeStr = str;

/// Media type tree as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeTreeAsTypeString = String::from("x."); // unregistered
/// let x: MediaTypeTreeAsTypeString = String::from("vnd."); // vendor
/// ```
pub type MediaTypeTreeAsTypeString = String;

/// Media type tree as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeTreeAsStructStr("x."); // unregistered
/// let x = MediaTypeTreeAsStructStr("vnd."); // vendor
/// ```
pub struct MediaTypeTreeAsStructStr(pub &'static str);

/// Media type tree as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x = MediaTypeTreeAsStructString(String::from("x.")); // unregistered
/// let x = MediaTypeTreeAsStructString(String::from("vnd.")); // vendor
/// ```
pub struct MediaTypeTreeAsStructString(pub String);
