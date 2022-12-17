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
    use actix_web::{App, HttpServer, web::JsonConfig, web, web::Data};
    use actix_cors::Cors;
    use crate::routes::routes;
    use clap::Parser;
    use crate::utils::{
        get_file,
        upload_files,
        ConfigToStaticServer,
    };

    let config_to_static_server = ConfigToStaticServer::parse();

    HttpServer::new(move || {
        let http_client = awc::Client::default();
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_origin("194.58.90.123:9050")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .app_data(Data::new(config_to_static_server.clone()))
            //.app_data(Data::new(config_to_user_server.clone()))
            .app_data(Data::new(http_client))
            .app_data(JsonConfig::default().limit(4096))
            .wrap(cors)
            .configure(routes)

            // прокси на серверы медиа для открытия фото.
            .service(web::resource("/static{path:.*}").to(get_file))

            // прокси на серверы медиа для сохранения фото.
            .service(web::resource("/create_files{path:.*}").to(upload_files))
            
    })
    .bind("194.58.90.123:9004")?
    .run()
    .await
}
