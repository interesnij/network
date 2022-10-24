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
//use log::Level;


mod models;
mod handlers;
mod repositories;
mod config;
mod utils;
mod schema;

//use handlers::{
//    auth_handlers::auth_scope,
//    user_handlers::user_scope,
//};


#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_state = AppState{
        key: Arc::new(env::var("KEY").unwrap()),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(cors)
            //.service(user_scope())
            //.service(auth_scope())

    })
    .bind("194.58.90.123:9001")?
    .run()
    .await
}
