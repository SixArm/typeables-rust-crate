//! # Noun
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

/// Noun as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::noun::*;
/// let x: &NounAsTypeStr = "Alice"; // person
/// let x: &NounAsTypeStr = "beach"; // place
/// let x: &NounAsTypeStr = "clock"; // thing
/// let x: &NounAsTypeStr = "dream"; // idea
/// let x: &NounAsTypeStr = "eagle"; // animal
/// let x: &NounAsTypeStr = "fruit"; // plant
/// ```
pub type NounAsTypeStr = str;

/// Noun as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::noun::*;
/// let x: NounAsTypeString = String::from("Alice"); // person
/// let x: NounAsTypeString = String::from("beach"); // place
/// let x: NounAsTypeString = String::from("clock"); // thing
/// let x: NounAsTypeString = String::from("dream"); // idea
/// let x: NounAsTypeString = String::from("eagle"); // animal
/// let x: NounAsTypeString = String::from("fruit"); // plant
/// ```
pub type NounAsTypeString = String;

/// Noun as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::noun::*;
/// let x = NounAsStructStr("Alice"); // person
/// let x = NounAsStructStr("beach"); // place
/// let x = NounAsStructStr("clock"); // thing
/// let x = NounAsStructStr("dream"); // idea
/// let x = NounAsStructStr("eagle"); // animal
/// let x = NounAsStructStr("fruit"); // plant
/// ```
pub struct NounAsStructStr(pub &'static str);

/// Noun as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::noun::*;
/// let x = NounAsStructString(String::from("Alice")); // person
/// let x = NounAsStructString(String::from("beach")); // place
/// let x = NounAsStructString(String::from("clock")); // thing
/// let x = NounAsStructString(String::from("dream")); // idea
/// let x = NounAsStructString(String::from("eagle")); // animal
/// let x = NounAsStructString(String::from("fruit")); // plant
/// ```
pub struct NounAsStructString(pub String);
