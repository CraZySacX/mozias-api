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
use crate::routes::{auth, system};
use rocket::routes;
use rocket_contrib::serve::StaticFiles;

crate fn run() -> MoziasApiResult<()> {
    Err(rocket::ignite()
        .manage(db::init_pool()?)
        .mount("/", StaticFiles::from("static"))
        .mount("/api/v1", routes![system::healthcheck, auth::auth])
        .launch()
        .into())
}
