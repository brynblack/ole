use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod auth;
pub mod courses;
pub mod login;

#[derive(Deserialize, Serialize)]
struct Claims {
    /// Subject (Who the token is given to)
    sub: String,
    /// Expiration time (UTC)
    exp: usize,
}
