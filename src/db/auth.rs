// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Authentication Database Access
//!
//! ```
//! ```
use crate::db::result_filter;
use crate::error::{MoziasApiErrKind, MoziasApiResult};
use lazy_static::lazy_static;
use mysql::{params, Pool};

lazy_static! {
    static ref USER_AUTH_QUERY: &'static str = r#"
SELECT user.id, profile.id as profile_id, password, refresh_token
FROM mozias_user as user
LEFT JOIN mozias_user_profile as profile on user.id = profile.user_id
WHERE user.username = :username"#;
    static ref INSERT_REFRESH_TOKEN: &'static str = r#"
UPDATE mozias_user_profile
SET refresh_token = :refresh_token
WHERE id = :profile_id"#;
}

type AuthQueryResult = (String, String, String, Option<String>);

crate fn auth_info_by_username(
    pool: &Pool,
    username: &str,
) -> MoziasApiResult<Vec<AuthQueryResult>> {
    Ok(pool
        .prep_exec(*USER_AUTH_QUERY, params! {"username" => &username})?
        .filter_map(result_filter)
        .collect())
}

crate fn add_refresh_token_to_profile(
    pool: &Pool,
    profile_id: &str,
    refresh_token: &str,
) -> MoziasApiResult<()> {
    match pool.prepare(*INSERT_REFRESH_TOKEN) {
        Ok(mut stmt) => {
            let result = stmt.execute(params! {
                "refresh_token" => refresh_token,
                "profile_id" => profile_id,
            })?;

            if result.affected_rows() != 1 {
                return Err(MoziasApiErrKind::InsertFailed.into());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e.into())
        }
    }
}
