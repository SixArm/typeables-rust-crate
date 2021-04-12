//! Media type
//!
//! Example: "text/plain" means that some media (e.g. a disk file) 
//! is "text" (e.g. not an image) and plain (e.g. not HTML).

/// Media type text as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeTextStr = "text/plain";
/// let x: &MediaTypeTextStr = "image/jpeg";
/// let x: &MediaTypeTextStr = "audio/ogg";
/// let x: &MediaTypeTextStr = "video/mpeg";
/// let x: &MediaTypeTextStr = "application/json";
/// ```
pub type MediaTypeTextStr = str;

/// Media type text as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeTextString = "text/plain".into();
/// let x: MediaTypeTextString = "image/jpeg".into();
/// let x: MediaTypeTextString = "audio/ogg".into();
/// let x: MediaTypeTextString = "video/mpeg".into();
/// let x: MediaTypeTextString = "application/json".into();
/// ```
pub type MediaTypeTextString = String;


/// Media type supertype as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSupertypeStr = "text";        // e.g. "text/plain"
/// let x: &MediaTypeSupertypeStr = "image";       // e.g. "image/jpeg"
/// let x: &MediaTypeSupertypeStr = "audio";       // e.g. "audio/ogg"
/// let x: &MediaTypeSupertypeStr = "video";       // e.g. "video/mpeg"
/// let x: &MediaTypeSupertypeStr = "application"; // e.g. "application/json"
/// ```
pub type MediaTypeSupertypeStr = str;

/// Media type supertype as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSupertypeString = "text".into();        // e.g. "text/plain"
/// let x: MediaTypeSupertypeString = "image".into();       // e.g. "image/jpeg"
/// let x: MediaTypeSupertypeString = "audio".into();       // e.g. "audio/ogg"
/// let x: MediaTypeSupertypeString = "video".into();       // e.g. "video/mpeg"
/// let x: MediaTypeSupertypeString = "application".into(); // e.g. "application/json"
/// ```
pub type MediaTypeSupertypeString = String;

/// Media type subtype as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSubtypeStr = "plain"; // e.g. "text/plain"
/// let x: &MediaTypeSubtypeStr = "jpeg";  // e.g. "image/jpeg"
/// let x: &MediaTypeSubtypeStr = "ogg";   // e.g. "audio/ogg"
/// let x: &MediaTypeSubtypeStr = "mpeg";  // e.g. "video/mpeg"
/// let x: &MediaTypeSubtypeStr = "json";  // e.g. "application/json"
/// ```
pub type MediaTypeSubtypeStr = str;

/// Media type subtype as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSubtypeString = "plain".into(); // e.g. "text/plain"
/// let x: MediaTypeSubtypeString = "jpeg".into();  // e.g. "image/jpeg"
/// let x: MediaTypeSubtypeString = "ogg".into();   // e.g. "audio/ogg"
/// let x: MediaTypeSubtypeString = "mpeg".into();  // e.g. "video/mpeg"
/// let x: MediaTypeSubtypeString = "json".into();  // e.g. "application/json"
/// ```
pub type MediaTypeSubtypeString = String;

/// Media type suffix as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeSuffixStr = "+json";
/// let x: &MediaTypeSuffixStr = "+zip";
/// ```
pub type MediaTypeSuffixStr = str;

/// Media type suffix as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeSuffixString = "+json".into();
/// let x: MediaTypeSuffixString = "+zip".into();
/// ```
pub type MediaTypeSuffixString = String;

/// Media type parameter as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeParameterStr = "charset=UTF-8";
/// let x: &MediaTypeParameterStr = "charset=UTF-16";
/// let x: &MediaTypeParameterStr = "charset=ASCII";
/// let x: &MediaTypeParameterStr = "boundary=alpha";
/// let x: &MediaTypeParameterStr = "boundary=bravo";
/// let x: &MediaTypeParameterStr = "boundary=charlie";
/// ```
pub type MediaTypeParameterStr = str;

/// Media type parameter as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeParameterString = "charset=UTF-8".into();
/// let x: MediaTypeParameterString = "charset=UTF-16".into();
/// let x: MediaTypeParameterString = "charset=ASCII".into();
/// let x: MediaTypeParameterString = "boundary=alpha".into();
/// let x: MediaTypeParameterString = "boundary=bravo".into();
/// let x: MediaTypeParameterString = "boundary=charlie".into();
/// ```
pub type MediaTypeParameterString = String;

/// Media type parameters as `Vec<String>`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeParametersVecString = vec![
///     "charset=UTF-8".into(),
///     "boundary=alpha".into(),
/// ];
/// ```
pub type MediaTypeParametersVecString = Vec<MediaTypeParameterString>;

/// Media type tree as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: &MediaTypeTreeStr = "x."; // unregistered
/// let x: &MediaTypeTreeStr = "vnd."; // vendor
/// ```
pub type MediaTypeTreeStr = str;

/// Media type tree as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::media_type::*;
/// let x: MediaTypeTreeString = "x.".into(); // unregistered
/// let x: MediaTypeTreeString = "vnd.".into(); // vendor
/// ```
pub type MediaTypeTreeString = String;
