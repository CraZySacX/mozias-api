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
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use lazy_static::lazy_static;
use mysql::prelude::FromRow;
use mysql::{from_row_opt, OptsBuilder, Pool, Row, Transaction};
use std::env;

crate mod auth;
crate mod telemetry;

lazy_static! {
    static ref POOL: MoziasApiResult<Pool> = {
        let mut opts = OptsBuilder::new();
        let _ = opts.ip_or_hostname(Some(env::var("MOZIASDB_HOST")?));
        let _ = opts.db_name(Some(env::var("MOZIASDB_DB")?));
        let _ = opts.user(Some(env::var("MOZIASDB_USERNAME")?));
        let _ = opts.pass(Some(env::var("MOZIASDB_PASSWORD")?));
        let _ = opts.tcp_keepalive_time_ms(Some(1000));

        Ok(Pool::new(opts)?)
    };
    static ref LAST_INSERT_ID: &'static str = r#"SELECT LAST_INSERT_ID()"#;
}

crate fn get_pool() -> MoziasApiResult<Pool> {
    match &(*POOL) {
        Ok(pool) => Ok(pool.clone()),
        Err(_e) => Err("cannot get pool".into()),
    }
}

crate fn start_txn<'a>() -> MoziasApiResult<Transaction<'a>> {
    match &(*POOL) {
        Ok(pool) => Ok(pool.start_transaction(false, None, None)?),
        Err(_e) => Err("cannot start txn".into()),
    }
}

crate fn result_filter<T>(result: Result<Row, mysql::Error>) -> Option<T>
where
    T: FromRow,
{
    if let Ok(row) = result {
        if let Ok(typ) = from_row_opt::<T>(row) {
            Some(typ)
        } else {
            None
        }
    } else {
        None
    }
}

#[allow(dead_code)]
crate fn last_insert_id() -> MoziasApiResult<u64> {
    println!("Getting Last Inserted ID");
    let pool = get_pool()?;
    Ok(*pool
        .prep_exec(*LAST_INSERT_ID, ())?
        .filter_map(result_filter)
        .collect::<Vec<u64>>()
        .first()
        .ok_or_else(|| MoziasApiErrKind::NoInsertId)?)
}
