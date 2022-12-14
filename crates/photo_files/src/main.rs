use actix_web::{
    Result,
    HttpRequest,
    web,
    web::Json,
};
use actix_cors::Cors;
use actix_files::NamedFile;
use std::path::PathBuf;
use actix_multipart::Multipart;

mod utils;


async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn create_files(mut payload: Multipart, list_id: web::Path<i32>) -> 
    Result<Json<Vec<String>>> {
        use crate::utils::files_form;

        let form = files_form(payload.borrow_mut(), list_id).await;
        Ok(Json(form.files))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("194.58.90.123:8000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/{filename:.*}", web::post().to(get_file))
            .route("/create_files/{list_id}", web::get().to(get_file))

    })
    .bind("194.58.90.123:9050")?
    .run()
    .await
}
