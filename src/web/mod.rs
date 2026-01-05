pub mod mw_auth;
pub mod routes_login;
mod error;
pub mod mw_res_map;
pub mod routes_static;

use tower_cookies::{Cookie, Cookies};
use crate::crypt::token::generate_web_token;
pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token = generate_web_token(user, salt)?;
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    // Important: default path is the URI path of the request (e.g. /api/login).
    // Hence, we need to set it to "/" instead
    cookie.set_path("/");

    // Add the cookie
    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}