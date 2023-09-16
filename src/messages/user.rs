use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReadByIdPayload {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct ReadByUsernamePayload {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ReadByEmailPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub first_name: String,
    pub last_name: Option<String>,
}
