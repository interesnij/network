#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web::JsonConfig};
    use actix_cors::Cors;
    use crate::routes::routes;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .app_data(JsonConfig::default().limit(4096))
            .wrap(cors)
            .configure(routes)
    })
    .bind("194.58.90.123:9004")?
    .run()
    .await
}
