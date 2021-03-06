# Typeables: Rust crate of type aliases and struct tuples

Typeables is a Rust crate of semantic types, such as unit types (e.g. metre
for length, second for time), content types (e.g. email address, phone
number), locale types (e.g. "en" for English, "zh" for Chinese), etc.

* scientific units
  * ampere
  * becquerel
  * candela
  * degree_celcius
  * farad
  * gram
  * gray
  * hertz
  * henry
  * joule
  * katal
  * kelvin
  * kilogram
  * litre
  * lumen
  * lux
  * metre
  * metre^2
  * metre^3
  * metre_per_second
  * mole
  * ohm
  * pascal
  * radian
  * second
  * siemens
  * sievert
  * steradian
  * tesla
  * volt
  * watt
  * weber
* currency
  * currency_name
  * currency_symbol
* email address
  * email_address
  * email_address_addr
  * email_address_name
* geolocation
  * latitude
  * longitude
  * altitude
  * elevation
  * open_location_code
  * what_freewords_code
* interval
  * unit_interval
  * dual_interval
* locale
  * locale_code
  * locale_language_code
  * locale_country_code
  * locale_region_code
  * locale_script_code
  * locale_variant_code
* localization
  * decimal_separator
  * grouping_separator
  * quotation_start_delimiter
  * quotation_stop_delimiter
* media type
  * media_type_code
  * media_type_supertype
  * media_type_subtype
  * media_type_suffix
  * media_type_parameter
  * media_type_tree
* phone e164
  * phone_e164_text
  * phone_e164_country_code
  * phone_e164_national_destination_code
  * phone_e164_subscriber_number
  * phone_e164_group_identification_code
  * phone_e164_trial_identification_code
* grammar
  * adjective
  * adverb
  * noun
  * pronoun
  * verb
* text formats
  * html_text
  * json_text
  * markdown_text
  * yaml_text
  * xml_text
* content
  * global_location_number
  * international_standard_of_industrial_classification_revision_4_code
  * international_standard_of_industrial_classification_revision_4_name
  * legal_entity_identifier_code
  * value_added_tax_identification_number
* date time
  * year
  * month
  * day
  * hour
  * minute
  * second
  * date
  * time
  * time_offset
  * time_zone
 

## Introduction

Typeables is based on the Rust pattern of "New Type". This uses a Rust struct
tuple as a wrapper of another type (or types), in order to provide
encapsulation.

Example:

```rust
pub struct MetreAsF64(pub f64); // This is a "New Type" struct tuple.

let length = MetreAsF64(1.2); // 1.2 metres as floating-point 64-bit.
```

Typeables helps you write clearer code and stronger code, because you can be
more-precise about your variable types and your function inputs and outputs.

Example to calculate rectangular area:

```rust
pub struct MetreAsF64(pub f64); // Metre which is for distance.
pub struct Metre2AsF64(pub f64); // Metre^2 which is for area.

fn area(length: MetreAsF64, width: MetreAsF64) -> Metre2AsF64 {
   Metre2AsF64(length.0 * width.0)
}
```

Typeables helps you create better domain driven design, stronger
compile-time checking, and crisper run-time diagnostics.


### What is a struct tuple?

A struct tuple is akin to a wrapper for another type such as:

```rust
pub struct Year(pub i16);
```

A struct tuple can make your code safer because it provides encapsulation:

```rust
# pub struct Year(pub i16);
let x = Year(2022);
```

### What is a type alias?

A type alias is akin to a nickname for another type such as:

```rust
pub type Year = i16;
```

A type alias can make your code clearer because it expresses your intent:

```rust
# pub type Year = i16;
let x: Year = 2022;
```


### What does Typeables do?

Typeables provides many concept types, each implemented as a struct tuple
and a type alias, and

Example variable:

```rust
use ::typeables::year::*;

let x = YearAsStructI16(2022); // Year as struct tuple
```

Example function:

```rust
use ::typeables::year::*;

fn f(x: YearAsStructI16) { // Year as struct tuple
    println!("Year {}", x.0) // Use the zero field
}
```


### How do I refactor code to use Typeables?

Typeables helps you refactor from weaker-type code to stronger-type code.

Suppose you start with typical code:

```rust
fn f(x: i16) {
    println!("{}", x)
}

fn main() {
    let x = 2022;
    f(x)
}
```

Step 1. Refactor to a Typeables type alias. This is annotation.

```rust
use ::typeables::year::*;

fn f(x: YearAsTypeI16) {
    println!("{}", x)
}

fn main() {
    let x = 1 as YearAsTypeI16;
    f(x)
}
```

Step 2. Refactor to a Typealias struct tuple. This is encapsulation.

```rust
use ::typeables::year::*;

fn f(x: YearAsStructI16) {
    println!("{}", x.0)
}

fn main() {
    let x = YearAsStructI16(2022);
    f(x)
}
```


## Semantics


### Semantic examples

Calendar examples:

