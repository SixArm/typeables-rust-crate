//! Media type supertype

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeSupertype_str = "text";
/// let x: &MediaTypeSupertype_str = "image";
/// let x: &MediaTypeSupertype_str = "audio";
/// let x: &MediaTypeSupertype_str = "video";
/// let x: &MediaTypeSupertype_str = "application";
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSupertype_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeSupertype_String = "text".into();
/// let x: MediaTypeSupertype_String = "image".into();
/// let x: MediaTypeSupertype_String = "audio".into();
/// let x: MediaTypeSupertype_String = "video".into();
/// let x: MediaTypeSupertype_String = "application".into();
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSupertype_String = String;
