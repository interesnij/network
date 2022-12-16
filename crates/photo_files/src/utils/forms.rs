use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};

#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new (
        filename: String,
        list_id:  i32
    ) -> UploadedFiles {
        use chrono::Datelike;

        //let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "/media/ser1/{}/",
            list_id.to_string(),
            //now.year().to_string(),
            //now.month().to_string(),
            //now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        //let create_path = format_folder.replace("./", "/my/"); // вариант для https
        let create_path = format_folder.replace("./", "/");    // вариант для debug
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileVars {
    pub original: String, // Путь к оригиналу загруженного фото
    pub file:     String, // Путь к оптимизированному варианту
    pub preview:  String, // Путь к миниатюре
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm {
    pub files: Vec<FileVars>,
}
pub async fn files_form(payload: &mut Multipart, list_id: i32) -> FileForm {
    use std::path::Path;
    use image_convert::{ImageResource, identify, JPGConfig , to_jpg};
    use uuid::Uuid;

    let mut form: FileForm = FileForm {
        files: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        let _uuid = Uuid::new_v4();
        let _new_path = _uuid.to_string() + &".jpg".to_string();
        if _new_path != "" { 
            let file = UploadedFiles::new(_new_path.to_string(), list_id);
            let file_path = file.path.clone();
            println!("=============");
            println!("path {:?}", file.path.clone());
            println!("=============");
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };

            let folder_path = "/media/ser1/".to_owned() + &list_id.to_string() + &"/".to_string();
            let folder = Path::new(&folder_path);

            let input = ImageResource::from_path(file.path.clone());
            let mut output = None;
            let id = identify(&mut output, &input).unwrap();

            let width = id.resolution.width;
            let height = id.resolution.height;

            let source_image_path = Path::new(&file.path);
            let thumb_p = "thumb-".to_string() + &_new_path;
            let thumb_image_path = Path::join(folder, &thumb_p);
            let mut config = JPGConfig::new();
            config.width = (width / 10) as u16;
            config.height = (height / 10) as u16;
            config.quality = 0;
            let input = ImageResource::from_path(source_image_path.clone());
            let mut output = ImageResource::from_path(thumb_image_path.clone());

            println!("input {:?}", source_image_path.clone());
            println!("output {:?}", thumb_image_path.clone()); 
            to_jpg(&mut output, &input, &config).unwrap();

            let source_image_path = Path::new(&file.path);
            let cur_p = "thumb-".to_string() + &_new_path;
            let cur_image_path = Path::join(folder, &cur_p);
    
            let mut config = JPGConfig::new();
            config.width = width as u16;
            config.height = height as u16;
            config.quality = 99;
            let input = ImageResource::from_path(source_image_path);
            let mut output = ImageResource::from_path(cur_image_path.clone());
            to_jpg(&mut output, &input, &config).unwrap();

            form.files.push (
                FileVars {
                    original: file.path.clone(),
                    file:     &folder_path + &cur_p,
                    preview:  &folder_path + &thumb_p,
                }
            );
        }
    }
    form
}