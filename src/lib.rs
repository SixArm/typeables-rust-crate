//! # Typeables: Rust crate of type aliases 
//! 
//! Typeables is a Rust crate of type aliases, intended to help improve source code self-documenting semantic type names. This helps with literate programming, domain driven design, and developer knowledge.
//! 
//! For example, suppose we want to distinguish between a noun string and a verb string:
//! 
//! ```rust
//! use typeables::*;
//! let y: &Noun_str = "book";
//! let x: &Verb_str = "read";
//! ```
//! 
//! # Purpose
//! 
//! The purpose of this crate is syntax sugar for better readabiliy.
//! 
//! The purpose of this library is not any kind type-based coding, such as data encapsulation, or parameter validation, or object oriented programming. If you want these kinds of aspects, we recommend looking at the crate `uom` (unit of measure) and the Rust book examples of the `newtype` pattern.
//! 
//! 
//! # Implementation
//! 
//! The type aliases are all for Rust primitives and standards such as `String`. 
//! 
//! The type aliases are zero-overhead because they are replaced at compile time. 

pub mod grammar {
    pub mod noun;
    pub mod verb;
}

pub use self::grammar::noun::Noun_str;
pub use self::grammar::noun::Noun_String;
pub use self::grammar::verb::Verb_str;
pub use self::grammar::verb::Verb_String;

pub mod media_type {
    pub mod media_type_parameter;
    pub mod media_type_parameters;
    pub mod media_type_suffix;
    pub mod media_type_supertype;
    pub mod media_type_text;
    pub mod media_type_tree;
}

pub use self::media_type::media_type_parameter::MediaTypeParameter_str;
pub use self::media_type::media_type_parameter::MediaTypeParameter_String;
pub use self::media_type::media_type_parameters::MediaTypeParameters_Vec_String;
pub use self::media_type::media_type_suffix::MediaTypeSuffix_str;
pub use self::media_type::media_type_suffix::MediaTypeSuffix_String;
pub use self::media_type::media_type_supertype::MediaTypeSupertype_str;
pub use self::media_type::media_type_supertype::MediaTypeSupertype_String;
pub use self::media_type::media_type_text::MediaTypeText_str;
pub use self::media_type::media_type_text::MediaTypeText_String;
pub use self::media_type::media_type_tree::MediaTypeTree_str;
pub use self::media_type::media_type_tree::MediaTypeTree_String;

