use actix_web::{
    delete, patch, post,
    web::{scope, Data, Json},
    Error as ActixError, Responder, Scope,
};

use crate::app::state::AppState;
use crate::messages::auth::*;

#[post("/signup")]
async fn sign_up(
    state: Data<AppState>,
    body: Json<SignUpBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[post("/signin/username")]
async fn username_sign_in(
    state: Data<AppState>,
    body: Json<UsernameSignInBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[post("/signin/email")]
async fn email_sign_in(
    state: Data<AppState>,
    body: Json<EmailSignInBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[patch("/username")]
async fn update_username(
    state: Data<AppState>,
    body: Json<EmailSignInBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[patch("/email")]
async fn update_email(
    state: Data<AppState>,
    body: Json<EmailSignInBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[patch("/phone-number")]
async fn update_phone(
    state: Data<AppState>,
    body: Json<UpdatePhoneBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[patch("/password")]
async fn update_password(
    state: Data<AppState>,
    body: Json<UpdatePasswordBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[delete("/deactivate")]
async fn deactivate(state: Data<AppState>) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[delete("/")]
async fn delete(state: Data<AppState>) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

pub fn setup() -> Scope {
    scope("/v1/auth")
        .service(sign_up)
        .service(username_sign_in)
        .service(email_sign_in)
        .service(update_username)
        .service(update_email)
        .service(update_phone)
        .service(update_password)
        .service(deactivate)
        .service(delete)
}
