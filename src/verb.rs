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
//!
//! Compare: Noun

/// Verb as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::verb::*;
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
/// # use ::typeables::verb::*;
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
/// # use ::typeables::verb::*;
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
/// # use ::typeables::verb::*;
/// let x = VerbAsStructString(String::from("like")); // action transitive verb e.g. "I like you."
/// let x = VerbAsStructString(String::from("walk")); // action intransitive verb e.g. "I walk."
/// let x = VerbAsStructString(String::from("feel")); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x = VerbAsStructString(String::from("seem")); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x = VerbAsStructString(String::from("will")); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x = VerbAsStructString(String::from("must")); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```
pub struct VerbAsStructString(pub String);
