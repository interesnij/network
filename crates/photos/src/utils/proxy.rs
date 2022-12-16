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
    let mut server_id: i16 = 0;
    let v: Vec<&str> = _path.split("/").collect();
    for _v in v.iter() {
        if _v.contains("ser") {
            server_id = _v[3..].parse().unwrap();
            break;
        }
    };
    let filename = v.last().unwrap();
    let f: Vec<&str> = _path.split("-").collect();
    photo_id = (f.split_first().unwrap().parse()).unwrap();

    let item: Photo;
    let list: PhotoList;
    let item_res = get_photo(photo_id);
    if item_res.is_ok() {
        item = item_res.expect("E");
        list = item.get_list().expect("E");
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