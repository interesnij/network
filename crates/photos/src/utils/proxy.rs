use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Data};
use awc::http::StatusCode;
use clap::Parser;
use env_logger::Env;
use futures::TryStreamExt;
use log::{debug, info, warn};
use serde::Deserialize;
use crate::utils::{
    get_community,
    get_user,
    get_photo,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_owner_data, 
    ErrorParams,
    SearchTargetListData, SearchRegListData,
    CardPhotoListJson, CardPhotoJson,
};
use crate::models::{
    Photo, PhotoList,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::str;
use std::borrow::BorrowMut;


#[derive(Clone, Parser)]
pub struct ConfigToStaticServer {
    #[clap(short, long, default_value = "194.58.90.123")]             // наш ip
    pub address: String,
    #[clap(short, long, default_value = "9004")]                      // наш порт
    pub port: u16,
    #[clap(short, long, default_value = "http://194.58.90.123:9050")] // адрес, на который будем перенаправлять запросы
    pub to: String,
}


#[derive(Debug, Deserialize)]
pub struct LoadPhotoParams {
    pub token:        Option<String>, // токен
    pub user_id:      Option<i32>,    // пользователь
    pub community_id: Option<i32>,    // сообщество
} 

pub async fn get_file (
    req:         HttpRequest,
    body:        web::Payload,
    path:        web::Path<String>,
    http_client: Data<awc::Client>,
) -> impl Responder {
    /* 
    id сервера картинок мы зашиваем в названии папки пути файла - "ser1":
    это сервер 1

    id фото мы зашиваем в названии файла - "1-000-000-000-000.jpg":
    это фото с id 1
    */

    let _path = path.clone();
    let mut is_open = false;
    let params_some = web::Query::<LoadPhotoParams>::from_query(&req.query_string());

    let mut photo_id: i32 = 0;
    let server_id: i16;
    let v: Vec<&str> = _path.split("/").collect();
    let filename = v.last().unwrap();
    let f: Vec<&str> = filename.split("-").collect();
    for (i, _f) in f.iter().enumerate() {
        if i == 0 {
            let photo_id_some = _f.parse();
            if photo_id_some.is_ok() {
                photo_id = photo_id_some.unwrap();
            }
            break;
        }
    };

    let item: Photo;
    let list: PhotoList;
    let item_res = get_photo(photo_id);
    if item_res.is_ok() {
        item = item_res.expect("E");
        list = item.get_list().expect("E");
        server_id = item.server_id;
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "photo not found!".to_string(),
        }).unwrap();
        return HttpResponse::Ok().body(body);
    }

    if item.community_id.is_some() {
        let community = item.get_community().expect("E.");
        let _tuple = get_anon_community_permission(&community);
        if _tuple.0 == false {
            is_open = false;
        }
        else if community.is_anon_user_see_el() && list.is_anon_user_see_el() {
            is_open = true;
        }
        else if params_some.is_err() {
            is_open = false;
        }
        else {
            let params = params_some.unwrap();
            if params.token.is_none() {
                is_open = false;
            }
        
            let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id, 8);
                
            if err.is_some() {
                is_open = false;
            }
            else if community_id > 0 && community.id != community_id {
                is_open = false;
            }

            else if user_id > 0 {
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    is_open = false;
                }
                else if community.is_user_see_el(user_id) && list.is_user_see_el(user_id) {
                    is_open = true;
                }
                else {
                    is_open = false;
                }
            }
        }
    }
    else {
        let owner = item.get_creator().expect("E.");
        let _tuple = get_anon_user_permission(&owner);
        if _tuple.0 == false {
            is_open = false;
        }
        else if owner.is_anon_user_see_el() && list.is_anon_user_see_el() {
            is_open = true;
        }
        else if params_some.is_err() {
            is_open = false;
        }
        else {
            let params = params_some.unwrap();
            if params.token.is_none() {
                is_open = false;
            }
        
            let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id, 8);
                
            if err.is_some() {
                is_open = false;
            }
            if community_id > 0 {
                is_open = false;
            }

            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    is_open = false;
                }
                else if owner.is_user_see_el(user_id) && list.is_user_see_el(user_id) {
                    is_open = true;
                }
                else {
                    is_open = false;
                }
            }
        }
    }

    if is_open {
        let to: String; 
        if server_id == 1 {
            to = "http://194.58.90.123:9050".to_string();
        }
        else if server_id == 2 {
            to = "http://194.58.90.123:9051".to_string();
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "server not found!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        let url = format!(
            "{to}{path}",
            to = to,
            path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
        );
        debug!("=> {url}");
        return match http_client
            .request_from(&url, req.head())
            .send_stream(body)
            .await
        {
            Ok(resp) => {
                let status = resp.status();
                debug!("<= [{status}] {url}", status = status.as_u16());
                let mut resp_builder = HttpResponse::build(status);
                for header in resp.headers() {
                    resp_builder.insert_header(header);
                }
                resp_builder.streaming(resp.into_stream())
            }
            Err(err) => {
                warn!("{url}: {err:?}");
                HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
}

pub async fn files_form(payload: &mut Multipart) -> FileForm {
    use std::path::Path;
    use image_convert::{ImageResource, JPGConfig, identify, to_jpg};
    use uuid::Uuid;

    let mut form: FileForm = FileForm {
        token: None,
        user_id: None,
        community_id: None,
    };

    while let Some(item) = payload.next().await { 
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "token" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.token = Some(data_string);
                }
            }
        }
        else if name == "user_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.user_id = Some(_int);
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
    }
    form
}

pub async fn upload_files (
    mut payload: Multipart,
    path: web::Path<String>,
) -> impl Responder {
    let form = files_form(payload.borrow_mut()).await;
    let mut is_open = false;

    let (err, user_id, community_id) = get_owner_data(Some(form.token.clone()), Some(form.user_id), 21);
    if err.is_some() { 
        is_open = false;
    }
    else if user_id < 1 && community_id < 1 {
        is_open = false;
    }
    else {
        let list = get_photo_list(path).expect("E.");
        let c_id: Option<i32>;
        if community_id > 0 {
            c_id = Some(community_id);
        }
        else {
            c_id = list.community_id;
        }

        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            if community_id > 0 && list.community_id.unwrap() == community_id
                ||
                user_id > 0 && (list.is_user_create_el(user_id) || community.is_user_create_el(user_id))
            {
                is_open = true;
            }
            else {
                is_open = false;
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            if community_id < 1 || user_id > 0 && (list.is_user_create_el(user_id) || owner.is_user_create_el(user_id)) {
                is_open = true;
            }
            else {
                is_open = false;
            }
        }
    }

    if is_open {
        let to: String; 
        if server_id == 1 {
            to = "http://194.58.90.123:9050".to_string();
        }
        else if server_id == 2 {
            to = "http://194.58.90.123:9051".to_string();
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "server not found!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        let url = format!(
            "{to}{path}",
            to = to,
            path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
        );
        debug!("=> {url}");
        return match http_client
            .request_from(&url, req.head())
            .send_stream(body)
            .await
        {
            Ok(resp) => {
                let status = resp.status();
                debug!("<= [{status}] {url}", status = status.as_u16());
                let mut resp_builder = HttpResponse::build(status);
                for header in resp.headers() {
                    resp_builder.insert_header(header);
                }
                resp_builder.streaming(resp.into_stream())
            }
            Err(err) => {
                warn!("{url}: {err:?}");
                HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}