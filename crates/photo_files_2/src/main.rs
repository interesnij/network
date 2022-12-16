use actix_web::{
    Result,
    HttpRequest,
    web,
};
use actix_files::NamedFile;
use std::path::PathBuf;


async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(move || {
        App::new()
            .route("/{filename:.*}", web::get().to(get_file))
    })
    .bind("194.58.90.123:9051")?
    .run()
    .await
}
