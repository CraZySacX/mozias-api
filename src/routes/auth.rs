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
use crate::error::MoziasApiResult;
use crate::model::auth::Credentials;
use mysql::{from_row, Pool};
use rocket::{post, State};
use rocket_contrib::json::Json;

#[post("/auth", data = "<auth>", format = "application/json")]
#[allow(clippy::needless_pass_by_value)]
crate fn auth(pool: State<'_, Pool>, auth: Json<Credentials>) -> MoziasApiResult<&'static str> {
    let username = auth.username();
    let _password = auth.password();

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

    for _password in pass_vec {
        println!("Found password for user '{}'", username);
    }
    Ok("ok")
}
