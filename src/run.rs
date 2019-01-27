// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Runtime
//!
//! ```
//! ```
use crate::db;
use crate::error::MoziasApiResult;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Health<'a> {
    status: &'a str,
}

#[get("/healthcheck")]
fn healthcheck<'a>() -> Json<Health<'a>> {
    Json(Health { status: "ok" })
}

#[get("/auth")]
fn auth() -> &'static str {
    "ok"
}

crate fn run() -> MoziasApiResult<()> {
    Err(rocket::ignite()
        .manage(db::init_pool()?)
        .mount("/", StaticFiles::from("static"))
        .mount("/api/v1", routes![healthcheck, auth])
        .launch()
        .into())
}
