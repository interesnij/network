use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user, get_moderation, get_user_owner_data,
    get_user_permission, TOKEN, USERS_SERVICES,
    ErrorParams, KeyValue, CardUserJson,
};
use crate::models::ModeratedLog;
use crate::errors::Error;
use serde::{Deserialize, Serialize}; 


pub fn manager_urls(config: &mut web::ServiceConfig) {
    config.route("/get_claim", web::get().to(get_claim_page));
    config.route("/create_claim_user", web::post().to(create_claim_user));
    config.route("/close_user", web::post().to(close_user));
    config.route("/unclose_user", web::post().to(unclose_user));
    config.route("/suspend_user", web::post().to(suspend_user));
    config.route("/unsuspend_user", web::post().to(unsuspend_user));
    config.route("/suspend_moderation", web::post().to(suspend_moderation));
    config.route("/close_moderation", web::post().to(close_moderation));
    config.route("/unclose_moderation", web::post().to(unclose_moderation));
    config.route("/unsuspend_moderation", web::post().to(unsuspend_moderation));
    config.route("/unverify_moderation", web::post().to(unverify_moderation));
    config.route("/reject_moderation", web::post().to(reject_moderation));
}

#[derive(Deserialize)]
pub struct ReportParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub target_id:   Option<i32>,
    pub types:       Option<i16>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CloseParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub target_id:   Option<i32>,
    pub description: Option<String>,
}

// для отправки на другие сервисы с пользователями
#[derive(Serialize)]
pub struct DataCloseParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct ModerationParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub target_id:   Option<i32>,
    pub description: Option<String>,
    pub expiration:  Option<chrono::NaiveDateTime>,
}
// для отправки на другие сервисы с пользователями
#[derive(Serialize)]
pub struct DataModerationParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
    pub expiration:  Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct ReportResp {
    pub options: Vec<KeyValue>,
    pub user:    CardUserJson,
}

pub async fn get_claim_page(req: HttpRequest) -> Result<Json<ReportResp>, Error> {
    let params_some = web::Query::<ReportParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 1);
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner_res = get_user(user_id);
            let target_res = get_user(params.target_id.unwrap());
            if owner_res.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner user not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            else if target_res.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "target user not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            let target = target_res.expect("E.");
            let _tuple = get_user_permission(&target, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let mut list = Vec::new();
                let card = CardUserJson {
                    id:         target.id,
                    first_name: target.first_name,
                    last_name:  target.last_name,
                    link:       target.link,
                    image:      target.s_avatar,
                };
                list.push(KeyValue {
                    value: 1,
                    info:  "Эротика / Порнография".to_string(),
                });
                list.push(KeyValue {
                    value: 2,
                    info:  "Оскорбительное содержание".to_string(),
                });
                list.push(KeyValue {
                    value: 3,
                    info:  "Мошенничество".to_string(),
                });
                list.push(KeyValue {
                    value: 4,
                    info:  "Наркотики".to_string(),
                });
                list.push(KeyValue {
                    value: 5,
                    info:  "Продажа оружия".to_string(),
                });
                list.push(KeyValue {
                    value: 6,
                    info:  "Насилие".to_string(),
                });
                list.push(KeyValue {
                    value: 7,
                    info:  "Призыв к травле".to_string(),
                });

                list.push(KeyValue {
                    value: 8,
                    info:  "Призыв к суициду".to_string(),
                });
                list.push(KeyValue {
                    value: 9,
                    info:  "Жестокое обращение c животными".to_string(),
                });
                list.push(KeyValue {
                    value: 10,
                    info:  "Введение в заблуждение".to_string(),
                });
                list.push(KeyValue {
                    value: 11,
                    info:  "Экстремизм".to_string(),
                });
                list.push(KeyValue {
                    value: 12,
                    info:  "Риторика ненависти".to_string(),
                });

                Ok(Json(ReportResp {
                    options: list,
                    user:    card,
                }))
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn create_claim_user(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.types.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'types' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let owner = get_user(data.target_id.unwrap()).expect("E.");
        let _tuple = get_user_permission(&owner, user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                user_id,
                data.types.unwrap(),
                owner.id, 
                data.description.clone(),
            )).await?;
            Ok(Json(_res))
        }
    }
}

pub async fn close_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 { 
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id, 
                        item.id,
                        2,
                        data.description.clone(),
                        None,
                    );
                    item.close_item()
                }
            ).await?;
            
            let copy_user = DataCloseParams {
                token:       Some(TOKEN.to_string()),
                user_id:     data.user_id,
                item_id:     data.target_id,
                description: data.description.clone(),
            };
        
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let res = client.post(link.to_string() + &"/close_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unclose_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        4,
                        data.description.clone(),
                        None,
                    );
                    item.close_item()
                }
            ).await?;

            let copy_user = DataCloseParams {
                token:       Some(TOKEN.to_string()),
                user_id:     data.user_id,
                item_id:     data.target_id,
                description: data.description.clone(),
            };
        
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let res = client.post(link.to_string() + &"/unclose_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn suspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is requiredd!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        let target_id = data.target_id;
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        1,
                        data.description.clone(),
                        data.expiration,
                    );
                    item.suspend_item()
                }
            ).await?;

            let copy_user = DataModerationParams {
                token:       Some(TOKEN.to_string()),
                user_id:     Some(user_id),
                item_id:     target_id,
                description: data.description.clone(),
                expiration:  data.expiration,
            };
        
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let res = client.post(link.to_string() + &"/suspend_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn unsuspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        let target_id = data.target_id;
        let description = data.description;
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        description.clone(),
                        data.expiration,
                    );
                    item.unsuspend_item()
                }
            ).await?; 

            let copy_user = DataModerationParams {
                token:       Some(TOKEN.to_string()),
                user_id:     Some(user_id),
                item_id:     target_id,
                description: description.clone(),
                expiration:  data.expiration,
            };
        
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let res = client.post(link.to_string() + &"/unsuspend_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn suspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        let description = data.description;
        if manager.is_administrator() {
            let _res = block ( move || {
                item.create_suspend (
                    manager.id,
                    data.expiration,
                    description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
                    description.clone(),
                    data.expiration,
                )
            }).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn close_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.create_close (
                    manager.id,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    2,
                    data.description.clone(),
                    data.expiration,
                )
            }).await?;
             Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unsuspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.delete_suspend (
                    manager.id,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    3,
                    data.description.clone(),
                    data.expiration,
                )
            }).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unclose_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.delete_close (
                    manager.id,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    4,
                    data.description.clone(),
                    data.expiration,
                )
            }).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unverify_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.unverify (
                    manager.id,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    5,
                    data.description.clone(),
                    data.expiration,
                )
            }).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn reject_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 1);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.target_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.reject (
                    manager.id,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    6,
                    data.description.clone(),
                    data.expiration,
                )
            }).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}