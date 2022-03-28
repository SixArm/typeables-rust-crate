//! # Markdown Text
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::markdown_text::*;
//! let x = MarkdownTextAsStructStr("This is *bold* text");
//! ```

pub struct MarkdownTextAsStructStr(pub &'static str);
pub struct MarkdownTextAsStructString(pub String);

pub type MarkdownTextAsTypeStr = str;
pub type MarkdownTextAsTypeString = String;
