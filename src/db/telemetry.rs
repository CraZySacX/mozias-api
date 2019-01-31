// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Telemetry Database
//!
//! ```
//! ```
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use lazy_static::lazy_static;
use mysql::params;
use mysql::prelude::GenericConnection;
use rocket::http::{Cookie, Header};

lazy_static! {
    static ref INSERT_TELEMETRY: &'static str = r#"
INSERT INTO mozias_telemetry
  (UUID, METHOD, URI, REMOTE, REAL_IP, ELAPSED)
VALUES
  (:uuid, :method, :uri, :remote, :real_ip, :elapsed)
"#;
    static ref INSERT_HEADERS: &'static str = r#"
INSERT INTO mozias_telemetry_headers
  (`telemetry_id`, `key`, `value`)
VALUES
  (:telemetry_id, :key, :value)"#;
    static ref INSERT_COOKIES: &'static str = r#"
INSERT INTO mozias_telemetry_cookies
  (`telemetry_id`, `key`, `value`)
VALUES
  (:telemetry_id, :key, :value)"#;
}

crate fn insert_telemetry<T>(
    conn: &mut T,
    uuid: &str,
    method: &str,
    uri: &str,
    remote: &Option<String>,
    real_ip: &Option<String>,
    elapsed: u64,
) -> MoziasApiResult<u64>
where
    T: GenericConnection,
{
    match conn.prepare(*INSERT_TELEMETRY) {
        Ok(mut stmt) => {
            let result = stmt.execute(params! {
                "uuid" => uuid,
                "method" => method,
                "uri" => uri,
                "remote" => remote,
                "real_ip" => real_ip,
                "elapsed" => elapsed,
            })?;

            if result.affected_rows() != 1 {
                return Err(MoziasApiErrKind::InsertFailed.into());
            }
            Ok(result.last_insert_id())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e.into())
        }
    }
}

crate fn insert_headers<T>(
    conn: &mut T,
    last_insert_id: u64,
    headers: &[Header<'_>],
) -> MoziasApiResult<()>
where
    T: GenericConnection,
{
    match conn.prepare(*INSERT_HEADERS) {
        Ok(mut stmt) => {
            for header in headers {
                let result = stmt.execute(params! {
                    "telemetry_id" => last_insert_id,
                    "key" => header.name(),
                    "value" => header.value(),
                })?;

                if result.affected_rows() != 1 {
                    return Err(MoziasApiErrKind::InsertFailed.into());
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e.into())
        }
    }
}

crate fn insert_cookies<T>(
    conn: &mut T,
    last_insert_id: u64,
    cookies: &[Cookie<'_>],
) -> MoziasApiResult<()>
where
    T: GenericConnection,
{
    match conn.prepare(*INSERT_HEADERS) {
        Ok(mut stmt) => {
            for cookie in cookies {
                let result = stmt.execute(params! {
                    "telemetry_id" => last_insert_id,
                    "key" => cookie.name(),
                    "value" => cookie.value(),
                })?;

                if result.affected_rows() != 1 {
                    return Err(MoziasApiErrKind::InsertFailed.into());
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e.into())
        }
    }
}
