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
use crate::db;
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use lazy_static::lazy_static;
use mysql::params;

lazy_static! {
    static ref INSERT_TELEMETRY: &'static str = r#"
INSERT INTO mozias_telemetry
  (UUID, METHOD, URI, REMOTE, REAL_IP, ELAPSED)
VALUES
  (:uuid, :method, :uri, :remote, :real_ip, :elapsed)
"#;
}

crate fn insert_telemetry(
    uuid: &str,
    method: &str,
    uri: &str,
    remote: &Option<String>,
    real_ip: &Option<String>,
    elapsed: u64,
) -> MoziasApiResult<()> {
    println!("Inserting telemetry data");
    let pool = db::get_pool()?;

    match pool.prepare(*INSERT_TELEMETRY) {
        Ok(mut stmt) => {
            let result = stmt.execute(params! {
                "uuid" => uuid,
                "method" => method,
                "uri" => uri,
                "remote" => remote,
                "real_ip" => real_ip,
                "elapsed" => elapsed,
            })?;

            println!("Result: {}", result.affected_rows());

            if result.affected_rows() != 1 {
                return Err(MoziasApiErrKind::InsertFailed.into());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e.into())
        }
    }
}
