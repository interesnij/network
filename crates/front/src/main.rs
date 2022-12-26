use std::sync::Arc;
use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    dev::ServiceRequest,
    Error,
};
use actix_cors::Cors;
use std::{sync::Mutex, env}; 

mod views;
mod utils;
mod routes;
mod errors;

pub struct AppState {
    device: Mutex<u8>,             // 1 - комп, 2 - телефон
    token:  Mutex<String>,
}
pub struct UserState {
    id:           Mutex<i32>,
    name:         Mutex<String>,
    link:         Mutex<String>,
    image:        Mutex<String>,
    new_follows:  Mutex<u16>,
    new_messages: Mutex<u16>,
    new_notifies: Mutex<u16>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        App::new()  
            .app_data(web::Data::new (
                AppState {
                    device: Mutex::new(0),
                    token:  Mutex::new("".to_string()),
                }
            ))
            .app_data(web::Data::new (
                UserState {
                    id:           Mutex::new(0),
                    name:         Mutex::new("".to_string()),
                    link:         Mutex::new("".to_string()),
                    image:        Mutex::new("".to_string()),
                    new_follows:  Mutex::new(0),
                    new_messages: Mutex::new(0),
                    new_notifies: Mutex::new(0),
                }
            ))
            .configure(routes)
            .service(_files)
    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}