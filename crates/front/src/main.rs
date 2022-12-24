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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;
    use crate::utils::UserState;

    let data = web::Data::new(Mutex::new(UserState {
        device:       0,
        user_name:    "".to_string(),
        user_link:    "".to_string(),
        user_id:      0,
        user_image:   "".to_string(),
        new_follows:  0,
        new_messages: 0,
        new_notifies: 0,
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