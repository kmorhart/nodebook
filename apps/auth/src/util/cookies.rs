use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use time::OffsetDateTime;

use crate::models::domain::{TokenPairPublic};

pub async fn create_cookies(jar: CookieJar, pair: TokenPairPublic) -> CookieJar {
    let access_expiry = OffsetDateTime::from_unix_timestamp(pair.access_token.expires_at.timestamp()).unwrap();
    let refresh_expiry = OffsetDateTime::from_unix_timestamp(pair.refresh_token.expires_at.timestamp()).unwrap();

    let access_cookie = Cookie::build(("access_token", pair.access_token.token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .expires(access_expiry)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", pair.refresh_token.token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .expires(refresh_expiry)
        .build();

    jar.add(access_cookie).add(refresh_cookie)
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