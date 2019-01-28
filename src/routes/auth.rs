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
use crate::model::auth::{Credentials, TokenResponse};
use lazy_static::lazy_static;
use mysql::{from_row_opt, Pool, Row};
use rocket::{post, State};
use rocket_contrib::json::Json;
use std::env;
use std::result::Result;

lazy_static! {
    static ref USER_AUTH_QUERY: &'static str = r#"
SELECT password, refresh_token
FROM mozias_user user
LEFT JOIN mozias_user_profile profile on user.id = profile.user_id
WHERE user.username = ?"#;
}

fn result_filter(result: Result<Row, mysql::Error>) -> Option<(String, Option<String>)> {
    if let Ok(row) = result {
        if let Ok((password, refresh_token)) = from_row_opt(row) {
            Some((password, refresh_token))
        } else {
            None
        }
    } else {
        None
    }
}

#[post("/auth/token", data = "<auth>", format = "application/json")]
#[allow(clippy::needless_pass_by_value)]
crate fn auth(
    pool: State<'_, Pool>,
    auth: Json<Credentials>,
) -> MoziasApiResult<Json<TokenResponse>> {
    let username = auth.username();
    let password = auth.password();

    println!("Got auth request for '{}'", username);
    let pass_vec: Vec<(String, Option<String>)> = pool
        .prep_exec(*USER_AUTH_QUERY, (&username,))?
        .filter_map(result_filter)
        .collect();

    if pass_vec.len() == 1 {
        let secret_key = env::var("ARGON2_SECRET_KEY")?;
        let secret_bytes = secret_key.as_bytes();

        if argon2::verify_encoded_ext(&pass_vec[0].0, password.as_bytes(), secret_bytes, &[])? {
            let mut token_response = TokenResponse::default();
            let _ = token_response.set_refresh_token(if let Some(refresh_tok) = &pass_vec[0].1 {
                refresh_tok.clone()
            } else {
                // create a new refresh token and store it
                "invalid token".to_string()
            });
            Ok(Json(token_response))
        } else {
            Err(MoziasApiErrKind::Unauthorized.into())
        }
    } else {
        Err(MoziasApiErrKind::Unauthorized.into())
    }
}
