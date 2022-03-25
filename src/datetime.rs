//! Date-time
//!
//! This module provides:
//!
//!   * Year As (format)
//!
//!   * Month As (format)
//!
//!   * Day As (format)
//!
//!   * Hour As (format)
//!
//!   * Minute As (format)
//!
//!   * Second As (format)
//!
//!   * Date As (format)
//!
//!   * Time As (format)
//!
//!   * Time Offset As (format)
//!
//!   * Time Zone As (format)
//!
//!   * DateTime As (format)
//!
//!
//! ## Year format as "YYYY"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = YearAsYYYYAsStructStr("2020"); // NASA launches Mars Perseverance Rover
//! let x = YearAsYYYYAsStructStr("0000"); // Start of common era
//! ```
//!
//!
//! ## Month format as "MM" from "01" January to "12" December
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = MonthAsMMAsStructStr("01"); // January
//! let x = MonthAsMMAsStructStr("12"); // December
//! ```
//!
//!
//! ## Day format as "DD" from "01" Monday to "07" Sunday
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = DayAsDDAsStructStr("01"); // Monday
//! let x = DayAsDDAsStructStr("07"); // Sunday
//! ```
//!
//!
//! ## Hour format as "HH" from "00" to "23"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = HourAsHHAsStructStr("00"); // start of day a.k.a. midnight
//! let x = HourAsHHAsStructStr("23"); // finish of day a.k.a. 11 p.m.
//! ```
//!
//!
//! ## Minute format as "MM" from "00" to "59"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = MinuteAsMMAsStructStr("00"); // start of hour
//! let x = MinuteAsMMAsStructStr("59"); // finish of hour
//! ```
//!
//!
//! ## Second format as "SS" from "00" to "59"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = SecondAsSSAsStructStr("00"); // start of minute
//! let x = SecondAsSSAsStructStr("59"); // finish of minute
//! ```
//!
//!
//! ## Date format as "YYYYMMDD"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = DateAsYYYYMMDDAsStructStr("20200730"); // NASA launches Mars Perseverance Rover
//! let x = DateAsYYYYMMDDAsStructStr("00000000"); // Start of common era
//! ```
//!
//!
//! ### Date format as "YYYY-MM-DD", same as ISO 8601.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = DateAsYYYYXMMXDDAsStructStr("2020-07-30"); // NASA launches Mars Perseverance Rover
//! let x = DateAsYYYYXMMXDDAsStructStr("0000-00-00"); // Start of common era
//! ```
//!
//!
//! ## Time format as "HHMMSS"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = TimeAsHHMMSSAsStructStr("000000"); // start of day
//! let x = TimeAsHHMMSSAsStructStr("235959"); // finish of day
//! ```
//!
//!
//! ## Time format as "HH:MM:SS", same as ISO 8601.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = TimeAsHHXMMXSSAsStructStr("00:00:00"); // start of day
//! let x = TimeAsHHXMMXSSAsStructStr("23:59:59"); // finish of day
//! ```
//!
//!
//! ## Time offset format as "+HH:MM"
//!
//! Examples:
//! ```
//! # use ::typeables::datetime::*;
//! let x = TimeOffsetAsXHHXMMAsStructStr("+00:00"); // UTC a.k.a. GMT a.k.a. Zulu time zone.
//! let x = TimeOffsetAsXHHXMMAsStructStr("-05:00"); // Eastern Daylight Time (EDT)
//! ```
//!
//!
//! ## Time zone abbreviation format
//!
//! Examples:
//! ```
//! # use ::typeables::datetime::*;
//! let x = TimeZoneAsAbbreviationAsStructStr("ET"); // U.S. Eastern Time
//! let x = TimeZoneAsAbbreviationAsStructStr("BST"); // British Summer Time
//! ```
//!
//!
//! ## DateTime format as "YYYYMMDDHHMMSS"
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = DateTimeAsYYYYMMDDHHMMSSAsStructStr("20200730075000"); // NASA launches Mars Perseverance Rover (EDT)
//! let x = DateTimeAsYYYYMMDDHHMMSSAsStructStr("00000000000000"); // Start of common era
//! ```
//!
//!
//! DateTime format as "YYYY-MM-DDTHH:MM:SS", same as ISO 8601.
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::datetime::*;
//! let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr("2020-07-30T07:50:00"); // NASA launches Mars Perseverance Rover (EDT)
//! let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr("0000-00-00T00:00:00"); // Start of common era
//! ```

