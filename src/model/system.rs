// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! System Models
//!
//! ```
//! ```
use getset::Setters;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Setters)]
crate struct Health<'a> {
    #[set = "pub"]
    status: &'a str,
}

impl Default for Health<'_> {
    fn default() -> Self {
        Self { status: "ok" }
    }
}
