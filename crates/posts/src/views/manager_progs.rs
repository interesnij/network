use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_community,
    get_user,
    get_post_list,
    get_post,
    get_post_comment,
    get_moderation,
    get_community_permission,
    get_user_permission,
    get_user_owner_data,
    get_owner_data,
    ErrorParams, AttachOwner,
    TOKEN,
};
use crate::models::ModeratedLog;
use crate::errors::Error;
use serde::Deserialize; 
use crate::AppState;


pub fn manager_urls(config: &mut web::ServiceConfig) {
    config.route("/create_claim_list", web::post().to(create_claim_list));
    config.route("/create_claim_post", web::post().to(create_claim_post));
    config.route("/create_claim_comment", web::post().to(create_claim_comment));

    config.route("/close_community", web::post().to(close_community));
    config.route("/close_user", web::post().to(close_user));
    config.route("/close_list", web::post().to(close_list));
    config.route("/close_post", web::post().to(close_post));
    config.route("/close_comment", web::post().to(close_comment));
    config.route("/unclose_community", web::post().to(unclose_community));
    config.route("/unclose_user", web::post().to(unclose_user));
    config.route("/unclose_list", web::post().to(unclose_list));
    config.route("/unclose_post", web::post().to(unclose_post));
    config.route("/unclose_comment", web::post().to(unclose_comment));

    config.route("/suspend_community", web::post().to(suspend_community));
    config.route("/suspend_user", web::post().to(suspend_user));
    config.route("/suspend_list", web::post().to(suspend_list));
    config.route("/unsuspend_community", web::post().to(unsuspend_community));
    config.route("/unsuspend_user", web::post().to(unsuspend_user));
    config.route("/unsuspend_list", web::post().to(unsuspend_list));

    config.route("/suspend_moderation", web::post().to(suspend_moderation));
    config.route("/close_moderation", web::post().to(close_moderation));
    config.route("/unclose_moderation", web::post().to(unclose_moderation));
    config.route("/unsuspend_moderation", web::post().to(unsuspend_moderation));
    config.route("/unverify_moderation", web::post().to(unverify_moderation));
    config.route("/reject_moderation", web::post().to(reject_moderation));

    config.route("/edit_user_staff", web::post().to(edit_user_staff));
    config.route("/edit_member_staff", web::post().to(edit_member_staff));
    config.route("/edit_user_private", web::post().to(edit_user_private));
    config.route("/edit_user_all_private", web::post().to(edit_user_all_private));
    config.route("/edit_community_private", web::post().to(edit_community_private));
    config.route("/edit_list_private", web::post().to(edit_list_private));
}

#[derive(Deserialize)]
pub struct ReportParams {
    pub token:       Option<String>,
    pub id:          Option<i32>,
    pub item_id:     Option<i32>,
    pub types:       Option<i16>,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct CloseParams {
    pub token:       Option<String>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct ModerationParams {
    pub token:       Option<String>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
    pub expiration:  Option<chrono::NaiveDateTime>,
}

pub async fn create_claim_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ReportParams>
) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
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
        let item = get_post_list(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = get_community(item.community_id.unwrap()).expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    3,
                )).await?;
                Ok(Json(_res)) 
            }
        }
        else {
            let owner = get_user(item.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    3,
                )).await?;
                Ok(Json(_res))
            }
        }
    }
}
pub async fn create_claim_post (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ReportParams>
) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
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
        let item = get_post(data.id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if list.community_id.is_some() {
            let community = get_community(list.community_id.unwrap()).expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    4,
                )).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    4,
                )).await?;
                Ok(Json(_res))
            }
        }
    }
}
pub async fn create_claim_comment (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ReportParams>
) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
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
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        let list = item.get_list();
        if list.community_id.is_some() {
            let community = get_community(list.community_id.unwrap()).expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    5,
                )).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || ModeratedReport::create (
                    user_id,
                    data.types.unwrap(),
                    item.id,
                    data.description.clone(),
                    5,
                )).await?;
                Ok(Json(_res))
            }
        }
    }
}

