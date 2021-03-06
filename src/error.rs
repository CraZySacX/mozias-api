// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Error Handling
//!
//! ```
//! ```
use getset::Setters;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::io::Cursor;

/// A result that includes a `mussh::Error`
crate type MoziasApiResult<T> = Result<T, MoziasApiErr>;

/// An error response
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Setters)]
crate struct ErrorResponse {
    #[set = "pub"]
    message: String,
}

/// An error thrown by the mussh library
#[derive(Debug)]
crate struct MoziasApiErr {
    /// The kind of error
    inner: MoziasApiErrKind,
}

impl Error for MoziasApiErr {
    fn description(&self) -> &str {
        "MoziasApi Error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl fmt::Display for MoziasApiErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())?;

        if let Some(source) = self.inner.source() {
            write!(f, ": {}", source)?;
        }
        write!(f, ": {}", self.inner)
    }
}

impl<'r> Responder<'r> for MoziasApiErr {
    fn respond_to(self, _: &Request<'_>) -> response::Result<'r> {
        let status = match self.inner {
            MoziasApiErrKind::Unauthorized => Status::Unauthorized,
            _ => Status::InternalServerError,
        };

        let mut err_response = ErrorResponse::default();
        let _ = err_response.set_message(self.inner.description().to_string());
        let err_json = json!(err_response);

        Response::build()
            .status(status)
            .sized_body(Cursor::new(err_json.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

macro_rules! external_error {
    ($error:ty, $kind:expr) => {
        impl From<$error> for MoziasApiErr {
            fn from(inner: $error) -> Self {
                Self {
                    inner: $kind(inner),
                }
            }
        }
    };
}

impl From<MoziasApiErrKind> for MoziasApiErr {
    fn from(inner: MoziasApiErrKind) -> Self {
        Self { inner }
    }
}

impl From<&str> for MoziasApiErr {
    fn from(inner: &str) -> Self {
        Self {
            inner: MoziasApiErrKind::Str(inner.to_string()),
        }
    }
}

external_error!(argon2::Error, MoziasApiErrKind::Argon2);
external_error!(clap::Error, MoziasApiErrKind::Clap);
external_error!(std::io::Error, MoziasApiErrKind::Io);
external_error!(jsonwebtoken::errors::Error, MoziasApiErrKind::JsonWebToken);
external_error!(rocket::error::LaunchError, MoziasApiErrKind::Launch);
external_error!(mysql::Error, MoziasApiErrKind::Mysql);
external_error!(String, MoziasApiErrKind::Str);
external_error!(uuid::Error, MoziasApiErrKind::UuidParse);
external_error!(std::env::VarError, MoziasApiErrKind::Var);

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
#[allow(variant_size_differences)]
crate enum MoziasApiErrKind {
    Argon2(argon2::Error),
    Clap(clap::Error),
    Header,
    InsertFailed,
    Io(std::io::Error),
    JsonWebToken(jsonwebtoken::errors::Error),
    Launch(rocket::error::LaunchError),
    Mysql(mysql::Error),
    NoInsertId,
    Str(String),
    Unauthorized,
    UuidParse(uuid::Error),
    Var(std::env::VarError),
}

impl Error for MoziasApiErrKind {
    fn description(&self) -> &str {
        match self {
            Self::Argon2(inner) => inner.description(),
            Self::Clap(inner) => inner.description(),
            Self::Header => "invalid header",
            Self::InsertFailed => "insert failed",
            Self::Io(inner) => inner.description(),
            Self::JsonWebToken(inner) => inner.description(),
            Self::Launch(inner) => inner.description(),
            Self::Mysql(inner) => inner.description(),
            Self::NoInsertId => "no insert id found",
            Self::Str(inner) => &inner[..],
            Self::Unauthorized => "unauthorized",
            Self::UuidParse(inner) => inner.description(),
            Self::Var(inner) => inner.description(),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Argon2(inner) => inner.source(),
            Self::Clap(inner) => inner.source(),
            Self::Io(inner) => inner.source(),
            Self::Launch(inner) => inner.source(),
            Self::Mysql(inner) => inner.source(),
            Self::UuidParse(inner) => inner.source(),
            Self::Var(inner) => inner.source(),
            _ => None,
        }
    }
}

impl fmt::Display for MoziasApiErrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())?;
        match self {
            Self::Argon2(inner) => write!(f, ": {}", inner),
            Self::Clap(inner) => write!(f, ": {}", inner),
            Self::Io(inner) => write!(f, ": {}", inner),
            Self::Launch(inner) => write!(f, ": {}", inner),
            Self::Mysql(inner) => write!(f, ": {}", inner),
            Self::UuidParse(inner) => write!(f, ": {}", inner),
            Self::Var(inner) => write!(f, ": {}", inner),
            _ => write!(f, ""),
        }
    }
}
