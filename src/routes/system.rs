// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! System Routes
//!
//! ```
//! ```
use crate::model::system::Health;
use rocket::get;
use rocket_contrib::json::Json;

#[get("/healthcheck")]
crate fn healthcheck<'a>() -> Json<Health<'a>> {
    Json(Health::default())
}