```rust
# use ::typeables::year::*;
# use ::typeables::month::*;
let year = YearAsStructI16(2022);
let month = MonthAsStructI8(12);
```

Geolocation examples of New York City Grand Central Terminal:

```rust
# use ::typeables::{latitude::*, longitude::*};
let latitude = LatitudeAsDecimalDegreeAsStructF32(40.75);
let longitude = LongitudeAsDecimalDegreeAsStructF32(-73.97);
```

Date-time format examples of the NASA launch of the Mars Perseverance Rover:

```rust
# use ::typeables::datetime::*;
let date_stamp = DateAsYYYYXMMXDDAsStructString(String::from("2020-07-30")); // Year 2020 on July 30th
let time_stamp = TimeAsHHXMMXSSAsStructString(String::from("07:50:00")); // 7:50 in the morning
let offset_stamp = TimeOffsetAsXHHXMMAsStructString(String::from("-05:00")); // 5 hours ahead of UTC
```


### Why use semantic names?

When you use semantic names, such as clear descriptions and purposeful
naming conventions, then you help developers understand your code, and help
compilers provide reliability, and help tools provide inspectability.

Suppose your code has this function:

```rust
fn f(year: i16, month: i16) {
    println!("Year {} Month {}", year, month)
}
```

A developer can use your code like this:

```rust
# use ::typeables::{year::*, month::*};
# fn f(year: i16, month: i16) {
#     println!("Year {} Month {}", year, month)
# }
let year = 2022;
let month = 12;

f(year, month); // right
// f(month, year); // wrong, yet will compile and be a bug
```

You can make your code clearer by adding a type alias:

```rust
# use ::typeables::{year::*, month::*};
fn f(year: YearAsTypeI16, month: MonthAsTypeI16) {
    println!("Year {} Month {}", year, month)
}
```

You can make your code stronger by using a struct tuple:

```rust
# use ::typeables::{year::*, month::*};
fn f(year: YearAsStructI16, month: MonthAsStructI16) {
    println!("Year {} Month {}", year.0, month.0)
}
```

A developer can use your code like this:

```rust
# use ::typeables::{year::*, month::*};
# fn f(year: YearAsStructI16, month: MonthAsStructI16) {
#    println!("Year {} Month {}", year.0, month.0)
# }
let year = YearAsStructI16(2022);
let month = MonthAsStructI16(12);

f(year, month); // right
// f(month, year); // wrong and won't compile
```


### Why use semantic names, representation names, unit names, and implementation names?

Suppose you're writing an application for aircraft.

You want to keep track of:

  * Aircraft altitudes.

  * Representation as "Above Ground Level (AGL)" such as the height of the
    aircraft above the runway during takeoff or landing, or as "Mean Sea
    Level (MSL)" such as the worldwide height of the aircraft during
    cruising flight.

  * Unit of measurement as "Metre" which is the international system, or as
    "Foot" which is the United States system.

  * The implemention as a signed integer 16-bit, because altitude can be
    negative in some rare areas such as Death Valley California, and your
    application may need to integrate with legacy code that requires signed
    integer 16-bit numbers.

You can use this naming convention:

  * Semantic name "Altitude"

  * As representation name "Above Ground Level" or "Mean Sea Level"

  * As unit name "Metre" or "Foot"

  * As primitive name "I16".

The code looks like this:

```rust
# use ::typeables::altitude::*;
pub struct AltitudeAsAboveGroundLevelAsMetreAsStructI16(pub i16);
pub struct AltitudeAsAboveGroundLevelAsFootAsStructI16(pub i16);
pub struct AltitudeAsMeanSeaLevelAsMetreAsStructI16(pub i16);
pub struct AltitudeAsMeanSeaLevelAsFootAsStructI16(pub i16);
```

Suppose your app also needs to keep track of:

  * Airport elevations.

  * The representation as "Above Ground Level (AGL)" such as the height of
    an airport building above the airport runway, or as "Mean Sea Level
    (MSG)" such as the worldwide height of the airporse runway.

  * Etc.

The code looks like this:

```rust
# use ::typeables::elevation::*;
pub struct ElevationAsAboveGroundLevelAsMetreAsStructI16(pub i16);
pub struct ElevationAsAboveGroundLevelAsFootAsStructI16(pub i16);
pub struct ElevationAsMeanSeaLevelAsMetreAsStructI16(pub i16);
pub struct ElevationAsMeanSeaLevelAsFootAsStructI16(pub i16);
```

The naming convention is crystal clear and fully descriptive:

* Developers can understand your code better, and how to use it.

* Compilers can provide stronger compile-time guarantees.

* Debuggers can provide crisper run-time diagnostics.

* Editors can provide better auto-complete and auto-suggest.


### Use words rather than abbreviations

Examples of semantic names:

  * Use "Latitude" not "Lat".

  * Use "Longitude" not "Lon", "Lng", "Long".

Examples of representation names:

  * Use "Decimal Degree" not "DD"

  * Use "Degree Minute Second" not "DMS".

