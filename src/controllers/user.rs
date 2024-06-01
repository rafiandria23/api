use actix_web::{
    get, put,
    web::{scope, Data, Json},
    Error as ActixError, Responder, Scope,
};

use crate::app::state::AppState;
use crate::messages::user::*;

#[get("/me")]
async fn me(state: Data<AppState>) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

#[put("/")]
async fn update(
    state: Data<AppState>,
    body: Json<UpdateBodyMessage>,
) -> Result<impl Responder, ActixError> {
    Ok("Hello world!")
}

pub fn setup() -> Scope {
    scope("/v1/users").service(me).service(update)
}
