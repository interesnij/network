use actix_web::{
    Result,
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use actix_cors::Cors;
use actix_files::NamedFile;
use std::path::PathBuf;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
mod utils;
use crate::utils::DataNewPhotos;


async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body(
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I media server #1.
            </p>
        </div>")
}


pub async fn create_files (
    mut payload: Multipart,
    list_id: web::Path<i32>
) -> Result<Json<i16>> {
        use crate::utils::files_form;

        let form = files_form(payload.borrow_mut(), *list_id).await;
        let client = reqwest::Client::new();
        let res = client.post("194.58.90.123:9004/add_photos_in_list")
            .form( &DataNewPhotos {
                token:        form.token.clone(),
                list_id:      *list_id,
                server_id:    1,
                user_id:      form.user_id,
                community_id: form.community_id,
                files:        form.files,
            })
            .send()
            .await;
        return Ok(Json(1));
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
            .route("/", web::get().to(index_page))
            .route("/{filename:.*}", web::get().to(get_file))
            .route("{list_id}", web::post().to(create_files))
    }) 
    .bind("194.58.90.123:9050")?
    .run()
    .await
}