// веерное событие
pub async fn close_user (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.close_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn close_community (
    data: Json<CloseParams>,
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.close_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn unclose_user (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.close_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn unclose_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.close_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn close_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_list(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        data.description.clone(),
                        2,
                        None
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
pub async fn close_post (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        4,
                        data.description.clone(),
                        2,
                        None
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
pub async fn close_comment (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_comment(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        5,
                        data.description.clone(),
                        2,
                        None
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

pub async fn unclose_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_list(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        data.description.clone(),
                        4,
                        None
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
pub async fn unclose_post (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        4,
                        data.description.clone(),
                        4,
                        None
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
pub async fn unclose_comment (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CloseParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_comment(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_moderator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        5,
                        data.description.clone(),
                        4,
                        None
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

// веерное событие
pub async fn suspend_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.suspend_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn unsuspend_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_community(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.unsuspend_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn suspend_user (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is requiredd!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.suspend_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// веерное событие
pub async fn unsuspend_user (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_user(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.unsuspend_item()
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn suspend_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_list(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        data.description.clone(),
                        1,
                        data.expiration,
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
pub async fn unsuspend_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_list(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || {
                    ModeratedLog::create (
                        manager.id,
                        item.id,
                        3,
                        data.description.clone(),
                        3,
                        data.expiration,
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

pub async fn suspend_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.create_suspend (
                    manager.id,
                    data.expiration,
                    data.description.clone(),

                )).await?;
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
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.create_close (
                    manager.id,
                    data.description.clone(),
                )).await?;
                Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unsuspend_moderation (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ModerationParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.delete_suspend (
                    manager.id,
                    data.description.clone(),
                )).await?;
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
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.delete_close (
                    manager.id,
                    data.description.clone(),
                )).await?;
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
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.unverify (
                    manager.id,
                    data.description.clone(),
                )).await?;
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
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_moderation(data.item_id.unwrap()).expect("E.");
        let manager = get_user(user_id).expect("E.");
        if manager.is_administrator() {
            let _res = block (
                move || item.reject (
                    manager.id,
                    data.description.clone(),
                )).await?;
                Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct UStaffParams {
    pub token:     Option<String>,
    pub target_id: Option<i32>,
    pub types:     Option<i16>,
}

pub async fn edit_user_staff (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UStaffParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 0).await;
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
        let request_user = get_user(user_id).expect("E.");
        let target_user = get_user(data.target_id.unwrap()).expect("E.");
        let types = data.types.unwrap();
        let check = match request_user.types {
            13 => types > 9 && types < 13,
            17 => types > 13 && types < 17,
            20 => types > 17 && types < 20,
            23 => types > 20 && types < 23,
            25 => true,
            _ => false,
        };
        if check {
            let _res = block (
                move || target_user.change_staff(types)
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct CStaffParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub types:        Option<i16>,
}

pub async fn edit_member_staff (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CStaffParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 0).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.community_id.is_none() && community_id < 1 {
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
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community = get_community(c_id).expect("E.");
        
        if community.is_user_admin(user_id) {
            let _res = block (
                move || community.update_staff_member(user_id, data.types.unwrap())
            ).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct UPrivateParams {
    pub token:   Option<String>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<AttachOwner>>,
}

pub async fn edit_user_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UPrivateParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.field.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'field' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.value.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'value' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let request_user = get_user(user_id).expect("E.");
        let _res = block (
            move || request_user.edit_private(
                data.field.as_deref().unwrap(),
                data.value.unwrap(),
                data.users.clone(),
            )
        ).await?;
        Ok(Json(_res))
    }
}

#[derive(Deserialize)]
pub struct AllPrivateData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<AttachOwner>>,
} 

pub async fn edit_user_all_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<AllPrivateData>
) -> Result<Json<i16>, Error> {
    if data.token.is_none() || data.value.is_none() || data.user_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Fields 'value', 'user_id', 'token' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN { 
            let request_user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block (
                move || request_user.edit_private (
                    "see_all",
                    data.value.unwrap(),
                    data.users.clone(),
                )
                ).await?;
            Ok(Json(_res))
        } else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct CPrivateParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub field:        Option<String>,
    pub value:        Option<i16>,
    pub users:        Option<Vec<AttachOwner>>,
}

pub async fn edit_community_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CPrivateParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.field.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'field' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.community_id.is_none() && community_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.value.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'value' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community = get_community(c_id).expect("E.");
        let _res = block (
            move || community.edit_private(
                data.field.as_deref().unwrap(),
                data.value.unwrap(),
                data.users.clone(),
            )
        ).await?;
        Ok(Json(_res))
    }
}

#[derive(Deserialize)]
pub struct LPrivateParams {
    pub token:   Option<String>,
    pub list_id: Option<i32>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<AttachOwner>>,
}

pub async fn edit_list_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<LPrivateParams>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 21).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.field.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'field' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.value.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'value' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_post_list(data.list_id.unwrap()).expect("E.");
        if list.community_id.is_some() {
            let c_id = list.community_id.unwrap();
            let community = get_community(c_id).expect("E.");
            if community_id == c_id || community.is_user_admin(user_id) {
                let _res = block (
                    move || list.edit_private(
                        data.field.as_deref().unwrap(),
                        data.value.unwrap(),
                        data.users.clone(),
                    )
                ).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if user_id == list.user_id {
                let _res = block (
                    move || list.edit_private(
                        data.field.as_deref().unwrap(),
                        data.value.unwrap(),
                        data.users.clone(),
                )
                ).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}