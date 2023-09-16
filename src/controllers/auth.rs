use actix_web::{
    get, patch, post,
    web::{scope, Data, Json},
    Responder, Scope,
};
use std::io::Error;

use crate::app::state::AppState;
use crate::messages::auth::*;

#[post("/signup")]
async fn sign_up(
    state: Data<AppState>,
    body: Json<SignUpPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[post("/signin/username")]
async fn username_sign_in(
    state: Data<AppState>,
    body: Json<UsernameSignInPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[post("/signin/email")]
async fn email_sign_in(
    state: Data<AppState>,
    body: Json<EmailSignInPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[patch("/username")]
async fn update_username(
    state: Data<AppState>,
    body: Json<EmailSignInPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[patch("/email")]
async fn update_email(
    state: Data<AppState>,
    body: Json<EmailSignInPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[patch("/phone-number")]
async fn update_phone_number(
    state: Data<AppState>,
    body: Json<UpdatePhoneNumberPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

#[patch("/password")]
async fn update_password(
    state: Data<AppState>,
    body: Json<UpdatePasswordPayload>,
) -> Result<impl Responder, Error> {
    Ok(Json("Hello, world!"))
}

pub fn setup() -> Scope {
    scope("/v1/auth")
        .service(sign_up)
        .service(username_sign_in)
        .service(email_sign_in)
        .service(update_username)
        .service(update_email)
        .service(update_phone_number)
        .service(update_password)
}
