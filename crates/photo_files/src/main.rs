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


async fn get_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn index_page() -> impl Responder {
    use image_convert::{ImageResource, InterlaceType, identify};

    let input = ImageResource::from_path("static/service_cat.jpg");
    let mut output = None;
    let id = identify(&mut output, &input).unwrap();

    let width = id.resolution.width;
    let height = id.resolution.height;
    let format = id.format;
    //let interlace = id.interlace.to_string();
    let text = format!("<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
        <p style='text-align: center'>
            {}<br />{}<br />{}
        </p>
    </div>", width, height, format);
    HttpResponse::Ok().body (
        text
    )
}

pub async fn create_files(mut payload: Multipart, list_id: web::Path<i32>) -> 
    Result<Json<Vec<String>>> {
        use crate::utils::files_form;

        let form = files_form(payload.borrow_mut(), *list_id).await;
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
            .route("/", web::get().to(index_page))
            .route("/create_files/{list_id}", web::get().to(create_files))
            .route("/{filename:.*}", web::post().to(get_file))

    })
    .bind("194.58.90.123:9050")?
    .run()
    .await
}
