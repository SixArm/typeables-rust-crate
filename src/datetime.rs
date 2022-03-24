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
//!   * DateTime As (format)
//!
//!   * Time Offset As (format)
//!
//!   * Time Zone As (format)

/// Year format as "YYYY" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &YearAsYYYYAsTypeStr = "2020"; // NASA launches Mars Perseverance Rover
/// let x: &YearAsYYYYAsTypeStr = "1088"; // First university founded
/// let x: &YearAsYYYYAsTypeStr = "0105"; // First use of modern paper
/// let x: &YearAsYYYYAsTypeStr = "0000"; // Start of common era
/// ```
pub type YearAsYYYYAsTypeStr = str;

/// Year format as "YYYY" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: YearAsYYYYAsTypeString = String::from("2020"); // NASA launches Mars Perseverance Rover
/// let x: YearAsYYYYAsTypeString = String::from("1088"); // First university founded
/// let x: YearAsYYYYAsTypeString = String::from("0105"); // First use of modern paper
/// let x: YearAsYYYYAsTypeString = String::from("0000"); // Start of common era
/// ```
pub type YearAsYYYYAsTypeString = String;

/// Year format as "YYYY" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = YearAsYYYYAsStructStr("2020"); // NASA launches Mars Perseverance Rover
/// let x = YearAsYYYYAsStructStr("1088"); // First university founded
/// let x = YearAsYYYYAsStructStr("0105"); // First use of modern paper
/// let x = YearAsYYYYAsStructStr("0000"); // Start of common era
/// ```
pub struct YearAsYYYYAsStructStr(pub &'static str);

/// Year format as "YYYY" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = YearAsYYYYAsStructString(String::from("2020")); // NASA launches Mars Perseverance Rover
/// let x = YearAsYYYYAsStructString(String::from("1088")); // First university founded
/// let x = YearAsYYYYAsStructString(String::from("0105")); // First use of modern paper
/// let x = YearAsYYYYAsStructString(String::from("0000")); // Start of common era
/// ```
pub struct YearAsYYYYAsStructString(pub String);

/// Month format as "MM" as type `str` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &MonthAsMMAsTypeStr = "01"; // January
/// let x: &MonthAsMMAsTypeStr = "02"; // February
/// let x: &MonthAsMMAsTypeStr = "03"; // Marck
/// ```
pub type MonthAsMMAsTypeStr = str;

/// Month format as "MM" as type `String` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: MonthAsMMAsTypeString = String::from("01"); // January
/// let x: MonthAsMMAsTypeString = String::from("02"); // February
/// let x: MonthAsMMAsTypeString = String::from("03"); // Marck
/// ```
pub type MonthAsMMAsTypeString = String;

/// Month format as "MM" as struct `str` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = MonthAsMMAsStructStr("01"); // January
/// let x = MonthAsMMAsStructStr("02"); // February
/// let x = MonthAsMMAsStructStr("03"); // Marck
/// ```
pub struct MonthAsMMAsStructStr(pub &'static str);

/// Month format as "MM" as struct `String` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = MonthAsMMAsStructString(String::from("01")); // January
/// let x = MonthAsMMAsStructString(String::from("02")); // February
/// let x = MonthAsMMAsStructString(String::from("03")); // Marck
/// ```
pub struct MonthAsMMAsStructString(pub String);

/// Day format as "DD" as type `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DayAsDDAsTypeStr = "01"; // Monday
/// let x: &DayAsDDAsTypeStr = "02"; // Tuesday
/// let x: &DayAsDDAsTypeStr = "03"; // Wednesday
/// ```
pub type DayAsDDAsTypeStr = str;

/// Day format as "DD" as type `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DayAsDDAsTypeString = String::from("01"); // Monday
/// let x: DayAsDDAsTypeString = String::from("02"); // Tuesday
/// let x: DayAsDDAsTypeString = String::from("03"); // Wednesday
/// ```
pub type DayAsDDAsTypeString = String;

/// Day format as "DD" as struct `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DayAsDDAsStructStr("01"); // Monday
/// let x = DayAsDDAsStructStr("02"); // Tuesday
/// let x = DayAsDDAsStructStr("03"); // Wednesday
/// ```
pub struct DayAsDDAsStructStr(pub &'static str);

/// Day format as "DD" as struct `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DayAsDDAsStructString(String::from("01")); // Monday
/// let x = DayAsDDAsStructString(String::from("02")); // Tuesday
/// let x = DayAsDDAsStructString(String::from("03")); // Wednesday
/// ```
pub struct DayAsDDAsStructString(pub String);

/// Hour format as "HH" as type `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &HourAsHHAsTypeStr = "00"; // start of day a.k.a. midnight
/// let x: &HourAsHHAsTypeStr = "23"; // finish of day a.k.a. 11 p.m.
/// ```
pub type HourAsHHAsTypeStr = str;

/// Hour format as "HH" as type `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: HourAsHHAsTypeString = String::from("00"); // start a.k.a. midnight
/// let x: HourAsHHAsTypeString = String::from("23"); // finish a.k.a. 11 p.m.
/// ```
pub type HourAsHHAsTypeString = String;

/// Hour format as "HH" as struct `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = HourAsHHAsStructStr("00"); // start of day a.k.a. midnight
/// let x = HourAsHHAsStructStr("23"); // finish of day a.k.a. 11 p.m.
/// ```
pub struct HourAsHHAsStructStr(pub &'static str);

/// Hour format as "HH" as struct `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = HourAsHHAsStructString(String::from("00")); // start a.k.a. midnight
/// let x = HourAsHHAsStructString(String::from("23")); // finish a.k.a. 11 p.m.
/// ```
pub struct HourAsHHAsStructString(pub String);

/// Minute format as "MM" as type `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &MinuteAsMMAsTypeStr = "00"; // start of hour
/// let x: &MinuteAsMMAsTypeStr = "59"; // finish of hour
/// ```
pub type MinuteAsMMAsTypeStr = str;

/// Minute format as "MM" as type `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: MinuteAsMMAsTypeString = String::from("00"); // start
/// let x: MinuteAsMMAsTypeString = String::from("59"); // finish
/// ```
pub type MinuteAsMMAsTypeString = String;

/// Minute format as "MM" as struct `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = MinuteAsMMAsStructStr("00"); // start of hour
/// let x = MinuteAsMMAsStructStr("59"); // finish of hour
/// ```
pub struct MinuteAsMMAsStructStr(pub &'static str);

/// Minute format as "MM" as struct `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = MinuteAsMMAsStructString(String::from("00")); // start
/// let x = MinuteAsMMAsStructString(String::from("59")); // finish
/// ```
pub struct MinuteAsMMAsStructString(pub String);

/// Second format as "SS" as type `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &SecondAsSSAsTypeStr = "00"; // start
/// let x: &SecondAsSSAsTypeStr = "59"; // finish
/// ```
pub type SecondAsSSAsTypeStr = str;

/// Second format as "SS" as type `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: SecondAsSSAsTypeString = String::from("00"); // start
/// let x: SecondAsSSAsTypeString = String::from("59"); // finish
/// ```
pub type SecondAsSSAsTypeString = String;

/// Second format as "SS" as struct `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = SecondAsSSAsStructStr("00"); // start
/// let x = SecondAsSSAsStructStr("59"); // finish
/// ```
pub struct SecondAsSSAsStructStr(pub &'static str);

/// Second format as "SS" as struct `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = SecondAsSSAsStructString(String::from("00")); // start
/// let x = SecondAsSSAsStructString(String::from("59")); // finish
/// ```
pub struct SecondAsSSAsStructString(pub String);

/// Date format as "YYYYMMDD" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateAsYYYYMMDDAsTypeStr = "20200730"; // NASA launches Mars Perseverance Rover
/// let x: &DateAsYYYYMMDDAsTypeStr = "00000000"; // Start of common era
/// ```
pub type DateAsYYYYMMDDAsTypeStr = str;

/// Date format as "YYYYMMDD" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateAsYYYYMMDDAsTypeString = String::from("20200730"); // NASA launches Mars Perseverance Rover
/// let x: DateAsYYYYMMDDAsTypeString = String::from("00000000"); // Start of common era
/// ```
pub type DateAsYYYYMMDDAsTypeString = String;

/// Date format as "YYYYMMDD" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateAsYYYYMMDDAsStructStr("20200730"); // NASA launches Mars Perseverance Rover
/// let x = DateAsYYYYMMDDAsStructStr("00000000"); // Start of common era
/// ```
pub struct DateAsYYYYMMDDAsStructStr(pub &'static str);

/// Date format as "YYYYMMDD" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateAsYYYYMMDDAsStructString(String::from("20200730")); // NASA launches Mars Perseverance Rover
/// let x = DateAsYYYYMMDDAsStructString(String::from("00000000")); // Start of common era
/// ```
pub struct DateAsYYYYMMDDAsStructString(pub String);

/// Date format as "YYYY-MM-DD" as type `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateAsYYYYXMMXDDAsTypeStr = "2020-07-30"; // NASA launches Mars Perseverance Rover
/// let x: &DateAsYYYYXMMXDDAsTypeStr = "0000-00-00"; // Start of common era
/// ```
pub type DateAsYYYYXMMXDDAsTypeStr = str;

/// Date format as "YYYY-MM-DD" as type `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateAsYYYYXMMXDDAsTypeString = String::from("2020-07-30"); // NASA launches Mars Perseverance Rover
/// let x: DateAsYYYYXMMXDDAsTypeString = String::from("0000-00-00"); // Start of common era
/// ```
pub type DateAsYYYYXMMXDDAsTypeString = String;

/// Date format as "YYYY-MM-DD" as struct `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateAsYYYYXMMXDDAsStructStr("2020-07-30"); // NASA launches Mars Perseverance Rover
/// let x = DateAsYYYYXMMXDDAsStructStr("0000-00-00"); // Start of common era
/// ```
pub struct DateAsYYYYXMMXDDAsStructStr(pub &'static str);

/// Date format as "YYYY-MM-DD" as struct `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateAsYYYYXMMXDDAsStructString(String::from("2020-07-30")); // NASA launches Mars Perseverance Rover
/// let x = DateAsYYYYXMMXDDAsStructString(String::from("0000-00-00")); // Start of common era
/// ```
pub struct DateAsYYYYXMMXDDAsStructString(pub String);

/// Time format as "HHMMSS" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeAsHHMMSSAsTypeStr = "000000"; // start of day
/// let x: &TimeAsHHMMSSAsTypeStr = "235959"; // finish of day
/// ```
pub type TimeAsHHMMSSAsTypeStr = str;

/// Time format as "YYYYMMDD" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeAsHHMMSSAsTypeString = String::from("000000"); // start of day
/// let x: TimeAsHHMMSSAsTypeString = String::from("235959"); // finish of day
/// ```
pub type TimeAsHHMMSSAsTypeString = String;

/// Time format as "HHMMSS" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeAsHHMMSSAsStructStr("000000"); // start of day
/// let x = TimeAsHHMMSSAsStructStr("235959"); // finish of day
/// ```
pub struct TimeAsHHMMSSAsStructStr(pub &'static str);

/// Time format as "YYYYMMDD" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeAsHHMMSSAsStructString(String::from("000000")); // start of day
/// let x = TimeAsHHMMSSAsStructString(String::from("235959")); // finish of day
/// ```
pub struct TimeAsHHMMSSAsStructString(pub String);

/// Time format as "HH:MM:SS" as type `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeAsHHXMMXSSAsTypeStr = "00:00:00"; // start of day
/// let x: &TimeAsHHXMMXSSAsTypeStr = "23:59:59"; // finish of day
/// ```
pub type TimeAsHHXMMXSSAsTypeStr = str;

/// Time format as "YYYY-MM-DD" as type `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeAsHHXMMXSSAsTypeString = String::from("00:00:00"); // start of day
/// let x: TimeAsHHXMMXSSAsTypeString = String::from("23:59:59"); // finish of day
/// ```
pub type TimeAsHHXMMXSSAsTypeString = String;

/// Time format as "HH:MM:SS" as struct `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeAsHHXMMXSSAsStructStr("00:00:00"); // start of day
/// let x = TimeAsHHXMMXSSAsStructStr("23:59:59"); // finish of day
/// ```
pub struct TimeAsHHXMMXSSAsStructStr(pub &'static str);

/// Time format as "YYYY-MM-DD" as struct `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeAsHHXMMXSSAsStructString(String::from("00:00:00")); // start of day
/// let x = TimeAsHHXMMXSSAsStructString(String::from("23:59:59")); // finish of day
/// ```
pub struct TimeAsHHXMMXSSAsStructString(pub String);

/// DateTime format as "YYYYMMDDHHMMSS" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateTimeAsYYYYMMDDHHMMSSAsTypeStr = "20200730075000"; // NASA launches Mars Perseverance Rover (EDT)
/// let x: &DateTimeAsYYYYMMDDHHMMSSAsTypeStr = "00000000000000"; // Start of common era
/// ```
pub type DateTimeAsYYYYMMDDHHMMSSAsTypeStr = str;

/// DateTime format as "YYYYMMDDHHMMSS" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateTimeAsYYYYMMDDHHMMSSAsTypeString = String::from("20200730075000"); // Mars Perseverance Rover launch (EDT)
/// let x: DateTimeAsYYYYMMDDHHMMSSAsTypeString = String::from("00000000000000"); // Start of common era
/// ```
pub type DateTimeAsYYYYMMDDHHMMSSAsTypeString = String;

/// DateTime format as "YYYYMMDDHHMMSS" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateTimeAsYYYYMMDDHHMMSSAsStructStr("20200730075000"); // NASA launches Mars Perseverance Rover (EDT)
/// let x = DateTimeAsYYYYMMDDHHMMSSAsStructStr("00000000000000"); // Start of common era
/// ```
pub struct DateTimeAsYYYYMMDDHHMMSSAsStructStr(pub &'static str);

/// DateTime format as "YYYYMMDDHHMMSS" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateTimeAsYYYYMMDDHHMMSSAsStructString(String::from("20200730075000")); // Mars Perseverance Rover launch (EDT)
/// let x = DateTimeAsYYYYMMDDHHMMSSAsStructString(String::from("00000000000000")); // Start of common era
/// ```
pub struct DateTimeAsYYYYMMDDHHMMSSAsStructString(pub String);

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as type `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeStr = "2020-07-30T07:50:00"; // NASA launches Mars Perseverance Rover (EDT)
/// let x: &DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeStr = "0000-00-00T00:00:00"; // Start of common era
/// ```
pub type DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeStr = str;

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as type `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeString = String::from("2020-07-30T07:50:00"); // NASA launches Mars Perseverance Rover (EDT)
/// let x: DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeString = String::from("0000-00-00T00:00:00"); // Start of common era
/// ```
pub type DateTimeAsYYYYXMMXDDXHHXMMXSSAsTypeString = String;

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as struct `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr("2020-07-30T07:50:00"); // NASA launches Mars Perseverance Rover (EDT)
/// let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr("0000-00-00T00:00:00"); // Start of common era
/// ```
pub struct DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructStr(pub &'static str);

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as struct `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructString(String::from("2020-07-30T07:50:00")); // NASA launches Mars Perseverance Rover (EDT)
/// let x = DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructString(String::from("0000-00-00T00:00:00")); // Start of common era
/// ```
pub struct DateTimeAsYYYYXMMXDDXHHXMMXSSAsStructString(pub String);

/// Time zone format as "+HH:MM" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeOffsetAsHHXMMAsTypeStr = "+00:00"; // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x: &TimeOffsetAsHHXMMAsTypeStr = "-05:00"; // Eastern Daylight Time (EDT)
/// ```
pub type TimeOffsetAsHHXMMAsTypeStr = str;

/// Time zone format as "+HH:MM" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeOffsetAsHHXMMAsTypeString = String::from("+00:00"); // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x: TimeOffsetAsHHXMMAsTypeString = String::from("-05:00"); // Eastern Daylight Time (EDT)
/// ```
pub type TimeOffsetAsHHXMMAsTypeString = String;

/// Time zone format as "+HH:MM" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeOffsetAsHHXMMAsStructStr("+00:00"); // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x = TimeOffsetAsHHXMMAsStructStr("-05:00"); // Eastern Daylight Time (EDT)
/// ```
pub struct TimeOffsetAsHHXMMAsStructStr(pub &'static str);

/// Time zone format as "+HH:MM" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeOffsetAsHHXMMAsStructString(String::from("+00:00")); // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x = TimeOffsetAsHHXMMAsStructString(String::from("-05:00")); // Eastern Daylight Time (EDT)
/// ```
pub struct TimeOffsetAsHHXMMAsStructString(pub String);

/// Time zone format as "+HH:MM" as type `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeZoneAsAbbreviationAsTypeStr = "ET"; // U.S. Eastern Time
/// let x: &TimeZoneAsAbbreviationAsTypeStr = "BST"; // British Summer Time
/// ```
pub type TimeZoneAsAbbreviationAsTypeStr = str;

/// Time zone format as "+HH:MM" as type `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeZoneAsAbbreviationAsTypeString = String::from("ET"); // U.S. Eastern Time
/// let x: TimeZoneAsAbbreviationAsTypeString = String::from("BST"); // British Summer Time
/// ```
pub type TimeZoneAsAbbreviationAsTypeString = String;

/// Time zone format as "+HH:MM" as struct `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeZoneAsAbbreviationAsStructStr("ET"); // U.S. Eastern Time
/// let x = TimeZoneAsAbbreviationAsStructStr("BST"); // British Summer Time
/// ```
pub struct TimeZoneAsAbbreviationAsStructStr(pub &'static str);

/// Time zone format as "+HH:MM" as struct `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x = TimeZoneAsAbbreviationAsStructString(String::from("ET")); // U.S. Eastern Time
/// let x = TimeZoneAsAbbreviationAsStructString(String::from("BST")); // British Summer Time
/// ```
pub struct TimeZoneAsAbbreviationAsStructString(pub String);
