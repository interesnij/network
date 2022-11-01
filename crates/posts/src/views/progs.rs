use crate::schema;
use actix_web::{
    HttpResponse,
    web,
    web::block,
    web::Json,
    Responder,
};
use crate::utils::{
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
    NewListValues,
    ErrorParams, InfoParams,
    DataListJson, RespListJson,
};
use crate::models::{
    User, Community,
    PostList, Post, PostComment,

    NewUserJson, NewCommunityJson,
};
use serde::{Deserialize, Serialize};
use crate::errors::Error;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    config.route("/create_community/", web::post().to(create_community));
    config.route("/add_user_list/", web::post().to(add_user_list));
    config.route("/edit_user_list/", web::post().to(edit_user_list));
    config.route("/add_community_list/", web::post().to(add_community_list));
    config.route("/edit_community_list/", web::post().to(edit_community_list));

}

pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}
pub async fn add_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let _res = block(move || PostList::create_list(data)).await?;
    Ok(Json(_res))
}
pub async fn edit_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let list = get_post_list(data.id).expect("E.");
    if list.user_id != data.user_id {
        Error::BadRequest("Permission Denied".to_string())
    }
    else {
        let _res = block(move || PostList::edit_list(data)).await?;
        Ok(Json(_res))
    }
}
pub async fn add_community_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    if data.community_id.is_some() {
        let community = get_community(data.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Error::BadRequest(_tuple.1)
        }
        else {
            let _res = block(move || PostList::create_list(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        Error::BadRequest("Permission Denied".to_string())
    }
}
pub async fn edit_community_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    if data.community_id.is_some() {
        let community = get_community(data.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Error::BadRequest(_tuple.1)
        }
        else {
            let _res = block(move || PostList::edit_list(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        Error::BadRequest("Permission Denied".to_string())
    }
}
