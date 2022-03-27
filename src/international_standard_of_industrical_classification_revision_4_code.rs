//! International Standard of Industrial Classification, Revision 4, Code
//!
//! International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4, Code.
//! 
//! Examples:
//!
//!   * A: Agriculture, forestry and fishing
//!   * B05: Mining of coal and lignite
//!   * C101: Processing and preserving of meat
//!
//! Examples:
//! ```
//! # use ::typeables::international_standard_of_industrical_classification_revision_4_code::*;
//! let x = InternationalStandardOfIndustricalClassificationRevision4CodeAsStructStr("A"); // Agriculture, forestry and fishing
//! let x = InternationalStandardOfIndustricalClassificationRevision4CodeAsStructStr("B05"); // Mining of coal and lignite
//! let x = InternationalStandardOfIndustricalClassificationRevision4CodeAsStructStr("C101"); // Processing and preserving of meat
//! ```

pub struct InternationalStandardOfIndustricalClassificationRevision4CodeAsStructStr(pub &'static str);
pub struct InternationalStandardOfIndustricalClassificationRevision4CodeAsStructString(pub String);

pub type InternationalStandardOfIndustricalClassificationRevision4CodeAsTypeStr = str;
pub type InternationalStandardOfIndustricalClassificationRevision4CodeAsTypeString = String;
