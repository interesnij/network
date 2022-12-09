#[macro_use]
extern crate diesel;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use actix_web::{
    middleware::Logger,
    web,
    guard,
    App,
    HttpRequest,
    HttpServer,
    Result
};
use actix_cors::Cors;

mod models;
mod views;
mod utils;
mod schema;
mod errors;
mod routes;

#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::routes;
    use chrono::Duration;
    use actix_extensible_rate_limit::{
        backend::{
            SimpleInputFunctionBuilder,
            RedisBackend
        },
        RateLimiter,
    };

    dotenv().ok();
    let backend = RedisBackend::builder().build();
    let app_state = AppState {
        key: Arc::new(env::var("KEY").unwrap()),
    };

    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 5)
            .real_ip_key()
            .build();
        let limit_middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8100")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(guard::Header("content-type", "application/json"))
            .wrap(cors)
            .wrap(limit_middleware)
            .configure(routes)

    })
    .bind("194.58.90.123:8000")?
    .run()
    .await
}
