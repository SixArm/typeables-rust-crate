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

/// Noun as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: &NounStr = "Alice"; // person
/// let x: &NounStr = "beach"; // place
/// let x: &NounStr = "clock"; // thing
/// let x: &NounStr = "dream"; // idea
/// let x: &NounStr = "eagle"; // animal
/// let x: &NounStr = "fruit"; // plant
/// ```
pub type NounStr = str;

/// Noun as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: NounString = "Alice".into(); // person
/// let x: NounString = "beach".into(); // place
/// let x: NounString = "clock".into(); // thing
/// let x: NounString = "dream".into(); // idea
/// let x: NounString = "eagle".into(); // animal
/// let x: NounString = "fruit".into(); // plant
/// ```
pub type NounString = String;

/// Verb as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: &VerbStr = "like"; // action transitive verb e.g. "I like you."
/// let x: &VerbStr = "walk"; // action intransitive verb e.g. "I walk."
/// let x: &VerbStr = "feel"; // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x: &VerbStr = "seem"; // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x: &VerbStr = "will"; // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x: &VerbStr = "must"; // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub type VerbStr = str;

/// Verb as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::grammar::*;
/// let x: VerbString = "like".into(); // action transitive verb e.g. "I like you."
/// let x: VerbString = "walk".into(); // action intransitive verb e.g. "I walk."
/// let x: VerbString = "feel".into(); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x: VerbString = "seem".into(); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x: VerbString = "will".into(); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x: VerbString = "must".into(); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub type VerbString = String;
