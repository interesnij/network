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

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/ser1/{}/{}/{}/{}/",
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
pub struct FileForm {
    pub files: Vec<String>,
}
pub async fn files_form(payload: &mut Multipart, list_id: i32) -> FileForm {
    use uuid::Uuid;

    let mut _files: Vec<UploadedFiles> = Vec::new();

    let mut form: FileForm = FileForm {
        files: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "files[]" {
            let _new_path = format!("{}.jpg", Uuid::new_v4());
            if _new_path != "" { 
                let file = UploadedFiles::new(_new_path.to_string(), list_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                _files.push(file.clone());
                form.files.push(file.path.clone()).replace("./","/"));
            }
        }
    }
    form
}