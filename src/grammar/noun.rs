//! Noun
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

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: &Noun_str = "Alice"; // person
/// let x: &Noun_str = "beach"; // place
/// let x: &Noun_str = "clock"; // thing
/// let x: &Noun_str = "dream"; // idea
/// let x: &Noun_str = "eagle"; // animal
/// let x: &Noun_str = "fruit"; // plant
/// ```
#[allow(non_camel_case_types)] 
pub type Noun_str = str;

/// Examples:
/// ```
/// # use ::typeables::*;
/// let x: Noun_String = "Alice".into(); // person
/// let x: Noun_String = "beach".into(); // place
/// let x: Noun_String = "clock".into(); // thing
/// let x: Noun_String = "dream".into(); // idea
/// let x: Noun_String = "eagle".into(); // animal
/// let x: Noun_String = "fruit".into(); // plant
/// ```
#[allow(non_camel_case_types)] 
pub type Noun_String = String;
