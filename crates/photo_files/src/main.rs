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

pub async fn index_page(req: HttpRequest) -> Result<NamedFile> {
    use std::path::Path;
    use image_convert::{ImageResource, identify, JPGConfig , to_jpg};

    let input = ImageResource::from_path("static/service_cat.jpg");
    let mut output = None;
    let id = identify(&mut output, &input).unwrap();

    let width = id.resolution.width;
    let height = id.resolution.height;
    let format = id.format;
    
    let source_image_path = Path::new("static/service_cat.jpg");
    let target_image_path = Path::join(source_image_path.parent().unwrap(), "bus_small.jpg");
    
    let mut config = JPGConfig::new();
    config.width = (width / 10) as u16;
    config.height = (height / 10) as u16;
    config.quality = 0;

    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(target_image_path.clone());
    to_jpg(&mut output, &input, &config).unwrap();

    Ok(NamedFile::open(target_image_path)?)
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
            .route("/{filename:.*}", web::get().to(get_file))
            .route("/create_files/{list_id}", web::post().to(create_files))
    })
    .bind("194.58.90.123:9050")?
    .run()
    .await
}
