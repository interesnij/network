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
    get_user,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
    NewListValues, ItemParams,
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
    config.route("/delete_user_list/", web::post().to(delete_user_list));
    config.route("/recover_user_list/", web::post().to(recover_user_list));
    config.route("/delete_community_list/", web::post().to(delete_community_list));
    config.route("/recover_community_list/", web::post().to(recover_community_list));

    config.route("/fixed/", web::post().to(fixed));
    config.route("/unfixed/", web::post().to(unfixed));
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
        Err(Error::BadRequest("Permission Denied".to_string()))
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
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || PostList::create_list(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}
pub async fn edit_community_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    if data.community_id.is_some() {
        let community = get_community(data.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false || !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || PostList::edit_list(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}

pub async fn delete_user_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id).expect("E.");
    if list.user_id != data.user_id || list.community_id.is_some() {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let _res = block(move || list.delete_item()).await?;
        Ok(Json(_res))
    }
}
pub async fn recover_user_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id).expect("E.");
    if list.user_id != data.user_id || list.community_id.is_some() {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let _res = block(move || list.restore_item()).await?;
        Ok(Json(_res))
    }
}
pub async fn delete_community_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.community_id.is_some() {
        let list = get_post_list(data.id).expect("E.");
        let community = get_community(list.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false || !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.delete_item()).await?;
            Ok(Json(_res))
        }
    }
    else {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}
pub async fn recover_community_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.community_id.is_some() {
        let list = get_post_list(data.id).expect("E.");
        let community = get_community(list.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false || !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.restore_item()).await?;
            Ok(Json(_res))
        }
    }
    else {

        Err(Error::BadRequest("Permission Denied".to_string()))
    }
}

pub async fn fixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.community_fixed_post(community)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.user_fixed_post(owner)).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn unfixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.unfixed_post()).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.unfixed_post()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}
