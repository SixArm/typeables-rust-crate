//! # Grammar
//!
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
//! # Verb
//!
//! https://wikipedia.org/wiki/Verb
//!
//! A verb, from the Latin verbum meaning word, is a word (part of speech) that
//! in syntax conveys an action (bring, read, walk, run, learn), an occurrence
//! (happen, become), or a state of being (be, exist, stand). In the usual
//! description of English, the basic form, with or without the particle to, is
//! the infinitive. In many languages, verbs are inflected (modified in form) to
//! encode tense, aspect, mood, and voice. A verb may also agree with the
//! person, gender or number of some of its arguments, such as its subject, or
//! object. Verbs have tenses: present, to indicate that an action is being
//! carried out; past, to indicate that an action has been done; future, to
//! indicate that an action will be done.

/// Noun as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
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
/// # use ::typeables::grammar::*;
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
/// # use ::typeables::grammar::*;
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
/// # use ::typeables::grammar::*;
/// let x = NounAsStructString(String::from("Alice")); // person
/// let x = NounAsStructString(String::from("beach")); // place
/// let x = NounAsStructString(String::from("clock")); // thing
/// let x = NounAsStructString(String::from("dream")); // idea
/// let x = NounAsStructString(String::from("eagle")); // animal
/// let x = NounAsStructString(String::from("fruit")); // plant
/// ```
pub struct NounAsStructString(pub String);

/// Verb as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: &VerbAsTypeStr = "like"; // action transitive verb e.g. "I like you."
/// let x: &VerbAsTypeStr = "walk"; // action intransitive verb e.g. "I walk."
/// let x: &VerbAsTypeStr = "feel"; // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x: &VerbAsTypeStr = "seem"; // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x: &VerbAsTypeStr = "will"; // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x: &VerbAsTypeStr = "must"; // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub type VerbAsTypeStr = str;

/// Verb as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: VerbAsTypeString = String::from("like"); // action transitive verb e.g. "I like you."
/// let x: VerbAsTypeString = String::from("walk"); // action intransitive verb e.g. "I walk."
/// let x: VerbAsTypeString = String::from("feel"); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x: VerbAsTypeString = String::from("seem"); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x: VerbAsTypeString = String::from("will"); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x: VerbAsTypeString = String::from("must"); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub type VerbAsTypeString = String;

/// Verb as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x = VerbAsStructStr("like"); // action transitive verb e.g. "I like you."
/// let x = VerbAsStructStr("walk"); // action intransitive verb e.g. "I walk."
/// let x = VerbAsStructStr("feel"); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x = VerbAsStructStr("seem"); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x = VerbAsStructStr("will"); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x = VerbAsStructStr("must"); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub struct VerbAsStructStr(pub &'static str);

/// Verb as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x = VerbAsStructString(String::from("like")); // action transitive verb e.g. "I like you."
/// let x = VerbAsStructString(String::from("walk")); // action intransitive verb e.g. "I walk."
/// let x = VerbAsStructString(String::from("feel")); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x = VerbAsStructString(String::from("seem")); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x = VerbAsStructString(String::from("will")); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x = VerbAsStructString(String::from("must")); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub struct VerbAsStructString(pub String);
