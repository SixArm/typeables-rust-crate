//! # Phone E.164 Trial Identification Code
//!
//! Examples:
//! ```
//! # use ::typeables::phone_e164_trial_identification_code::*;
//! let x = PhoneE164TrialIdentificationCodeAsStructStr("TODO");
//! ```
//! 
//! Compare:
//! 
//!   * [phone_e164_text](../phone_e164_text)
//!   * [phone_e164_country_code](../phone_e164_country_code)
//!   * [phone_e164_national_destimation_code](../phone_e164_national_destimation_code)
//!   * [phone_e164_subscriber_number](../phone_e164_subscriber_number)
//!   * [phone_e164_group_identification_code](../phone_e164_group_identification_code)
//!   * [phone_e164_trial_identification_code](../phone_e164_trial_identification_code)
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
//! international call prefix. The presentation of a number at the B-party
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

pub struct PhoneE164TrialIdentificationCodeAsStructStr(pub &'static str);
pub struct PhoneE164TrialIdentificationCodeAsStructString(pub String);

pub type PhoneE164TrialIdentificationCodeAsTypeStr = str;
pub type PhoneE164TrialIdentificationCodeAsTypeString = String;
