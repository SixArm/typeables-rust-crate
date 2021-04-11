//! Media type subtype

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeSubtype_str = "plain"; // e.g. "text/plain"
/// let x: &MediaTypeSubtype_str = "jpeg";  // e.g. "image/jpeg"
/// let x: &MediaTypeSubtype_str = "ogg";   // e.g. "audio/ogg"
/// let x: &MediaTypeSubtype_str = "mpeg";  // e.g. "video/mpeg"
/// let x: &MediaTypeSubtype_str = "json";  // e.g. "application/json"
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSubtype_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeSubtype_String = "plain".into(); // e.g. "text/plain"
/// let x: MediaTypeSubtype_String = "jpeg".into();  // e.g. "image/jpeg"
/// let x: MediaTypeSubtype_String = "ogg".into();   // e.g. "audio/ogg"
/// let x: MediaTypeSubtype_String = "mpeg".into();  // e.g. "video/mpeg"
/// let x: MediaTypeSubtype_String = "json".into();  // e.g. "application/json"
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSubtype_String = String;
