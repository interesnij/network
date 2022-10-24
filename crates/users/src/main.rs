#[macro_use]
extern crate diesel;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use actix_web::{
    middleware::Logger,
    web,
    App,
    HttpRequest,
    HttpServer,
    Result
};
use actix_cors::Cors;
use log::Level;


mod models;
mod handlers;
mod repositories;
mod config;
mod utils;

use handlers::{
    auth_handlers::auth_scope,
    user_handlers::user_scope,
};
use utils::establish_connection;


#[derive(Clone)]
pub struct AppState {
    pg:  Arc<String>,
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    log::info!("Load config:");
    dotenv().ok();

    let _connection = establish_connection();

    let app_state = AppState{
        pg: Arc::new(_connection),
        key: Arc::new(env::var("KEY").unwrap()),
    };

    log::info!("Start server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(cors)
            .service(user_scope())
            .service(auth_scope())

    })
    .bind("194.58.90.123:9001")?
    .run()
    .await
}
