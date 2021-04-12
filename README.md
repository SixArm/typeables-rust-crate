# Typeables: Rust crate of type aliases

Typeables is a Rust crate of type aliases, intended to help improve source code self-documenting semantic type names. This helps with literate programming, domain driven design, and developer knowledge.

## Examples

Geolocation example:

```rust
let a: LatitudeDecimalDegreeF64 = 40.75;
let b: LongitudeDecimalDegreeF64 = 73.97;
let c: AltitudeMeanSeaLevelMeterF64 = 56.00;
```

Geolocation example of combining types:

```rust
type Point = (
    LatitudeDecimalDegreeF64,
    LongitudeDecimalDegreeF64,
    AltitudeMeanSeaLevelMeterF64.
);
```

## Purpose

The purpose of this crate is syntax sugar for better readabiliy.

The purpose of this library is not any kind type-based coding, such as data encapsulation, or parameter validation, or object oriented programming. If you want these kinds of aspects, we recommend looking at the crate `uom` (unit of measure) and the Rust book examples of the `newtype` pattern.


## Implementation

The type aliases are all for Rust primitives and standards such as `str` and `String`.

The type aliases are zero-overhead because they are replaced at compile time.
