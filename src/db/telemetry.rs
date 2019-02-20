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
use crate::fairings::telemetry::{DirectionType, Telemetry};
use lazy_static::lazy_static;
use mysql::params;
use mysql::prelude::GenericConnection;
use rocket::http::{Cookie, Header};

lazy_static! {
    static ref INSERT_TELEMETRY: &'static str = r#"
INSERT INTO mozias_telemetry
  (UUID, METHOD, URI, REMOTE, REAL_IP, STATUS, CONTENT_TYPE, ELAPSED)
VALUES
  (:uuid, :method, :uri, :remote, :real_ip, :status, :content_type, :elapsed)
"#;
    static ref INSERT_HEADERS: &'static str = r#"
INSERT INTO mozias_telemetry_headers
  (`telemetry_id`, `header_type`, `key`, `value`)
VALUES
  (:telemetry_id, :header_type, :key, :value)"#;
    static ref INSERT_COOKIES: &'static str = r#"
INSERT INTO mozias_telemetry_cookies
  (`telemetry_id`, `cookie_type`, `key`, `value`)
VALUES
  (:telemetry_id, :cookie_type, :key, :value)"#;
}

crate fn insert_telemetry<T>(
    conn: &mut T,
    telemetry: &Telemetry,
    elapsed: u64,
) -> MoziasApiResult<u64>
where
    T: GenericConnection,
{
    match conn.prepare(*INSERT_TELEMETRY) {
        Ok(mut stmt) => {
            let result = stmt.execute(params! {
                "uuid" => telemetry.uuid(),
                "method" => telemetry.method(),
                "uri" => telemetry.uri(),
                "remote" => telemetry.remote(),
                "real_ip" => telemetry.real_ip(),
                "status" => telemetry.status(),
                "content_type" => telemetry.content_type(),
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
    header_type: DirectionType,
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
                    "header_type" => header_type.to_string(),
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
    cookie_type: DirectionType,
    cookies: &[Cookie<'_>],
) -> MoziasApiResult<()>
where
    T: GenericConnection,
{
    match conn.prepare(*INSERT_COOKIES) {
        Ok(mut stmt) => {
            for cookie in cookies {
                let result = stmt.execute(params! {
                    "telemetry_id" => last_insert_id,
                    "cookie_type" => cookie_type.to_string(),
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
