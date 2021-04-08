/// Media type tree
///
/// # Examples
///
/// ```
/// let x: MediaTypeTree = "x.".into(); // unregistered
/// let x: MediaTypeTree = "vnd.".into(); // vendor
/// ```

pub type MediaTypeTree = String;
