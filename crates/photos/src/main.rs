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
    use crate::utils::{proxy, Config};

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse();
    let Config { address, port, to } = config.clone();
    info!("Listening on {address}:{port}");
    info!("Proxying requests to {to}");

    HttpServer::new(|| {
        let http_client = awc::Client::default();
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(http_client))
            .app_data(JsonConfig::default().limit(4096))
            .wrap(cors)
            .configure(routes)
            .service(web::resource("{path:.*}").to(proxy))
    })
    .bind("194.58.90.123:9004")?
    .run()
    .await
}
