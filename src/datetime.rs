//! Date-time

/// Year format as "YYYY" as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &YearYYYYStr = "2020"; // NASA launches Mars Perseverance Rover
/// let x: &YearYYYYStr = "1088"; // First university founded
/// let x: &YearYYYYStr = "0105"; // First use of modern paper
/// let x: &YearYYYYStr = "0000"; // Start of common era
/// ```
pub type YearYYYYStr = str;

/// Year format as "YYYY" as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: YearYYYYString = "2020".into(); // NASA launches Mars Perseverance Rover
/// let x: YearYYYYString = "1088".into(); // First university founded
/// let x: YearYYYYString = "0105".into(); // First use of modern paper
/// let x: YearYYYYString = "0000".into(); // Start of common era
/// ```
pub type YearYYYYString = String;

/// Month format as "MM" as `str` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &MonthMMStr = "01"; // January
/// let x: &MonthMMStr = "02"; // February
/// let x: &MonthMMStr = "03"; // Marck
/// ```
pub type MonthMMStr = str;

/// Month format as "MM" as `String` starting with "01" as January.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: MonthMMString = "01".into(); // January
/// let x: MonthMMString = "02".into(); // February
/// let x: MonthMMString = "03".into(); // Marck
/// ```
pub type MonthMMString = String;

/// Day format as "DD" as `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DayDDStr = "01"; // Monday
/// let x: &DayDDStr = "02"; // Tuesday
/// let x: &DayDDStr = "03"; // Wednesday
/// ```
pub type DayDDStr = str;

/// Day format as "DD" as `str` starting with "01" as Monday.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DayDDString = "01".into(); // Monday
/// let x: DayDDString = "02".into(); // Tuesday
/// let x: DayDDString = "03".into(); // Wednesday
/// ```
pub type DayDDString = String;


/// Hour format as "HH" as `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &HourHHStr = "00"; // start of day a.k.a. midnight 
/// let x: &HourHHStr = "23"; // finish of day a.k.a. 11 p.m.
/// ```
pub type HourHHStr = str;

/// Hour format as "HH" as `str` from "00" to "23".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: HourHHString = "00".into(); // start a.k.a. midnight 
/// let x: HourHHString = "23".into(); // finish a.k.a. 11 p.m.
/// ```
pub type HourHHString = String;

/// Minute format as "MM" as `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &MinuteMMStr = "00"; // start of hour
/// let x: &MinuteMMStr = "59"; // finish of hour
/// ```
pub type MinuteMMStr = str;

/// Minute format as "MM" as `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: MinuteMMString = "00".into(); // start
/// let x: MinuteMMString = "59".into(); // finish
/// ```
pub type MinuteMMString = String;


/// Second format as "SS" as `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &SecondSSStr = "00"; // start
/// let x: &SecondSSStr = "59"; // finish
/// ```
pub type SecondSSStr = str;

/// Second format as "SS" as `str` from "00" to "59".
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: SecondSSString = "00".into(); // start
/// let x: SecondSSString = "59".into(); // finish
/// ```
pub type SecondSSString = String;

/// Date format as "YYYYMMDD" as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateYYYYMMDDStr = "20200730"; // NASA launches Mars Perseverance Rover
/// let x: &DateYYYYMMDDStr = "00000000"; // Start of common era
/// ```
pub type DateYYYYMMDDStr = str;

/// Date format as "YYYYMMDD" as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateYYYYMMDDString = "20200730".into(); // NASA launches Mars Perseverance Rover
/// let x: DateYYYYMMDDString = "00000000".into(); // Start of common era
/// ```
pub type DateYYYYMMDDString = String;

/// Date format as "YYYY-MM-DD" as `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateYYYYXMMXDDStr = "2020-07-30"; // NASA launches Mars Perseverance Rover
/// let x: &DateYYYYXMMXDDStr = "0000-00-00"; // Start of common era
/// ```
pub type DateYYYYXMMXDDStr = str;

/// Date format as "YYYY-MM-DD" as `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateYYYYXMMXDDString = "2020-07-30".into(); // NASA launches Mars Perseverance Rover
/// let x: DateYYYYXMMXDDString = "0000-00-00".into(); // Start of common era
/// ```
pub type DateYYYYXMMXDDString = String;

/// Time format as "HHMMSS" as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeHHMMSSStr = "000000"; // start of day
/// let x: &TimeHHMMSSStr = "235959"; // finish of day
/// ```
pub type TimeHHMMSSStr = str;

/// Time format as "YYYYMMDD" as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeHHMMSSString = "000000".into(); // start of day
/// let x: TimeHHMMSSString = "235959".into(); // finish of day
/// ```
pub type TimeHHMMSSString = String;

/// Time format as "HH:MM:SS" as `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &TimeHHXMMXSSStr = "00:00:00"; // start of day
/// let x: &TimeHHXMMXSSStr = "23:59:59"; // finish of day
/// ```
pub type TimeHHXMMXSSStr = str;

/// Time format as "YYYY-MM-DD" as `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: TimeHHXMMXSSString = "00:00:00".into(); // start of day
/// let x: TimeHHXMMXSSString = "23:59:59".into(); // finish of day
/// ```
pub type TimeHHXMMXSSString = String;

/// DateTime format as "YYYYMMDDHHMMSS" as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateTimeYYYYMMDDHHMMSSStr = "20200730075000"; // NASA launches Mars Perseverance Rover (EDT)
/// let x: &DateTimeYYYYMMDDHHMMSSStr = "00000000000000"; // Start of common era
/// ```
pub type DateTimeYYYYMMDDHHMMSSStr = str;

/// DateTime format as "YYYYMMDDHHMMSS" as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateTimeYYYYMMDDHHMMSSString = "20200730075000".into(); // Mars Perseverance Rover launch (EDT)
/// let x: DateTimeYYYYMMDDHHMMSSString = "00000000000000".into(); // Start of common era
/// ```
pub type DateTimeYYYYMMDDHHMMSSString = String;

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as `str`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &DateTimeYYYYXMMXDDXHHXMMXSSStr = "2020-07-30T07:50:00"; // NASA launches Mars Perseverance Rover (EDT)
/// let x: &DateTimeYYYYXMMXDDXHHXMMXSSStr = "0000-00-00T00:00:00"; // Start of common era
/// ```
pub type DateTimeYYYYXMMXDDXHHXMMXSSStr = str;

/// DateTime format as "YYYY-MM-DDTHH:MM:SS" as `String`, same as ISO 8601.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: DateTimeYYYYXMMXDDXHHXMMXSSString = "2020-07-30T07:50:00".into(); // NASA launches Mars Perseverance Rover (EDT)
/// let x: DateTimeYYYYXMMXDDXHHXMMXSSString = "0000-00-00T00:00:00".into(); // Start of common era
/// ```
pub type DateTimeYYYYXMMXDDXHHXMMXSSString = String;

/// Time zone format as "+HH:MM" as `str`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: &ZoneHHXMMStr = "+00:00"; // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x: &ZoneHHXMMStr = "-05:00"; // Eastern Daylight Time (EDT)
/// ```
pub type ZoneHHXMMStr = str;

/// Time zone format as "+HH:MM" as `String`.
///
/// Examples:
/// ```
/// # use ::typeables::datetime::*;
/// let x: ZoneHHXMMString = "+00:00".into(); // UTC a.k.a. GMT a.k.a. Zulu time zone.
/// let x: ZoneHHXMMString = "-05:00".into(); // Eastern Daylight Time (EDT)
/// ```
pub type ZoneHHXMMString = String;
