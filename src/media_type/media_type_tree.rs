/// Media type tree
///
/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeTree_str = "x."; // unregistered
/// let x: &MediaTypeTree_str = "vnd."; // vendor
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeTree_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeTree_String = "x.".into(); // unregistered
/// let x: MediaTypeTree_String = "vnd.".into(); // vendor
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeTree_String = String;
