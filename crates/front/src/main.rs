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
use std::{sync::Mutex, env};

mod views;
mod utils;
mod routes;
mod errors;

pub struct UserState {
    device:       Mutex<u8>,       // 1 - комп, 2 - телефон
    user_name:    Mutex<String>,
    user_link:    Mutex<String>,
    user_id:      Mutex<i32>,
    user_image:   Mutex<String>,
    new_follows:  Mutex<i32>,
    new_messages: Mutex<i32>,
    new_notifies: Mutex<i32>,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    let data = web::Data::new(Mutex::new(AppState{
        UserState {
            device:       0,
            user_name:    "".to_string(),
            user_link:    "".to_string(),
            user_id:      0,
            user_image:   "".to_string(),
            new_follows:  0,
            new_messages: 0,
            new_notifies: 0,
        }
    }));
    HttpServer::new(move || { 
        let _files = Files::new("/static", "static/").show_files_listing();
        App::new() 
            .register_data(data.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .configure(routes)
            .service(_files)
    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}