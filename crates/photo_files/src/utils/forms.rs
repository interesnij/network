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

        let format_folder = format!(
            "/network/crates/photo_files/media/{}/",
            list_id.to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub files:        Vec<String>,
}
#[derive(Serialize)]
// отдаем загруженные фото для сохранения объектов фото
pub struct DataNewPhotos { 
    pub token:        String,
    pub list_id:      i32,
    pub server_id:    i16,
    pub user_id:      i32,
    pub community_id: i32, 
    pub files:        Vec<String>,
}

pub async fn files_form(payload: &mut Multipart, list_id: i32) -> FileForm {
    use std::path::Path;
    use image_convert::{ImageResource, JPGConfig, identify, to_jpg};
    use uuid::Uuid;

    let mut form: FileForm = FileForm {
        user_id: 0,
        community_id: None,
        files: Vec::new(),
    };

    while let Some(item) = payload.next().await { 
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "user_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.user_id = _int;
                }
            }
        }
        else if name == "community_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.community_id = Some(_int);
                }
            }
        }
        else if name == "files[]" {
            let _uuid = Uuid::new_v4();
            let _new_path = _uuid.to_string() + &".jpg".to_string();
            if _new_path != "" { 
                let file = UploadedFiles::new(_new_path.to_string(), list_id);
                let file_path = file.path.clone();
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

                let folder_path = "/network/crates/photo_files/media/".to_owned() + &list_id.to_string() + &"/".to_string();
                let folder = Path::new(&folder_path);
                let cur_path = "/media/".to_owned() + &list_id.to_string() + &"/".to_string();

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
                config.quality = 30;
                let input = ImageResource::from_path(source_image_path.clone());
                let mut output = ImageResource::from_path(thumb_image_path);
                to_jpg(&mut output, &input, &config).unwrap();

                let source_image_path = Path::new(&file.path);
                let cur_p = "stand-".to_string() + &_new_path;
                let cur_image_path = Path::join(folder, &cur_p);
    
                let mut config = JPGConfig::new();
                if width > height {
                    if width > 1920 {
                        config.width = 1920;
                    }
                    if height > 1080 {
                        config.height = 1080;
                    } 
                }
                else if height > width {
                    if width > 960 {
                        config.width = 960;
                    }
                    if height > 1280 {
                        config.height = 1280;
                    } 
                }
                else if height == width {
                    if width > 1000 {
                        config.width = 1000;
                        config.height = 1000;
                    }
                }
                else {
                    config.width = width as u16;
                    config.height = height as u16;
                }

                //config.quality = 99;
                let input = ImageResource::from_path(source_image_path);
                let mut output = ImageResource::from_path(cur_image_path);
                to_jpg(&mut output, &input, &config).unwrap();

                form.files.push(_new_path);
            }
        }
    }
    form
}