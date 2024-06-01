// use actix::SyncArbiter;
use actix_web::{middleware, web::Data, App, HttpServer};
use dotenv::dotenv;
use std::{env, io::Error};

mod app;
mod controllers;
mod db;
mod messages;
mod models;
mod services;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

    // Server Env
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    // DB Env
    let db_url = env::var("DATABASE_URL").expect("DB_URL env must be set!");

    let pool = db::utils::get_pool(&db_url);
    // let db_addr = SyncArbiter::start(5, move || db::actor::DbActor(pool.clone()));

    let server = HttpServer::new(move || {
        App::new()
            // .app_data(Data::new(app::state::AppState {
            //     db: db_addr.clone(),
            // }))
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(controllers::auth::setup())
            .service(controllers::user::setup())
    })
    .bind((host, port.parse::<u16>().unwrap()))
    .unwrap()
    .run();

    match futures::try_join!(server, server_run_callback(port)) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("{}", err);
            Err(err)
        }
    }
}

async fn server_run_callback(port: String) -> Result<(), Error> {
    log::info!("API is running on port {}!", port);
    Ok(())
}
