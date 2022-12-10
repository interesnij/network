//#[macro_use]
//extern crate diesel;

//use std::sync::Arc;
//use dotenv::dotenv;
//use std::env;
use actix_web::{
    //middleware::Logger,
    Result,
    HttpRequest,
};
use actix_cors::Cors;
use actix_files::NamedFile;
use std::path::PathBuf;

//mod models;
//mod views;
//mod utils;
//mod schema;
//mod errors;
//mod routes;



async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use crate::routes::routes;
    use actix_web::{web, App, HttpServer};

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8100")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/{filename:.*}", web::get().to(get_file))
            //.configure(routes)

    })
    .bind("194.58.90.123:8050")?
    .run()
    .await
}
