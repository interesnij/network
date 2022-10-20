#[macro_use]
extern crate diesel;
//#[macro_use(concat_string)]
//extern crate concat_string;

pub mod schema;
pub mod models;
pub mod routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use actix_cors::Cors;
    use crate::routes::routes;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_origin("194.58.90.123:8001")
            .allowed_origin("194.58.90.123:8002")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .configure(routes)
    })
    .bind("194.58.90.123:9003")?
    .run()
    .await
}
