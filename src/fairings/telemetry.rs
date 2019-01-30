// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Telemetry Fairing
//!
//! ```
//! ```
use crate::db;
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Data, Request, Response};
use std::time::Instant;
use uuid::Uuid;

const MOZIAS_UUID_HEADER: &str = "x-request-id";

#[derive(Clone, Default)]
crate struct Telemetry {
    start: Option<Instant>,
}

impl Telemetry {
    fn request(&self, req: &mut Request<'_>, _: &Data) -> MoziasApiResult<()> {
        let now = Instant::now();
        let _ = req.local_cache(|| Self { start: Some(now) });
        Ok(())
    }

    fn response(&self, req: &Request<'_>, _: &mut Response<'_>) -> MoziasApiResult<()> {
        let uuid_header = req
            .headers()
            .get_one(MOZIAS_UUID_HEADER)
            .ok_or_else(|| MoziasApiErrKind::Header)?;
        let uuid = Uuid::parse_str(uuid_header)?;
        let uuid_str = uuid.to_hyphenated().to_string();

        // Pull data off request
        let method = req.method().to_string();
        let uri = req.uri().path();
        let remote = req.remote().map(|r| r.to_string());
        let real_ip = req.real_ip().map(|r| r.to_string());
        let headers: Vec<Header<'_>> = req.headers().iter().map(|h| h).collect();

        let telemetry = req.local_cache(|| Self { start: None });

        let elapsed = if let Some(duration) = telemetry.start.map(|st| st.elapsed()) {
            duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
        } else {
            0
        };

        let mut txn = db::start_txn()?;
        let last_insert_id = db::telemetry::insert_telemetry(
            &mut txn, &uuid_str, &method, &uri, &remote, &real_ip, elapsed,
        )?;
        match db::telemetry::insert_headers(&mut txn, last_insert_id, &headers) {
            Ok(_) => {
                txn.commit()?;
            }
            Err(e) => {
                eprintln!("{}", e);
                txn.rollback()?;
            }
        }

        Ok(())
    }
}

impl Fairing for Telemetry {
    fn info(&self) -> Info {
        Info {
            name: "Atlas Telemetry",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, req: &mut Request<'_>, data: &Data) {
        let _ = self.request(req, data);
    }

    fn on_response(&self, req: &Request<'_>, res: &mut Response<'_>) {
        let _ = self.response(req, res);
    }
}
