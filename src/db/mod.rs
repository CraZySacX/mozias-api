// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Database
//!
//! ```
//! ```
use crate::error::MoziasApiResult;
use mysql::{OptsBuilder, Pool};
use std::env;

crate mod auth;

/// initialize the database pool
crate fn init_pool() -> MoziasApiResult<Pool> {
    let mut opts = OptsBuilder::new();
    let _ = opts.ip_or_hostname(Some(env::var("MOZIASDB_HOST")?));
    let _ = opts.db_name(Some(env::var("MOZIASDB_DB")?));
    let _ = opts.user(Some(env::var("MOZIASDB_USERNAME")?));
    let _ = opts.pass(Some(env::var("MOZIASDB_PASSWORD")?));

    Ok(Pool::new(opts)?)
}