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
    get_community_permission,
    get_user_permission,
};
use crate::models::{
    User, Community,
    PostList,
};
use crate::errors::Error;
use serde::Deserialize;


pub fn manager_urls(config: &mut web::ServiceConfig) {
    config.route("/create_claim_list/", web::post().to(create_claim_list));
    config.route("/create_claim_post/", web::post().to(create_claim_post));
    config.route("/create_claim_comment/", web::post().to(create_claim_comment));

    //config.route("/close_community/", web::post().to(close_community));
    //config.route("/close_user/", web::post().to(close_user));
    //config.route("/close_list/", web::post().to(close_list));
    //config.route("/close_post/", web::post().to(close_post));
    //config.route("/close_comment/", web::post().to(close_comment));
    //config.route("/unclose_community/", web::post().to(unclose_community));
    //config.route("/unclose_user/", web::post().to(unclose_user));
    //config.route("/unclose_list/", web::post().to(unclose_list));
    //config.route("/unclose_post/", web::post().to(unclose_post));
    //config.route("/unclose_comment/", web::post().to(unclose_comment));

    //config.route("/suspend_community/", web::post().to(suspend_community));
    //config.route("/suspend_user/", web::post().to(suspend_user));
    //config.route("/suspend_list/", web::post().to(suspend_list));
    //config.route("/unsuspend_community/", web::post().to(unsuspend_community));
    //config.route("/unsuspend_user/", web::post().to(unsuspend_user));
    //config.route("/unsuspend_list/", web::post().to(unsuspend_list));
}

#[derive(Deserialize)]
pub struct ReportParams {
    pub id:          i32,
    pub user_id:     i32,
    pub item_id:     i32,
    pub types:       i16,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct CloseParams {
    pub id:      i32,
    pub user_id: i32,
    pub item_id: i32,
}
#[derive(Deserialize)]
pub struct SuspendParams {
    pub id:         i32,
    pub user_id:    i32,
    pub item_id:    i32,
    pub expiration: Option<chrono::NaiveDateTime>,
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
                data.description,
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
                data.description,
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
                data.description,
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
                data.description,
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
                data.description,
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
                data.description,
                4,
            )).await?;
            Ok(Json(_res))
        }
    }
}
