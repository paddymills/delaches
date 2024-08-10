use std::sync::{Arc, OnceLock};

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use rand::distributions::{Alphanumeric, DistString};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, Key};

use crate::server::AppState;

// log::debug!("{:?}", cookies);
// let key = KEY.get().unwrap();
// let private_cookies = cookies.private(key); // You can use `cookies.signed` as well

const COOKIE_NAME: &str = "delaches_auth";
pub static KEY: OnceLock<Key> = OnceLock::new();

#[derive(Debug)]
pub struct Auth {}

#[axum::async_trait]
impl FromRequestParts<Arc<AppState>> for Auth {
    type Rejection = Response;

    async fn from_request_parts(
        req: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        log::debug!("Auth Parts: {:?}", req);
        log::debug!("Auth State: {:?}", state);

        let cookies = Cookies::from_request_parts(req, state).await.unwrap();
        let state = state.clone();

        // check for authorization token
        if let Some(mut token) = cookies.get(COOKIE_NAME) {
            let token_str = token.value().to_string();

            // validate token
            if state.tokens.lock().await.contains(&token_str) {
                return Ok(Auth {});
            }

            log::debug!("Cookie token {:?} is not recognized", token_str);
            token.make_removal();
        }

        // in the case of the login page, we will ahve an authorization header
        if let Some(code) = req.headers.get(header::AUTHORIZATION) {
            log::debug!("Auth code: {:?}", code);
            if *code == state.auth_code {
                log::debug!("Authenticated!");

                // generate token somewhat random token
                let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
                state.clone().tokens.lock().await.push(token.clone());

                let mut cookie = Cookie::new(COOKIE_NAME, token);
                cookie.set_same_site(SameSite::Strict);
                let _ = cookies.add(cookie);

                return Ok(Auth {});
            }

            // else: return unauthorized
            return Err((StatusCode::UNAUTHORIZED, "invalid authorization code").into_response());
        }

        // if not from the login page, redirect
        Err(Redirect::to("/login").into_response())
    }
}
