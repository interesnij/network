#[macro_use]
extern crate diesel;
use std::sync::Arc;
pub mod schema;
pub mod models;
pub mod routes;
mod errors;
use dotenv::dotenv;
#[macro_use]
mod utils;
#[macro_use]
mod views;

#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web::JsonConfig, web::Data};
    use actix_cors::Cors;
    use crate::routes::routes;

    dotenv().ok();
    let app_state = AppState {
        key: Arc::new(env::var("KEY").unwrap()),
    };

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .app_data(Data::new(app_state.to_owned()))
            .app_data(JsonConfig::default().limit(4096))
            .wrap(cors)
            .configure(routes)
    })
    .bind("194.58.90.123:9003")?
    .run()
    .await
}
