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
use getset::{Getters, Setters};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Header};
use rocket::{Data, Request, Response};
use std::fmt;
use std::time::Instant;
use uuid::Uuid;

const MOZIAS_UUID_HEADER: &str = "x-request-id";

#[derive(Clone, Copy)]
crate enum DirectionType {
    Request,
    Response,
}

impl fmt::Display for DirectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirectionType::Request => "REQ",
                DirectionType::Response => "RESPONSE",
            }
        )
    }
}

#[derive(Clone, Default, Getters, Setters)]
crate struct Telemetry {
    #[get = "crate"]
    #[set]
    start: Option<Instant>,
    #[get = "crate"]
    #[set]
    uuid: String,
    #[get = "crate"]
    #[set]
    method: String,
    #[get = "crate"]
    #[set]
    uri: String,
    #[get = "crate"]
    #[set]
    remote: Option<String>,
    #[get = "crate"]
    #[set]
    real_ip: Option<String>,
    #[get = "crate"]
    #[set]
    status: u16,
    #[get = "crate"]
    #[set]
    content_type: Option<String>,
}

impl Telemetry {
    fn request(&self, req: &mut Request<'_>, _: &Data) -> MoziasApiResult<()> {
        let now = Instant::now();
        let mut telemetry = Self::default();
        telemetry.start = Some(now);
        let _ = req.local_cache(|| telemetry);
        Ok(())
    }

    fn response(&self, req: &Request<'_>, resp: &mut Response<'_>) -> MoziasApiResult<()> {
        let uuid_header = req
            .headers()
            .get_one(MOZIAS_UUID_HEADER)
            .ok_or_else(|| MoziasApiErrKind::Header)?;
        let uuid = Uuid::parse_str(uuid_header)?;
        let uuid_str = uuid.to_hyphenated().to_string();

        // Pull data off request
        let method = req.method().to_string();
        let uri = req.uri().path().to_string();
        let remote = req.remote().map(|r| r.to_string());
        let real_ip = req.real_ip().map(|r| r.to_string());
        let req_headers: Vec<Header<'_>> = req.headers().iter().map(|h| h).collect();
        let req_cookies: Vec<Cookie<'_>> = req.cookies().iter().cloned().collect();

        // Pull data off response
        let status = resp.status().code;
        let content_type = resp.content_type().map(|ct| ct.to_string());
        let resp_headers: Vec<Header<'_>> = resp.headers().iter().map(|h| h).collect();
        let resp_cookies: Vec<Cookie<'_>> = resp.cookies().to_vec();

        // Grab the request local telemetry and enhance with info for persistence
        let orig_telemetry = req.local_cache(Self::default);

        let mut telemetry = orig_telemetry.clone();
        let _ = telemetry.set_uuid(uuid_str);
        let _ = telemetry.set_method(method);
        let _ = telemetry.set_uri(uri);
        let _ = telemetry.set_remote(remote);
        let _ = telemetry.set_real_ip(real_ip);
        let _ = telemetry.set_status(status);
        let _ = telemetry.set_content_type(content_type);

        let elapsed = if let Some(duration) = telemetry.start.map(|st| st.elapsed()) {
            duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
        } else {
            0
        };

        let mut txn = db::start_txn()?;

        match db::telemetry::insert_telemetry(&mut txn, &telemetry, elapsed).and_then(
            |last_insert_id| {
                db::telemetry::insert_headers(
                    &mut txn,
                    last_insert_id,
                    DirectionType::Request,
                    &req_headers,
                )
                .and_then(|_| {
                    db::telemetry::insert_cookies(
                        &mut txn,
                        last_insert_id,
                        DirectionType::Request,
                        &req_cookies,
                    )
                })
                .and_then(|_| {
                    db::telemetry::insert_headers(
                        &mut txn,
                        last_insert_id,
                        DirectionType::Response,
                        &resp_headers,
                    )
                })
                .and_then(|_| {
                    db::telemetry::insert_cookies(
                        &mut txn,
                        last_insert_id,
                        DirectionType::Response,
                        &resp_cookies,
                    )
                })
            },
        ) {
            Ok(_) => txn.commit()?,
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
