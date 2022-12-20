use std::sync::Arc;
use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    cookie::Key,
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_redis::RedisSession;

mod views;
//mod utils;
mod routes;

#[derive(Clone)]
pub struct AppState {
    key: Arc<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    let app_state = AppState {
        key: Arc::new(0),
    };
    let _files = Files::new("/static", "static/").show_files_listing();

    HttpServer::new(move || {
        App::new() 
            //.app_data(web::Data::new(app_state.to_owned()))
            .wrap(IdentityMiddleware::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .configure(routes)
            .service(_files)

    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}