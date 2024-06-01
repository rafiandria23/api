use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignUpBodyMessage {
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsernameSignInBodyMessage {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailSignInBodyMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUsernameBodyMessage {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEmailBodyMessage {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePhoneBodyMessage {
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePasswordBodyMessage {
    pub old_password: String,
    pub new_password: String,
}
