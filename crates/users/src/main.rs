#[macro_use]
extern crate diesel;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use actix_web::{
    web,
    App,
    HttpServer,
};
use actix_cors::Cors;

mod models;
mod views;
mod utils;
mod schema;
mod errors;
mod routes;

#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;

    dotenv().ok();
    let app_state = AppState {
        key: Arc::new(env::var("KEY").unwrap()),
    };
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8100")
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(cors)
            .configure(routes)

    })
    .bind("194.58.90.123:9001")? // работа
    //.bind("194.58.90.123:9021")?   // разработка
    .run()
    .await
}
