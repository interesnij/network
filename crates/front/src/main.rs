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
use actix_session::{storage::RedisSessionStore, SessionMiddleware};

mod views;
//mod utils;
mod routes;

#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use actix_files::Files;

    let app_state = AppState {
        key: Arc::new("KEY".to_string()),
    };
    let _files = Files::new("/static", "static/").show_files_listing();
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
        //    .allowed_origin("194.58.90.123:8100")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone()
            ))
            .wrap(cors)
            .configure(routes)
            .service(_files)

    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}