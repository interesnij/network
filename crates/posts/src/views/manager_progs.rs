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
    get_owner_data,
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
    pub token:       String,
    pub id:          i32,
    pub user_id:     i32,
    pub item_id:     i32,
    pub types:       i16,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct CloseParams {
    pub token:       String,
    pub id:          i32,
    pub user_id:     i32,
    pub item_id:     i32,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct ModerationParams {
    pub token:       String,
    pub id:          i32,
    pub user_id:     i32,
    pub item_id:     i32,
    pub description: Option<String>,
    pub expiration:  Option<chrono::NaiveDateTime>,
}

pub async fn create_claim_list(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let item = get_post_list(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                3,
            )).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                3,
            )).await?;
            Ok(Json(_res))
        }
    }
}
pub async fn create_claim_post(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                4,
            )).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                4,
            )).await?;
            Ok(Json(_res))
        }
    }
}
pub async fn create_claim_comment(data: Json<ReportParams>) -> Result<Json<i16>, Error> {
    use crate::models::ModeratedReport;

    let item = get_post_comment(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                5,
            )).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || ModeratedReport::create (
                data.user_id,
                data.types,
                data.id,
                data.description.clone(),
                4,
            )).await?;
            Ok(Json(_res))
        }
    }
}

pub async fn close_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_user(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
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
pub async fn close_community(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_community(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    2,
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
pub async fn unclose_user(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_user(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
                    data.description.clone(),
                    4,
                    None
                );
                item.unclose_item()
            }
        ).await?;
        Ok(Json(_res))
    }
    else {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}
pub async fn unclose_community(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_community(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    2,
                    data.description.clone(),
                    4,
                    None
                );
                item.unclose_item()
            }
        ).await?;
        Ok(Json(_res))
    }
    else {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}

pub async fn close_list(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post_list(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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
pub async fn close_post(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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
pub async fn close_comment(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post_comment(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn unclose_list(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post_list(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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
pub async fn unclose_post(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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
pub async fn unclose_comment(data: Json<CloseParams>) -> Result<Json<i16>, Error> {
    let item = get_post_comment(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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


pub async fn suspend_community(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_community(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    2,
                    data.description.clone(),
                    1,
                    data.expiration
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
pub async fn unsuspend_community(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_community(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    2,
                    data.description.clone(),
                    3,
                    data.expiration
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

pub async fn suspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_user(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
                    data.description.clone(),
                    1,
                    data.expiration
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
pub async fn unsuspend_user(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_user(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    1,
                    data.description.clone(),
                    3,
                    data.expiration
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

pub async fn suspend_list(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_post_list(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    3,
                    data.description.clone(),
                    1,
                    data.expiration
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
pub async fn unsuspend_list(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_post_list(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
    if manager.is_administrator() {
        let _res = block (
            move || {
                ModeratedLog::create (
                    manager.id,
                    item.id,
                    3,
                    data.description.clone(),
                    3,
                    data.expiration
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

pub async fn suspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn close_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn unsuspend_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn unclose_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn unverify_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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

pub async fn reject_moderation(data: Json<ModerationParams>) -> Result<Json<i16>, Error> {
    let item = get_moderation(data.id).expect("E.");
    let manager = get_user(data.user_id).expect("E.");
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
