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
use crate::db::auth as db;
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use crate::model::auth::{Claims, Credentials, TokenResponse, ISSUER};
use jsonwebtoken::{Algorithm, Header};
use mysql::Pool;
use rocket::{post, State};
use rocket_contrib::json::Json;
use std::env;

#[post("/auth/token", data = "<auth>", format = "application/json")]
#[allow(clippy::needless_pass_by_value)]
crate fn auth(
    pool: State<'_, Pool>,
    auth: Json<Credentials>,
) -> MoziasApiResult<Json<TokenResponse>> {
    let username = auth.username();
    let given_password = auth.password();

    println!("Got auth request for '{}'", username);
    let user_vec = db::auth_info_by_username(&*pool, &username)?;

    if user_vec.len() == 1 {
        let secret_key = env::var("ARGON2_SECRET_KEY")?;
        let secret_bytes = secret_key.as_bytes();
        let id = &user_vec[0].0;
        let password = &user_vec[0].1;
        let refresh_tok_opt = &user_vec[0].2;

        if argon2::verify_encoded_ext(password, given_password.as_bytes(), secret_bytes, &[])? {
            let mut token_response = TokenResponse::default();
            let _ = token_response.set_refresh_token(if let Some(refresh_tok) = refresh_tok_opt {
                refresh_tok.clone()
            } else {
                // create a new refresh token and store it
                let mut claims = Claims::default();
                let _ = claims.set_iss(ISSUER.to_string());
                let _ = claims.set_sub(username.clone());
                let _ = claims.set_aid(id.clone());
                let _ = claims.set_tfa(false);
                // if let Ok(roles) = role::find_roles_by_user_id(&pool, id) {
                //     claims.set_rol(roles);
                // }

                let mut header = Header::default();
                header.alg = Algorithm::HS512;
                let secret = env::var("JWT_SECRET")?;
                jsonwebtoken::encode(&header, &claims, secret.as_bytes())?
            });
            Ok(Json(token_response))
        } else {
            Err(MoziasApiErrKind::Unauthorized.into())
        }
    } else {
        Err(MoziasApiErrKind::Unauthorized.into())
    }
}
