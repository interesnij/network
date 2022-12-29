use std::sync::Arc;
use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    dev::ServiceRequest,
    Error,
    web::Data,
};
use actix_cors::Cors;
use std::{sync::Mutex, env}; 

mod views;
mod utils;
mod routes;
mod errors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;
    //use crate::utils::{
    //    user_proxy,
    //    ConfigToUserServer,
    //};
    //use clap::Parser;

    //let config_to_user_server = ConfigToUserServer::parse();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        //let http_client = awc::Client::default();

        App::new()  
            //.app_data(Data::new(http_client))
            //.app_data(Data::new(config_to_user_server.clone()))
            .configure(routes)
            .service(_files)

            // прокси на сервер пользователей.
            //.service(web::resource("/users{path:.*}").to(user_proxy))
    }) 
    .bind("194.58.90.123:8100")? // работа
    //.bind("194.58.90.123:8101")? // разработка
    .run()
    .await
}