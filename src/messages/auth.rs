use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpPayload {
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UsernameSignInPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct EmailSignInPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUsernamePayload {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UpdateEmailPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdatePhoneNumberPayload {
    pub phone_number: String,
}

#[derive(Deserialize)]
pub struct UpdatePasswordPayload {
    pub password: String,
}
