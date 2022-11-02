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
    DataNewPost, DataEditPost, RespPost,
    ReactionData, JsonItemReactions,
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
    config.route("/delete_post/", web::post().to(delete_post));
    config.route("/recover_post/", web::post().to(recover_post));
    config.route("/on_comment/", web::post().to(on_comment));
    config.route("/off_comment/", web::post().to(off_comment));
    config.route("/add_post_in_list/", web::post().to(add_post_in_list));
    config.route("/edit_post/", web::put().to(edit_post));
    config.route("/send_reaction_post/", web::post().to(send_reaction_post));
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

pub async fn delete_post(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.delete_item()).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}
pub async fn recover_post(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.restore_item()).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn on_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.on_comments()).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.on_comments()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn off_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.off_comments()).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.on_comments()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn add_post_in_list(data: Json<DataNewPost>) -> Result<Json<RespPost>, Error> {
    let list = get_post_list(data.list_id).expect("E.");
    if list.community_id.is_some() {
        let community = get_community(list.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.create_post(None, Some(community), data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(list.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || list.create_post(Some(owner), None, data)).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn edit_post(data: Json<DataEditPost>) -> Result<Json<RespPost>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.edit_post(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        if owner.id == data.user_id {
            let _res = block(move || item.edit_post(data)).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn send_reaction_post(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let item = get_post(data.id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.send_reaction(data)).await?;
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
            let _res = block(move || item.send_reaction(data)).await?;
            Ok(Json(_res))
        }
    }
}
