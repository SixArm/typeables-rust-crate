//! Media type supertype

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeSupertype_str = "text";        // e.g. "text/plain"
/// let x: &MediaTypeSupertype_str = "image";       // e.g. "image/jpeg"
/// let x: &MediaTypeSupertype_str = "audio";       // e.g. "audio/ogg"
/// let x: &MediaTypeSupertype_str = "video";       // e.g. "video/mpeg"
/// let x: &MediaTypeSupertype_str = "application"; // e.g. "application/json"
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSupertype_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeSupertype_String = "text".into();        // e.g. "text/plain"
/// let x: MediaTypeSupertype_String = "image".into();       // e.g. "image/jpeg"
/// let x: MediaTypeSupertype_String = "audio".into();       // e.g. "audio/ogg"
/// let x: MediaTypeSupertype_String = "video".into();       // e.g. "video/mpeg"
/// let x: MediaTypeSupertype_String = "application".into(); // e.g. "application/json"
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSupertype_String = String;
