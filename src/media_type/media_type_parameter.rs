//! Media type parameter

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &MediaTypeParameter_str = "charset=UTF-8";
/// let x: &MediaTypeParameter_str = "charset=UTF-16";
/// let x: &MediaTypeParameter_str = "charset=ASCII";
/// let x: &MediaTypeParameter_str = "boundary=alpha";
/// let x: &MediaTypeParameter_str = "boundary=bravo";
/// let x: &MediaTypeParameter_str = "boundary=charlie";
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeParameter_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeParameter_String = "charset=UTF-8".into();
/// let x: MediaTypeParameter_String = "charset=UTF-16".into();
/// let x: MediaTypeParameter_String = "charset=ASCII".into();
/// let x: MediaTypeParameter_String = "boundary=alpha".into();
/// let x: MediaTypeParameter_String = "boundary=bravo".into();
/// let x: MediaTypeParameter_String = "boundary=charlie".into();
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeParameter_String = String;
