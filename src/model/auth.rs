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
use chrono::{NaiveDateTime, Utc};
use getset::{Getters, Setters};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

crate const SECONDS_PER_MINUTE: i64 = 60;
crate const SECONDS_PER_HOUR: i64 = SECONDS_PER_MINUTE * 60;
crate const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;
crate const SECONDS_PER_YEAR: i64 = SECONDS_PER_DAY * 365;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref ISSUER: String = {
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    };
    #[derive(Copy, Clone, Debug)]
    pub static ref URL_ENC_ISSUER: String = {
        format!("{}%20{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    };
}

/// User
#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
crate struct User {
    #[get = "pub"]
    #[set = "pub"]
    id: String,
    #[get = "pub"]
    #[set = "pub"]
    username: String,
    #[get = "pub"]
    #[set = "pub"]
    password: String,
    #[get = "pub"]
    #[set = "pub"]
    name: String,
    #[get = "pub"]
    #[set = "pub"]
    disabled: bool,
    #[get = "pub"]
    #[set = "pub"]
    created_by: String,
    #[get = "pub"]
    #[set = "pub"]
    created_date: NaiveDateTime,
    #[get = "pub"]
    #[set = "pub"]
    last_modified_by: String,
    #[get = "pub"]
    #[set = "pub"]
    last_modified_date: NaiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
            password: String::new(),
            name: String::new(),
            disabled: false,
            created_by: String::new(),
            created_date: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            last_modified_by: String::new(),
            last_modified_date: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        }
    }
}

/// User
#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize, Setters)]
crate struct UserProfile {
    #[get = "pub"]
    #[set = "pub"]
    id: String,
    #[get = "pub"]
    #[set = "pub"]
    user_id: String,
    #[get = "pub"]
    #[set = "pub"]
    refresh_token: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    created_by: String,
    #[get = "pub"]
    #[set = "pub"]
    created_date: NaiveDateTime,
    #[get = "pub"]
    #[set = "pub"]
    last_modified_by: String,
    #[get = "pub"]
    #[set = "pub"]
    last_modified_date: NaiveDateTime,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            id: String::new(),
            user_id: String::new(),
            refresh_token: None,
            created_by: String::new(),
            created_date: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            last_modified_by: String::new(),
            last_modified_date: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        }
    }
}

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

#[derive(Clone, Debug, Deserialize, Getters, Serialize, Setters)]
crate struct Claims {
    #[set = "pub"]
    iss: String,
    #[get = "pub"]
    #[set = "pub"]
    sub: String,
    iat: i64,
    nbf: i64,
    #[set = "pub"]
    exp: i64,
    // Atlas User ID
    #[set = "pub"]
    aid: String,
    // Is Two-Factor Authentication required?
    #[set = "pub"]
    tfa: bool,
    // // Atlas User Roles
    // #[get = "pub"]
    // #[set = "pub"]
    // rol: Vec<Role>,
}

impl Default for Claims {
    fn default() -> Self {
        let now = Utc::now().timestamp();
        // Set expiration 5 minutes from now.
        let exp = now + SECONDS_PER_MINUTE * 5;
        Self {
            iss: String::new(),
            sub: String::new(),
            iat: now,
            nbf: now,
            exp,
            aid: String::new(),
            tfa: false,
            // rol: Vec::new(),
        }
    }
}
