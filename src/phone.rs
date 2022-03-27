//! # Phone E.164 format
//!
//! <https://en.wikipedia.org/wiki/E.164>
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
//! <https://en.wikipedia.org/wiki/List_of_country_calling_codes>
//!
//! Fake phone numbers for example purposes:
//!
//!   * United States fake phone numbers start with local number 555.
//!   * China fake phone numbers start with 13, 15, 18.
//!   * India fake phone numbers start with 7, 8, 9.
//!
//!
//! ## Phone E.164 Text
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164TextAsStructStr("+1 212 555 0000"); // United States example
//! let x = PhoneE164TextAsStructStr("+86 13 0000 0000"); // China example
//! let x = PhoneE164TextAsStructStr("+91 900 0000 000"); // India example
//! ```
//!
//!
//! ## Phone E.164 Country Code
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164CountryCodeAsStructStr("1"); // United States
//! let x = PhoneE164CountryCodeAsStructStr("86"); // China
//! let x = PhoneE164CountryCodeAsStructStr("91"); // India
//! ```
//!
//!
//! ## Phone E.164 National Destination Code
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164NationalDestinationCodeAsStructStr("212"); // United States + New York City
//! let x = PhoneE164NationalDestinationCodeAsStructStr("10"); // China + Beijing
//! let x = PhoneE164NationalDestinationCodeAsStructStr("11"); // India + Delhi
//! ```
//!
//!
//! ## Phonee E.164 Subscriber Number
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164SubscriberNumberAsStructStr("555 0000"); // United States example
//! let x = PhoneE164SubscriberNumberAsStructStr("0000 0000"); // China example
//! let x = PhoneE164SubscriberNumberAsStructStr("0000 000"); // India example
//! ```
//!
//!
//! ## Phone E.164 Group Identification Code
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164GroupIdentificationCodeAsStructStr("TODO");
//! ```
//!
//!
//! ## Phone E.164 Trial Identification Code
//!
//! Examples:
//! ```
//! # use ::typeables::phone::*;
//! let x = PhoneE164TrialIdentificationCodeAsStructStr("TODO");
//! ```

//// Phone E.164

pub type PhoneE164TextAsTypeStr = str;
pub type PhoneE164TextAsTypeString = String;

pub struct PhoneE164TextAsStructStr(pub &'static str);
pub struct PhoneE164TextAsStructString(pub String);

/// Phone E.164 Country Code

pub type PhoneE164CountryCodeAsTypeStr = str;
pub type PhoneE164CountryCodeAsTypeString = String;

pub struct PhoneE164CountryCodeAsStructStr(pub &'static str);
pub struct PhoneE164CountryCodeAsStructString(pub String);

//// Phone E.164 National Destination Code

pub type PhoneE164NationalDestinationCodeAsTypeStr = str;
pub type PhoneE164NationalDestinationCodeAsTypeString = String;

pub struct PhoneE164NationalDestinationCodeAsStructStr(pub &'static str);
pub struct PhoneE164NationalDestinationCodeAsStructString(pub String);

/// Phonee E.164 Subscriber Number

pub type PhoneE164SubscriberNumberAsTypeStr = str;
pub type PhoneE164SubscriberNumberAsTypeString = String;

pub struct PhoneE164SubscriberNumberAsStructStr(pub &'static str);
pub struct PhoneE164SubscriberNumberAsStructString(pub String);


//// Phone E.164 Group Identification Code

pub struct PhoneE164GroupIdentificationCodeAsStructStr(pub &'static str);
pub struct PhoneE164GroupIdentificationCodeAsStructString(pub String);

pub type PhoneE164GroupIdentificationCodeAsTypeStr = str;
pub type PhoneE164GroupIdentificationCodeAsTypeString = String;

//// Phone E.164 Trial Identification Code

pub struct PhoneE164TrialIdentificationCodeAsStructStr(pub &'static str);
pub struct PhoneE164TrialIdentificationCodeAsStructString(pub String);

pub type PhoneE164TrialIdentificationCodeAsTypeStr = str;
pub type PhoneE164TrialIdentificationCodeAsTypeString = String;
