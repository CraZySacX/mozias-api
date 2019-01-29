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
use crate::error::MoziasApiResult;
use lazy_static::lazy_static;
use mysql::{from_row_opt, Pool, Row};

lazy_static! {
    static ref USER_AUTH_QUERY: &'static str = r#"
SELECT id, password, refresh_token
FROM mozias_user as user
LEFT JOIN mozias_user_profile as profile on user.id = profile.user_id
WHERE user.username = ?"#;
}

fn result_filter(result: Result<Row, mysql::Error>) -> Option<(String, String, Option<String>)> {
    if let Ok(row) = result {
        if let Ok((id, password, refresh_token)) = from_row_opt(row) {
            Some((id, password, refresh_token))
        } else {
            None
        }
    } else {
        None
    }
}

crate fn auth_info_by_username(
    pool: &Pool,
    username: &str,
) -> MoziasApiResult<Vec<(String, String, Option<String>)>> {
    Ok(pool
        .prep_exec(*USER_AUTH_QUERY, (&username,))?
        .filter_map(result_filter)
        .collect())
}
