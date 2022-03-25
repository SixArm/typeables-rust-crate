//! # Noun
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::noun::*;
//! let x = NounAsStructStr("Alice"); // person
//! let x = NounAsStructStr("beach"); // place
//! let x = NounAsStructStr("clock"); // thing
//! let x = NounAsStructStr("dream"); // idea
//! let x = NounAsStructStr("eagle"); // animal
//! let x = NounAsStructStr("fruit"); // plant
//! ```
//! 
//! https://wikipedia.org/wiki/Noun
//!
//! A noun is a word that functions as the name of a specific object or set of
//! objects, such as living creatures, places, actions, qualities, states of
//! existence, or ideas. However, noun is not a semantic category, so it cannot
//! be characterized in terms of its meaning. Thus, actions and states of
//! existence can also be expressed by verbs, qualities by adjectives, and
//! places by adverbs. Linguistically, a noun is a member of a large, open part
//! of speech whose members can occur as the main word in the subject of a
//! clause, the object of a verb, or the object of a preposition.
//!
//! Compare: Verb

//// Noun

pub struct NounAsStructStr(pub &'static str);
pub struct NounAsStructString(pub String);

pub type NounAsTypeStr = str;
pub type NounAsTypeString = String;
