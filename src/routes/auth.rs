// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Authentication Routes
//!
//! ```
//! ```
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use crate::model::auth::Credentials;
use argonautica::Verifier;
use mysql::{from_row, Pool};
use rocket::{post, State};
use rocket_contrib::json::Json;
use std::env;

#[post("/auth", data = "<auth>", format = "application/json")]
#[allow(clippy::needless_pass_by_value)]
crate fn auth(pool: State<'_, Pool>, auth: Json<Credentials>) -> MoziasApiResult<Json<bool>> {
    let username = auth.username();
    let password = auth.password();

    println!("Got auth request for '{}'", username);
    let pass_vec: Vec<String> = pool
        .prep_exec(
            "SELECT password FROM mozias_user WHERE username = ?",
            (&username,),
        )?
        .map(|result| {
            if let Ok(row) = result {
                let (password,) = from_row(row);
                password
            } else {
                "not found".to_string()
            }
        })
        .collect();

    if pass_vec.len() == 1 {
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(&pass_vec[0])
            .with_password(password)
            .with_secret_key(env::var("ARGON2_SECRET_KEY")?)
            .verify()?;
        Ok(Json(is_valid))
    } else {
        Err(MoziasApiErrKind::Unauthorized.into())
    }
}
