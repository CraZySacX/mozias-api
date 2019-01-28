// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Authentication Models
//!
//! ```
//! ```
use getset::{Getters, Setters};
use serde_derive::{Deserialize, Serialize};

/// Authentication struct
#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
crate struct Credentials {
    /// User email address
    #[get = "pub"]
    username: String,
    /// User password
    #[get = "pub"]
    password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Setters)]
crate struct TokenResponse {
    #[set = "pub"]
    refresh_token: String,
}
