//! # Phone E.164 format
//!
//! https://en.wikipedia.org/wiki/E.164
//!
//! E.164 is an international standard (ITU-T Recommendation), titled The
//! international public telecommunication numbering plan, that defines a
//! numbering plan for the worldwide public switched telephone network (PSTN)
//! and some other data networks.
//!
//! E.164 defines a general format for international telephone numbers.
//! Plan-conforming numbers are limited to a maximum of 15 digits, excluding the
//! international call prefix.[1] The presentation of a number at the B-party
//! device is usually prefixed with the plus sign (+), indicating that the
//! number includes the country calling code. This is done by the B-party
//! subscribers network by usually looking at the NOA (Nature Of Address) field
//! of the signaling messages. When dialing, the number must typically be
//! prefixed with the appropriate international call prefix (in place of the
//! plus sign), which is a trunk code to reach an international circuit from
//! within the country of call origination. 
//!
//! E.164 phone number format contains: [+][country code][area code][local phone number]
//! 
//!   * `+` is the plus sign which indicates use of a country code.
//!   * country code a.k.a. international dialer e.g. `1` is United States.
//!   * national destination code a.k.a. area code e.g. `212` is New York City.
//!   * local phone number e.g. `555 0000` is an example U.S. local phone number.
//!
//! List of country calling codes:
//! https://en.wikipedia.org/wiki/List_of_country_calling_codes
//!
//! Fake phone numbers for example purposes:
//!
//!   * United States fake phone numbers start with local number 555.
//!   * China fake phone numbers start with 13, 15, 18.
//!   * India fake phone numbers start with 7, 8, 9.

/// Phone E.164 text as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164TextAsTypeStr = "+1 212 555 0000"; // United States example
/// let x: &PhoneE164TextAsTypeStr = "+86 13 0000 0000"; // China example
/// let x: &PhoneE164TextAsTypeStr = "+91 900 0000 000"; // India example
/// ```
pub type PhoneE164TextAsTypeStr = str;

/// Phone E.164 text as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164TextAsTypeString = String::from("+1 212 555 0000"); // United States example
/// let x: PhoneE164TextAsTypeString = String::from("+86 13 0000 0000"); // China example
/// let x: PhoneE164TextAsTypeString = String::from("+91 900 0000 000"); // India example
/// ```
pub type PhoneE164TextAsTypeString = String;

