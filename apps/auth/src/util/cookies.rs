use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::models::domain::{Token, TokenPairPublic};

pub async fn create_cookies(jar: CookieJar, pair: &TokenPairPublic) -> CookieJar {
    let access_cookie = Cookie::build(("access_token", pair.access_token.token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", pair.refresh_token.token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    jar.add(access_cookie).add(refresh_cookie)
}

pub async fn create_refresh_cookie(jar: CookieJar, pair: &Token) -> CookieJar {
    let refresh_cookie = Cookie::build(("refresh_token", pair.token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    jar.add(refresh_cookie)
}

pub async fn remove_cookies(jar: CookieJar) -> CookieJar {
    let access_cookie = Cookie::build(("access_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    jar.remove(access_cookie).remove(refresh_cookie)
}