//! Phone E.164 format
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

/// Phone E.164 text as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164TextStr = "+1 212 555 0000"; // United States example
/// let x: &PhoneE164TextStr = "+86 13 0000 0000"; // China example
/// let x: &PhoneE164TextStr = "+91 900 0000 000"; // India example
/// ```
pub type PhoneE164TextStr = str;

/// Phone E.164 text as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164TextString = "+1 212 555 0000".into(); // United States example
/// let x: PhoneE164TextString = "+86 13 0000 0000".into(); // China example
/// let x: PhoneE164TextString = "+91 900 0000 000".into(); // India example
/// ```
pub type PhoneE164TextString = String;

/// Phone E.164 country code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164CountryCodeStr = "1"; // United States
/// let x: &PhoneE164CountryCodeStr = "86"; // China
/// let x: &PhoneE164CountryCodeStr = "91"; // India
/// ```
pub type PhoneE164CountryCodeStr = str;

/// Phone E.164 country code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164CountryCodeString = "1".into(); // United States
/// let x: PhoneE164CountryCodeString = "86".into(); // China
/// let x: PhoneE164CountryCodeString = "91".into(); // India
/// ```
pub type PhoneE164CountryCodeString = String;

/// Phone E.164 national destination code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164NationalDestinationCodeStr = "212"; // United States + New York City
/// let x: &PhoneE164NationalDestinationCodeStr = "10"; // China + Beijing
/// let x: &PhoneE164NationalDestinationCodeStr = "11"; // India + Delhi
/// ```
pub type PhoneE164NationalDestinationCodeStr = str;

/// Phone E.164 national destination code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164NationalDestinationCodeString = "212".into(); // United States + New York City
/// let x: PhoneE164NationalDestinationCodeString = "10".into(); // China + Beijing
/// let x: PhoneE164NationalDestinationCodeString = "11".into(); // India + Delhi
/// ```
pub type PhoneE164NationalDestinationCodeString = String;

/// Phonee E.164 subscriber number as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164SubscriberNumberStr = "555 0000"; // United States example
/// let x: &PhoneE164SubscriberNumberStr = "0000 0000"; // China example
/// let x: &PhoneE164SubscriberNumberStr = "0000 000"; // India example
/// ```
pub type PhoneE164SubscriberNumberStr = str;

/// Phonee E.164 subscriber number as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164SubscriberNumberString = "555 0000".into(); // United States example
/// let x: PhoneE164SubscriberNumberString = "0000 0000".into(); // China example
/// let x: PhoneE164SubscriberNumberString = "0000 000".into(); // India example
/// ```
pub type PhoneE164SubscriberNumberString = String;

/// Phone E.164 group identification code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164GroupIdentificationCodeStr = "TODO";
/// ```
pub type PhoneE164GroupIdentificationCodeStr = str;

/// Phone E.164 group identification code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164GroupIdentificationCodeString = "TODO".into();
/// ```
pub type PhoneE164GroupIdentificationCodeString = String;

/// Phone E.164 trial identification code as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: &PhoneE164TrialIdentificationCodeStr = "TODO";
/// ```
pub type PhoneE164TrialIdentificationCodeStr = str;

/// Phone E.164 trial identification code as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::phone::*;
/// let x: PhoneE164TrialIdentificationCodeString = "TODO".into();
/// ```
pub type PhoneE164TrialIdentificationCodeString = String;
