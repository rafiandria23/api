use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateBodyMessage {
    pub first_name: String,
    pub last_name: Option<String>,
}
