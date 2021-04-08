/// Verb
///
/// https://wikipedia.org/wiki/Verb
///
/// A verb, from the Latin verbum meaning word, is a word (part of speech) that
/// in syntax conveys an action (bring, read, walk, run, learn), an occurrence
/// (happen, become), or a state of being (be, exist, stand). In the usual
/// description of English, the basic form, with or without the particle to, is
/// the infinitive. In many languages, verbs are inflected (modified in form) to
/// encode tense, aspect, mood, and voice. A verb may also agree with the
/// person, gender or number of some of its arguments, such as its subject, or
/// object. Verbs have tenses: present, to indicate that an action is being
/// carried out; past, to indicate that an action has been done; future, to
/// indicate that an action will be done.
///
/// # Examples
///
/// ```
/// let x: Verb = "like".into(); // action transitive verb e.g. "I like you."
/// let x: Verb = "walk".into(); // action intransitive verb e.g. "I walk."
/// let x: Verb = "feel".into(); // auxiliary verb a.k.a. linking verb e.g. "I feel happy"
/// let x: Verb = "seem".into(); // auxiliary verb a.k.a. linking verb e.g. "You seem happy."
/// let x: Verb = "will".into(); // modal verb a.k.a. helping verb e.g. "I will walk."
/// let x: Verb = "must".into(); // modal verb a.k.a. helping verb e.g. "I must walk."
/// ```

pub type Verb = String;
