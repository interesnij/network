use actix_web::{
    web,
    web::block,
    web::Json,
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
    ErrorParams,
};
use crate::models::ModeratedLog;
use crate::errors::Error;
use serde::Deserialize; 


pub fn manager_urls(config: &mut web::ServiceConfig) {
    config.route("/create_claim_list/", web::post().to(create_claim_list));
    config.route("/create_claim_post/", web::post().to(create_claim_post));
    config.route("/create_claim_comment/", web::post().to(create_claim_comment));

    config.route("/close_community/", web::post().to(close_community));
    config.route("/close_user/", web::post().to(close_user));
    config.route("/close_list/", web::post().to(close_list));
    config.route("/close_post/", web::post().to(close_post));
    config.route("/close_comment/", web::post().to(close_comment));
    config.route("/unclose_community/", web::post().to(unclose_community));
    config.route("/unclose_user/", web::post().to(unclose_user));
    config.route("/unclose_list/", web::post().to(unclose_list));
    config.route("/unclose_post/", web::post().to(unclose_post));
    config.route("/unclose_comment/", web::post().to(unclose_comment));

    config.route("/suspend_community/", web::post().to(suspend_community));
    config.route("/suspend_user/", web::post().to(suspend_user));
    config.route("/suspend_list/", web::post().to(suspend_list));
    config.route("/unsuspend_community/", web::post().to(unsuspend_community));
    config.route("/unsuspend_user/", web::post().to(unsuspend_user));
    config.route("/unsuspend_list/", web::post().to(unsuspend_list));

    config.route("/suspend_moderation/", web::post().to(suspend_moderation));
    config.route("/close_moderation/", web::post().to(close_moderation));
    config.route("/unclose_moderation/", web::post().to(unclose_moderation));
    config.route("/unsuspend_moderation/", web::post().to(unsuspend_moderation));
    config.route("/unverify_moderation/", web::post().to(unverify_moderation));
    config.route("/reject_moderation/", web::post().to(reject_moderation));
}

#[derive(Deserialize)]
pub struct ReportParams {
    pub token:       Option<String>,
    pub id:          Option<i32>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub types:       Option<i16>,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct CloseParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct ModerationParams {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub description: Option<String>,
    pub expiration:  Option<chrono::NaiveDateTime>,
}

pub async fn create_claim_list(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn create_claim_post(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn create_claim_comment(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn close_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn close_community(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unclose_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unclose_community(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn close_list(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn close_post(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn close_comment(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn unclose_list(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unclose_post(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unclose_comment(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn suspend_community(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unsuspend_community(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn suspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unsuspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn suspend_list(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
pub async fn unsuspend_list(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn suspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn close_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn unsuspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn unclose_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn unverify_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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

pub async fn reject_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id);
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
