use actix_web::web::Json;

use crate::messages::auth::*;

pub async fn sign_up(body: Json<SignUpBodyMessage>) {}

pub async fn username_sign_in(body: Json<UsernameSignInBodyMessage>) {}

pub async fn email_sign_in(body: Json<EmailSignInBodyMessage>) {}

pub async fn update_username(user_id: String, body: Json<UpdateUsernameBodyMessage>) {}

pub async fn update_email(user_id: String, body: Json<UpdateEmailBodyMessage>) {}

pub async fn update_phone(user_id: String, body: Json<UpdatePhoneBodyMessage>) {}

pub async fn update_password(user_id: String, body: Json<UpdatePasswordBodyMessage>) {}
