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

const MOZIAS_UUID_HEADER: &str = "x-mozias-uuid";

#[derive(Clone, Default)]
crate struct Telemetry {
    start: Option<Instant>,
    uuid: Option<Uuid>,
}

impl Telemetry {
    fn request(&self, req: &mut Request<'_>, _: &Data) -> MoziasApiResult<()> {
        let uuid = Uuid::new_v4();
        let header = Header::new(MOZIAS_UUID_HEADER, uuid.to_hyphenated().to_string());
        let now = Instant::now();
        req.add_header(header);
        let _ = req.local_cache(|| Self {
            start: Some(now),
            uuid: Some(uuid),
        });
        Ok(())
    }

    fn response(&self, req: &Request<'_>, _: &mut Response<'_>) -> MoziasApiResult<()> {
        let uuid_header = req
            .headers()
            .get_one(MOZIAS_UUID_HEADER)
            .ok_or_else(|| MoziasApiErrKind::Header)?;
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let header_uuid = Uuid::parse_str(uuid_header)?;
        let telemetry = req.local_cache(|| Self {
            start: None,
            uuid: None,
        });

        let uuid = if let Some(uuid) = telemetry.uuid.map(|u| u) {
            if header_uuid == uuid {
                uuid.to_hyphenated().to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        };

        let elapsed = if let Some(duration) = telemetry.start.map(|st| st.elapsed()) {
            duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
        } else {
            0
        };

        db::telemetry::insert_telemetry(&uuid, &method, &uri, elapsed)?;

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
