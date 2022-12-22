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


mod views;
//mod utils;
mod routes;


async fn validator (
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    match auth::get_token_data(credentials.token()).await {
        Ok(_) => Ok(req),
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}


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

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        let auth = HttpAuthentication::bearer(validator);
        App::new() 
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(IdentityMiddleware::default())
            .wrap(auth)
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .configure(routes)
            .service(_files)

    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}