Examples of unit names:

  * Use "Metre" not "M".

  * Use "Second" not "S".

Examples of implementation names:

  * Use "TypeString" not "TS"

  * Use "StructString" not "SS".


### Prefer singular over plural

Examples of representation names:

  * Use "Decimal Degree" not "Decimal Degrees"

  * Use "Degree Minute Second" not "Degrees Minutes Seconds"

Examples of unit names:

  * Use "Metre" not "Metres".

  * Use "Second" not "Seconds"


### Naming conventions

Naming convention for struct tuples:

```rust
pub struct FooAsStructI8(pub i8);
pub struct FooAsStructI16(pub i16);
pub struct FooAsStructI32(pub i32);
pub struct FooAsStructI64(pub i64);
pub struct FooAsStructI128(pub i128);
pub struct FooAsStructISize(pub isize);

pub struct FooAsStructU8(pub u8);
pub struct FooAsStructU16(pub u16);
pub struct FooAsStructU32(pub u32);
pub struct FooAsStructU64(pub u64);
pub struct FooAsStructU128(pub u128);
pub struct FooAsStructUSize(pub usize);

pub struct FooAsStructF32(pub f32);
pub struct FooAsStructF64(pub f64);

pub struct FooAsStructStr(&'static String);
pub struct FooAsStructString(pub String);
```

Naming convention for type aliass:

```rust
pub type FooAsTypeI8 = i8;
pub type FooAsTypeI16 = i16;
pub type FooAsTypeI32 = i32;
pub type FooAsTypeI64 = i64;
pub type FooAsTypeI128 = i128;
pub type FooAsTypeISize = isize;

pub type FooAsTypeU8 = u8;
pub type FooAsTypeU16 = u16;
pub type FooAsTypeU32 = u32;
pub type FooAsTypeU64 = u64;
pub type FooAsTypeU128 = u128;
pub type FooAsTypeUSize = usize;

pub type FooAsTypeF32 = f32;
pub type FooAsTypeF64 = f64;

pub type FooAsTypeStr = str;
pub type FooAsTypeString = String;
```


## Comparisons

We recommend looking at the Rust crate `uom` (unit of measure) and the Rust book
examples of the `newtype` pattern.


### Comparison with uom

Broadly speaking:

* uom favors high-level work, such as automatic normalizations and conversions.

* Typeables favors low-level work, such as exact representations and primitives.

Quantities v. units v. primitives:

* uom deliberately favors working with conceptual quantities (length, mass,
  time, ???) rather than measurement units (metre, gram, second, ???) and
  implementation primitives (pub i8, u16, f32, ???).

* Typeables favors working with explicit measurement units and explicit
  implementation primitives. When you want the concept of "length" and unit
  "metre" and primitive "f32" then you write "LengthAsMetreAsTypeF32".

Normalization v. exactness:

* uom deliberately normalizes values to their base units, such as normalizing 1
  gram to 0.001 kilogram, and deliberately trades away representation
  capabilities (due to inexact conversions) and precision capabilties (due to
  bit limits).

* Typeables favors exactness, never normaliziation. When you want the concept of
  "mass" and unit "gram" and primitive "u128" for 128-bit unsigned integer
  precision, then you write "GramAsTypeI128".


### Comparison with Rust "New Type Idiom" a.k.a. "New Type Pattern"

Broadly speaking:

* The Rust "New Type Idiom" a.k.a. "New Type Pattern" is exactly what Typeables
  is doing with struct tuples. We like this idiom very much.

* Typeables additionally provides type aliass. In practice we find this is an
  important way to help professional developers with larger codebases, because
  the developers can phase in the type aliass as hints to developers and to
  tools, then later on can phase in the struct tuples.

Roll your own versus using Typeables crate:

* You can certainly roll your own new type pattern, and you can use your own
  type names, or even use the Typeables type names.

* The Typeables crate is helpful because it provides a bunch of definitions, so
  you can use the crate, then get all the benefits of the types, plus your tools
  can use the crate information, such as for editor tool autocomplete and
  autosuggest.


## Implementation

The type aliases are all for Rust primitives and standards such as strings
(using `str` and `String`) and numbers (using `i64`, `u64`, `f64`, et al.).


### Overhead

Typeables has zero or near-zero runtime overhead:

* A type alias is zero runtime overhead because the type alias is replaced at compile time.

* A struct tuple is near-zero runtime overhead because the struct tuple is a wrapper with a field.


### Typing

Typeables is deliberately verbose.

* We use editors with autocomplete and autosuggest, so typing is easy and fast.

* We like long names for low-level clarity.

* Typeables defines many type aliass and struct tuples. Typically these are fast
  during development because they're simple. Typically these are even faster
  during production because the Rust compiler can optimized these and also
  eliminate any that are not needed.


### Macros

The Typeables source code does not use macros.

* We like macros in general.

* Yet we discovered in practice that macros seem to interfere with some of our tooling.

* For example, macros do not seem to work with some editors that inspect the
  Typeables crate in order to do autocomplete and autosuggest.
