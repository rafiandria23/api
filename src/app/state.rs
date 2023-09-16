use crate::db::actor::DbActor;
use actix::Addr;

pub struct AppState {
    pub db: Addr<DbActor>,
}
