use std::sync::Arc;
use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    cookie::Key,
    dev::ServiceRequest,
    Error,
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_redis::RedisSession;
use actix_web_httpauth::{
    extractors::{bearer::{BearerAuth, Config}, AuthenticationError},
    middleware::HttpAuthentication,
};
use std::sync::Mutex;
use std::env;
mod views;
mod utils;
mod routes;


pub struct AppState {
    key:   Arc<String>,
    token: Mutex<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    let app_state = AppState {
        key: Arc::new(env::var("KEY").unwrap()),
        token: Mutex::new("".to_string()),
    };

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        App::new() 
            .app_data(web::Data::new (
                AppState {
                    key: Arc::new(env::var("KEY").unwrap()),
                    token: Mutex::new("".to_string()),
                }
            ))
            .wrap(IdentityMiddleware::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .configure(routes)
            .service(_files)

    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}