//// Year format as "YYYY"

pub struct YearAsYYYYAsStructStr(pub &'static str);
pub struct YearAsYYYYAsStructString(pub String);

pub type YearAsYYYYAsTypeStr = str;
pub type YearAsYYYYAsTypeString = String;

//// Month format as "MM" from "01" January to "12" December

pub struct MonthAsMMAsStructStr(pub &'static str);
pub struct MonthAsMMAsStructString(pub String);

pub type MonthAsMMAsTypeStr = str;
pub type MonthAsMMAsTypeString = String;

//// Day format as "DD" from "01" Monday to "07" Sunday

pub struct DayAsDDAsStructStr(pub &'static str);
pub struct DayAsDDAsStructString(pub String);

pub type DayAsDDAsTypeStr = str;
pub type DayAsDDAsTypeString = String;

//// Hour format as "HH" from "00" to "23"

pub struct HourAsHHAsStructStr(pub &'static str);
pub struct HourAsHHAsStructString(pub String);

pub type HourAsHHAsTypeStr = str;
pub type HourAsHHAsTypeString = String;

//// Minute format as "MM" from "00" to "59"

pub struct MinuteAsMMAsStructStr(pub &'static str);
pub struct MinuteAsMMAsStructString(pub String);

pub type MinuteAsMMAsTypeStr = str;
pub type MinuteAsMMAsTypeString = String;

//// Second format as "SS" from "00" to "59"

pub struct SecondAsSSAsStructStr(pub &'static str);
pub struct SecondAsSSAsStructString(pub String);

pub type SecondAsSSAsTypeStr = str;
pub type SecondAsSSAsTypeString = String;

//// Date format as "YYYYMMDD"

pub struct DateAsYYYYMMDDAsStructStr(pub &'static str);
pub struct DateAsYYYYMMDDAsStructString(pub String);

pub type DateAsYYYYMMDDAsTypeStr = str;
pub type DateAsYYYYMMDDAsTypeString = String;

//// Date format as "YYYY-MM-DD", same as ISO 8601.

pub struct DateAsYYYYXMMXDDAsStructStr(pub &'static str);
pub struct DateAsYYYYXMMXDDAsStructString(pub String);

pub type DateAsYYYYXMMXDDAsTypeStr = str;
pub type DateAsYYYYXMMXDDAsTypeString = String;

//// Time format as "HHMMSS"

pub struct TimeAsHHMMSSAsStructStr(pub &'static str);
pub struct TimeAsHHMMSSAsStructString(pub String);

pub type TimeAsHHMMSSAsTypeStr = str;
pub type TimeAsHHMMSSAsTypeString = String;

//// Time format as "HH:MM:SS", same as ISO 8601.

pub struct TimeAsHHXMMXSSAsStructStr(pub &'static str);

pub struct TimeAsHHXMMXSSAsStructString(pub String);

pub type TimeAsHHXMMXSSAsTypeStr = str;
pub type TimeAsHHXMMXSSAsTypeString = String;

//// DateTime format as "YYYYMMDDHHMMSS"

pub struct DateTimeAsYYYYMMDDHHMMSSAsStructStr(pub &'static str);
pub struct DateTimeAsYYYYMMDDHHMMSSAsStructString(pub String);

pub type DateTimeAsYYYYMMDDHHMMSSAsTypeStr = str;
pub type DateTimeAsYYYYMMDDHHMMSSAsTypeString = String;

//// DateTime format as "YYYY-MM-DDTHH:MM:SS", same as ISO 8601.

pub struct DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr(pub &'static str);

pub struct DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructString(pub String);

pub type DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeStr = str;
pub type DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeString = String;

//// Time offset format as "+HH:MM"

pub struct TimeOffsetAsXHHXMMAsStructStr(pub &'static str);
pub struct TimeOffsetAsXHHXMMAsStructString(pub String);

pub type TimeOffsetAsXHHXMMAsTypeStr = str;
pub type TimeOffsetAsXHHXMMAsTypeString = String;

//// Time zone abbreviation format as "[A-Z]+"

pub struct TimeZoneAsAbbreviationAsStructStr(pub &'static str);
pub struct TimeZoneAsAbbreviationAsStructString(pub String);

pub type TimeZoneAsAbbreviationAsTypeStr = str;
pub type TimeZoneAsAbbreviationAsTypeString = String;