/// Phone E.164 text as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164TextAsStructStr("+1 212 555 0000"); // United States example
/// let x = PhoneE164TextAsStructStr("+86 13 0000 0000"); // China example
/// let x = PhoneE164TextAsStructStr("+91 900 0000 000"); // India example
/// ```
pub struct PhoneE164TextAsStructStr(pub &'static str);

/// Phone E.164 text as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164TextAsStructString(String::from("+1 212 555 0000")); // United States example
/// let x = PhoneE164TextAsStructString(String::from("+86 13 0000 0000")); // China example
/// let x = PhoneE164TextAsStructString(String::from("+91 900 0000 000")); // India example
/// ```
pub struct PhoneE164TextAsStructString(pub String);

/// Phone E.164 country code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164CountryCodeAsTypeStr = "1"; // United States
/// let x: &PhoneE164CountryCodeAsTypeStr = "86"; // China
/// let x: &PhoneE164CountryCodeAsTypeStr = "91"; // India
/// ```
pub type PhoneE164CountryCodeAsTypeStr = str;

/// Phone E.164 country code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164CountryCodeAsTypeString = String::from("1"); // United States
/// let x: PhoneE164CountryCodeAsTypeString = String::from("86"); // China
/// let x: PhoneE164CountryCodeAsTypeString = String::from("91"); // India
/// ```
pub type PhoneE164CountryCodeAsTypeString = String;

/// Phone E.164 country code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164CountryCodeAsStructStr("1"); // United States
/// let x = PhoneE164CountryCodeAsStructStr("86"); // China
/// let x = PhoneE164CountryCodeAsStructStr("91"); // India
/// ```
pub struct PhoneE164CountryCodeAsStructStr(pub &'static str);

/// Phone E.164 country code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164CountryCodeAsStructString(String::from("1")); // United States
/// let x = PhoneE164CountryCodeAsStructString(String::from("86")); // China
/// let x = PhoneE164CountryCodeAsStructString(String::from("91")); // India
/// ```
pub struct PhoneE164CountryCodeAsStructString(pub String);

/// Phone E.164 national destination code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164NationalDestinationCodeAsTypeStr = "212"; // United States + New York City
/// let x: &PhoneE164NationalDestinationCodeAsTypeStr = "10"; // China + Beijing
/// let x: &PhoneE164NationalDestinationCodeAsTypeStr = "11"; // India + Delhi
/// ```
pub type PhoneE164NationalDestinationCodeAsTypeStr = str;

/// Phone E.164 national destination code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164NationalDestinationCodeAsTypeString = String::from("212"); // United States + New York City
/// let x: PhoneE164NationalDestinationCodeAsTypeString = String::from("10"); // China + Beijing
/// let x: PhoneE164NationalDestinationCodeAsTypeString = String::from("11"); // India + Delhi
/// ```
pub type PhoneE164NationalDestinationCodeAsTypeString = String;

/// Phone E.164 national destination code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164NationalDestinationCodeAsStructStr("212"); // United States + New York City
/// let x = PhoneE164NationalDestinationCodeAsStructStr("10"); // China + Beijing
/// let x = PhoneE164NationalDestinationCodeAsStructStr("11"); // India + Delhi
/// ```
pub struct PhoneE164NationalDestinationCodeAsStructStr(pub &'static str);

/// Phone E.164 national destination code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164NationalDestinationCodeAsStructString(String::from("212")); // United States + New York City
/// let x = PhoneE164NationalDestinationCodeAsStructString(String::from("10")); // China + Beijing
/// let x = PhoneE164NationalDestinationCodeAsStructString(String::from("11")); // India + Delhi
/// ```
pub struct PhoneE164NationalDestinationCodeAsStructString(pub String);

/// Phonee E.164 subscriber number as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164SubscriberNumberAsTypeStr = "555 0000"; // United States example
/// let x: &PhoneE164SubscriberNumberAsTypeStr = "0000 0000"; // China example
/// let x: &PhoneE164SubscriberNumberAsTypeStr = "0000 000"; // India example
/// ```
pub type PhoneE164SubscriberNumberAsTypeStr = str;

/// Phonee E.164 subscriber number as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164SubscriberNumberAsTypeString = String::from("555 0000"); // United States example
/// let x: PhoneE164SubscriberNumberAsTypeString = String::from("0000 0000"); // China example
/// let x: PhoneE164SubscriberNumberAsTypeString = String::from("0000 000"); // India example
/// ```
pub type PhoneE164SubscriberNumberAsTypeString = String;

/// Phonee E.164 subscriber number as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164SubscriberNumberAsStructStr("555 0000"); // United States example
/// let x = PhoneE164SubscriberNumberAsStructStr("0000 0000"); // China example
/// let x = PhoneE164SubscriberNumberAsStructStr("0000 000"); // India example
/// ```
pub struct PhoneE164SubscriberNumberAsStructStr(pub &'static str);

/// Phonee E.164 subscriber number as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164SubscriberNumberAsStructString(String::from("555 0000")); // United States example
/// let x = PhoneE164SubscriberNumberAsStructString(String::from("0000 0000")); // China example
/// let x = PhoneE164SubscriberNumberAsStructString(String::from("0000 000")); // India example
/// ```
pub struct PhoneE164SubscriberNumberAsStructString(pub String);


/// Phone E.164 group identification code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164GroupIdentificationCodeAsTypeStr = "TODO";
/// ```
pub type PhoneE164GroupIdentificationCodeAsTypeStr = str;

/// Phone E.164 group identification code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164GroupIdentificationCodeAsTypeString = String::from("TODO");
/// ```
pub type PhoneE164GroupIdentificationCodeAsTypeString = String;

/// Phone E.164 group identification code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164GroupIdentificationCodeAsStructStr("TODO");
/// ```
pub struct PhoneE164GroupIdentificationCodeAsStructStr(pub &'static str);

/// Phone E.164 group identification code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164GroupIdentificationCodeAsStructString(String::from("TODO"));
/// ```
pub struct PhoneE164GroupIdentificationCodeAsStructString(pub String);

/// Phone E.164 trial identification code as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164TrialIdentificationCodeAsTypeStr = "TODO";
/// ```
pub type PhoneE164TrialIdentificationCodeAsTypeStr = str;

/// Phone E.164 trial identification code as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164TrialIdentificationCodeAsTypeString = String::from("TODO");
/// ```
pub type PhoneE164TrialIdentificationCodeAsTypeString = String;

/// Phone E.164 trial identification code as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164TrialIdentificationCodeAsStructStr("TODO");
/// ```
pub struct PhoneE164TrialIdentificationCodeAsStructStr(pub &'static str);

/// Phone E.164 trial identification code as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x = PhoneE164TrialIdentificationCodeAsStructString(String::from("TODO"));
/// ```
pub struct PhoneE164TrialIdentificationCodeAsStructString(pub String);
