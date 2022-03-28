//! International Standard of Industrial Classification, Revision 4, Name
//!
//! International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4, Name.
//!
//! Examples:
//!
//!   * A: Agriculture, forestry and fishing
//!   * B05: Mining of coal and lignite
//!   * C101: Processing and preserving of meat
//!
//! Examples:
//! ```
//! # use typeables::international_standard_of_industrial_classification_revision_4_name::*;
//! let x = InternationalStandardOfIndustricalClassificationRevision4NameAsStructStr("Agriculture, forestry and fishing"); // Code A
//! let x = InternationalStandardOfIndustricalClassificationRevision4NameAsStructStr("Mining of coal and lignite"); // Code B05
//! let x = InternationalStandardOfIndustricalClassificationRevision4NameAsStructStr("Processing and preserving of meat"); // Code C101
//! ```

pub struct InternationalStandardOfIndustricalClassificationRevision4NameAsStructStr(pub &'static str);
pub struct InternationalStandardOfIndustricalClassificationRevision4NameAsStructString(pub String);

pub type InternationalStandardOfIndustricalClassificationRevision4NameAsTypeStr = str;
pub type InternationalStandardOfIndustricalClassificationRevision4NameAsTypeString = String;
