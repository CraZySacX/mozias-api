// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! CORS Fairing
//!
//! ```
//! ```
use getset::{Getters, Setters};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Request, Response};
use std::io::Cursor;

/// CORS options for dataq-api
#[derive(Clone, Debug, Default, Eq, Getters, PartialEq, Setters)]
crate struct Cors {
    #[get]
    #[set = "pub"]
    request_uris: Vec<String>,
    #[get]
    #[set = "pub"]
    allowed_origins: Vec<String>,
}

impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) {
        println!("Path: {}", request.uri().path().to_string());
        if self
            .request_uris
            .contains(&request.uri().path().to_string())
        {
            println!("WE HAVE A MATCH");
            let _ = response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                "http://localhost",
            ));
            let _ = response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                "http://localhost:8080",
            ));
            let _ = response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "DELETE, GET, HEAD, PATCH, POST, PUT, OPTIONS",
            ));
            let _ = response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Authorization, Content-Type",
            ));
            let _ = response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

            if request.method() == Method::Options {
                response.set_status(Status::Ok);
                let _ = response.set_header(ContentType::Plain);
                response.set_sized_body(Cursor::new(""));
            }
        }
    }
}
