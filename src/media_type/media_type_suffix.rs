//! Media type suffix

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeSuffix_str = "+json";
/// let x: &MediaTypeSuffix_str = "+zip";
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSuffix_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeSuffix_String = "+json".into();
/// let x: MediaTypeSuffix_String = "+zip".into();
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeSuffix_String = String;
