//! Media type parameters

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: MediaTypeParameters_Vec_String = vec![
///     "charset=UTF-8".into(),
///     "boundary=alpha".into(),
/// ];
/// ```
#[allow(non_camel_case_types)] 
pub type MediaTypeParameters_Vec_String = Vec<super::media_type_parameter::MediaTypeParameter_String>;
