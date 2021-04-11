//! Media type text

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeText_str = "text/plain";
/// let x: &MediaTypeText_str = "image/jpeg";
/// let x: &MediaTypeText_str = "audio/ogg";
/// let x: &MediaTypeText_str = "video/mpeg";
/// let x: &MediaTypeText_str = "application/json";
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeText_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeText_String = "text/plain".into();
/// let x: MediaTypeText_String = "image/jpeg".into();
/// let x: MediaTypeText_String = "audio/ogg".into();
/// let x: MediaTypeText_String = "video/mpeg".into();
/// let x: MediaTypeText_String = "application/json".into();
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeText_String = String;

