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
    Photo,
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
    pub token:     Option<String>, // токен
    pub user_id:   Option<i32>,    // кто запрашивает
    pub server_id: Option<i16>,    // какой сервер нужен
    pub photo_id:  Option<i32>,    // какое фото  интересует
} 

pub async fn get_file (
    req:         HttpRequest,
    body:        web::Payload,
    path:        web::Path<String>
    http_client: Data<awc::Client>,
) -> impl Responder {
    let _path = path.clone();

    if params_some.is_ok() {
        let mut is_open = false;
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id, 1);
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.server_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'server_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.photo_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'photo_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: Photo;
            let item_res = get_photo(params.photo_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if user_id > 0 {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        return HttpResponse::Ok().body(body);
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            return HttpResponse::Ok().body(body);
                        }
                        else {
                            is_open = true;
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        return HttpResponse::Ok().body(body);
                    }
                    else {
                        is_open = true;
                    }
                }
            }
            else {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        return HttpResponse::Ok().body(body);
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            return HttpResponse::Ok().body(body);
                        }
                        else {
                            is_open = true;
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        return HttpResponse::Ok().body(body);
                    }
                    else {
                        is_open = true;
                    }
                }
            }
        }
        if is_open {
            let _server_id = params.server_id.unwrap();
            let to: String; 
            if _server_id == 0 {
                to = "http://194.58.90.123:9050".to_string();
            }
            else if _server_id == 1 {
                to = "http://194.58.90.123:9050".to_string();
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "server not found!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
            let url = format!(
                "{to}{path}",
                to = to,
                path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
            );
            debug!("=> {url}");
            match http_client
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
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}