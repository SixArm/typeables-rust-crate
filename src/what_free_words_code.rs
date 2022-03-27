//! What Free Words Code
//!
//! WhatFreeWords.com code for geolocation.
//!
//! See https://whatfreewords.org/
//!
//! Example:
//!
//!   * "joyful.nail.harmonica" is geolocation 37.234328,-115.806657.
//!
//! ```
//! # use ::typeables::what_free_words_code::*;
//! let x = WhatFreeWordsCodeAsStructStr("joyful.nail.harmonica");
//! ```

pub struct WhatFreeWordsCodeAsStructStr(pub &'static str);
pub struct WhatFreeWordsCodeAsStructString(pub String);

pub type WhatFreeWordsCodeAsTypeStr = str;
pub type WhatFreeWordsCodeAsTypeString = String;
