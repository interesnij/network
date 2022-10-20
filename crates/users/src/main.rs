#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use actix_web::{
    middleware::Logger,
    web,
    App,
    HttpRequest,
    HttpServer,
    Result
};
use actix_cors::Cors;
use log::Level;


mod models;
mod handlers;
mod repositories;
mod config;

use handlers::{
    auth_handlers::auth_scope,
    user_handlers::user_scope,
};
use rbatis::{rbatis::Rbatis, plugin::snowflake::Snowflake};


#[derive(Clone)]
pub struct AppState{
    rb: Arc<Rbatis>,
    key: Arc<String>,
    sflake: Arc<Snowflake>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    log::info!("Load config:");
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    let rb = Rbatis::new();
    log::info!("Link database");
    rb.link(env::var("DATABASE_URL").unwrap().as_str()).await.expect("faile to link database");

    let app_state = AppState{
        rb: Arc::new(rb),
        key: Arc::new(env::var("KEY").unwrap()),
        sflake: Arc::new(Snowflake::new(161476480000, 1, 1))
    };

    log::info!("Start server");
    HttpServer::new(move || {
        //let cors = Cors::permissive();
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(cors)
            .service(user_scope())
            .service(auth_scope())

    })
    .bind("194.58.90.123:9001")?
    .run()
    .await
}
