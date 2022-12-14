use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user, 
    get_moderation, get_user_owner_data,
    get_community_permission, 
    get_community,
    ErrorParams, KeyValue, 
    //CardUserJson, 
    CardCommunityJson,
};
use crate::models::ModeratedLog;
use serde::{Deserialize, Serialize}; 
use crate::errors::Error;
use crate::AppState;


pub fn manager_urls(config: &mut web::ServiceConfig) {
    config.route("/get_claim", web::get().to(get_claim_page));
    config.route("/create_claim_community", web::post().to(create_claim_community));
    config.route("/close_community", web::post().to(close_community));
    config.route("/unclose_community", web::post().to(unclose_community));
    config.route("/suspend_community", web::post().to(suspend_community));
    config.route("/unsuspend_community", web::post().to(unsuspend_community));
    config.route("/suspend_moderation", web::post().to(suspend_moderation));
    config.route("/close_moderation", web::post().to(close_moderation));
    config.route("/unclose_moderation", web::post().to(unclose_moderation));
    config.route("/unsuspend_moderation", web::post().to(unsuspend_moderation));
    config.route("/unverify_moderation", web::post().to(unverify_moderation));
    config.route("/reject_moderation", web::post().to(reject_moderation));
}

#[derive(Deserialize)]
pub struct ReportParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub types:        Option<i16>,
    pub description:  Option<String>,
}
#[derive(Deserialize)]
pub struct CloseParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub description:  Option<String>,
}
#[derive(Deserialize)]
pub struct ModerationParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub description:  Option<String>,
    pub expiration:   Option<chrono::NaiveDateTime>,
}
#[derive(Serialize)]
pub struct ReportResp {
    pub options:   Vec<KeyValue>,
    pub community: CardCommunityJson,
} 

pub async fn get_claim_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<ReportResp>, Error> {
    let params_some = web::Query::<ReportParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 1).await;
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
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'community_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner_res = get_user(user_id);
            let target_res = get_community(params.community_id.unwrap());
            if owner_res.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner user not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            else if target_res.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "target community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            let target = target_res.expect("E.");
            let _tuple = get_community_permission(&target, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let mut list = Vec::new();
                let card = CardCommunityJson { 
                    id:      target.id,
                    name:    target.name,
                    link:    target.link,
                    image:   target.s_avatar,
                    members: target.members,
                };
                list.push(KeyValue {
                    value: 1,
                    info:  "?????????????? / ??????????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 2,
                    info:  "???????????????????????????? ????????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 3,
                    info:  "??????????????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 4,
                    info:  "??????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 5,
                    info:  "?????????????? ????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 6,
                    info:  "??????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 7,
                    info:  "???????????? ?? ????????????".to_string(),
                });

                list.push(KeyValue {
                    value: 8,
                    info:  "???????????? ?? ??????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 9,
                    info:  "???????????????? ?????????????????? c ??????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 10,
                    info:  "???????????????? ?? ??????????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 11,
                    info:  "????????????????????".to_string(),
                });
                list.push(KeyValue {
                    value: 12,
                    info:  "???????????????? ??????????????????".to_string(),
                });

                Ok(Json(ReportResp {
                    options:   list,
                    community: card,
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

pub async fn create_claim_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ReportParams>
) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
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
        let owner = get_community(data.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&owner, user_id);
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

pub async fn close_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.community_id.unwrap()).expect("E.");
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
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unclose_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.community_id.unwrap()).expect("E.");
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
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn suspend_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is requiredd!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.community_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
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
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unsuspend_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.community_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        data.description.clone(),
                        data.expiration,
                    );
                    item.unsuspend_item()
                }
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn suspend_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block ( move || {
                item.create_suspend (
                    manager.id,
                    data.expiration,
                    data.description.clone(),
                );
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
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

pub async fn close_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
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

pub async fn unsuspend_moderation (
    data: Json<ModerationParams>,
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
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

pub async fn unclose_moderation ( 
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
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

pub async fn unverify_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
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

pub async fn reject_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 1).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.community_id.unwrap()).expect("E.");
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