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
use std::error::Error;
use std::fmt;

/// A result that includes a `mussh::Error`
crate type DataqResult<T> = Result<T, DataqErr>;

/// An error thrown by the mussh library
#[derive(Debug)]
crate struct DataqErr {
    /// The kind of error
    inner: DataqErrKind,
}

impl Error for DataqErr {
    fn description(&self) -> &str {
        "DataQ Error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl fmt::Display for DataqErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())?;

        if let Some(source) = self.source() {
            write!(f, ": {}", source)?;
        }
        write!(f, "")
    }
}

macro_rules! external_error {
    ($error:ty, $kind:expr) => {
        impl From<$error> for DataqErr {
            fn from(inner: $error) -> Self {
                Self {
                    inner: $kind(inner),
                }
            }
        }
    };
}

impl From<DataqErrKind> for DataqErr {
    fn from(inner: DataqErrKind) -> Self {
        Self { inner }
    }
}

impl From<&str> for DataqErr {
    fn from(inner: &str) -> Self {
        Self {
            inner: DataqErrKind::Str(inner.to_string()),
        }
    }
}

external_error!(clap::Error, DataqErrKind::Clap);
external_error!(std::io::Error, DataqErrKind::Io);
external_error!(String, DataqErrKind::Str);
external_error!(rocket::error::LaunchError, DataqErrKind::Launch);

#[derive(Debug)]
crate enum DataqErrKind {
    Clap(clap::Error),
    Io(std::io::Error),
    Launch(rocket::error::LaunchError),
    Str(String),
}

impl Error for DataqErrKind {
    fn description(&self) -> &str {
        match self {
            DataqErrKind::Clap(inner) => inner.description(),
            DataqErrKind::Io(inner) => inner.description(),
            DataqErrKind::Launch(inner) => inner.description(),
            DataqErrKind::Str(inner) => &inner[..],
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DataqErrKind::Clap(inner) => inner.source(),
            DataqErrKind::Io(inner) => inner.source(),
            DataqErrKind::Launch(inner) => inner.source(),
            _ => None,
        }
    }
}

impl fmt::Display for DataqErrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
