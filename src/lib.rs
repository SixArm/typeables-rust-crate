//! # Typeables: Rust crate of type aliases 
//! 
//! Typeables is a Rust crate of type aliases, intended to help improve source code self-documenting semantic type names. This helps with literate programming, domain driven design, and developer knowledge.
//! 
//! For example, suppose we want to distinguish between a noun string and a verb string:
//! 
//! ```rust
//! use typeables::*;
//! let x: Verb = "read";
//! let y: Noun = "book";
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
//! 
//! 
//! # Types
//! 
//! Grammar:
//! 
//! * [`Noun`]
//! * [`Verb`]
//! 
//! Media Type:
//! 
//! * [`MediaTypeSupertype`]
//! * [`MediaTypeText`]
//! * [`MediaTypeTree`]

pub use self::grammar::noun::Noun;
pub use self::grammar::verb::Verb;
pub use self::media_type::media_type_supertype::MediaTypeSupertype;
pub use self::media_type::media_type_text::MediaTypeText;
pub use self::media_type::media_type_tree::MediaTypeTree;

pub mod grammar {
    pub mod noun;
    pub mod verb;
}

pub mod media_type {
    pub mod media_type_supertype;
    pub mod media_type_text;
    pub mod media_type_tree;
}
