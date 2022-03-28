//! # Verb
//!
//! Examples:
//!
//! ```rust
//! # use typeables::verb::*;
//! let x = GrammarVerbAsStructStr("like"); // action transitive verb e.g. "I like you."
//! let x = GrammarVerbAsStructStr("walk"); // action intransitive verb e.g. "I walk."
//! let x = GrammarVerbAsStructStr("feel"); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
//! let x = GrammarVerbAsStructStr("seem"); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
//! let x = GrammarVerbAsStructStr("will"); // modal verb a.k.a. helping verb e.g. "I will walk."
//! let x = GrammarVerbAsStructStr("must"); // modal verb a.k.a. helping verb e.g. "I must walk."
//! ```
//!
//! <https://wikipedia.org/wiki/Verb>
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

//// Verb

pub struct GrammarVerbAsStructStr(pub &'static str);
pub struct GrammarVerbAsStructString(pub String);

pub type GrammarVerbAsTypeStr = str;
pub type VerbAsTypeString = String;
