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
use actix_session::storage::RedisSessionStore;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{Session, SessionMiddleware};
use std::{sync::Mutex, env};

mod views;
mod utils;
mod routes;
mod errors;

pub struct AppState {
    device:       Mutex<u8>,             // 1 - комп, 2 - телефон
    user_name:    Mutex<String>,
    user_link:    Mutex<String>,
    user_id:      Mutex<i32>,
    user_image:   Mutex<String>,
    new_follows:  Mutex<u16>,
    new_messages: Mutex<u16>,
    new_notifies: Mutex<u16>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        App::new() 
            .app_data(web::Data::new (
                AppState {
                    device:       Mutex::new(0),
                    user_name:    Mutex::new("".to_string()),
                    user_link:    Mutex::new("".to_string()),
                    user_id:      Mutex::new(0),
                    user_image:   Mutex::new("".to_string()),
                    new_follows:  Mutex::new(0),
                    new_messages: Mutex::new(0),
                    new_notifies: Mutex::new(0),
                }
            ))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis_store.clone(), secret_key.clone()))
            .configure(routes)
            .service(_files)
    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}