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
    ItemParams,
    DataListJson, RespListJson,
    DataNewPost, DataEditPost, RespPost,
    DataNewComment, DataEditComment, RespComment,
    ReactionData, JsonItemReactions,
};
use crate::models::{
    User, Community,
    PostList,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;


pub fn comment_urls(config: &mut web::ServiceConfig) {
    config.route("/add_comment/", web::post().to(add_comment));
    config.route("/edit_comment/", web::put().to(edit_comment));
    config.route("/delete_comment/", web::post().to(delete_comment));
    config.route("/recover_comment/", web::post().to(recover_comment));
    config.route("/send_reaction_comment/", web::post().to(send_reaction_comment));
}


pub async fn add_comment(data: Json<DataNewComment>) -> Result<Json<RespComment>, Error> {
    let item = get_post(data.post_id).expect("E.");
    let list = get_post_list(item.post_list_id).expect("E.");
    if item.community_id.is_some() {
        let community = get_community(item.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false || !list.is_user_create_comment(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.create_comment(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(item.user_id).expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false || !list.is_user_create_comment(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || item.create_comment(data)).await?;
            Ok(Json(_res))
        }
    }
}

pub async fn edit_comment(data: Json<DataEditComment>) -> Result<Json<RespComment>, Error> {
    let comment = get_post_comment(data.id).expect("E.");
    let list = comment.get_list();
    if comment.community_id.is_some() {
        let community = comment.get_community().expect("E.");
        if comment.user_id == data.user_id || community.get_editors_ids().iter().any(|&i| i==data.user_id) {
            let _res = block(move || comment.edit_comment(data)).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
    else {
        if comment.user_id == data.user_id || list.user_id == data.user_id {
            let _res = block(move || comment.edit_comment(data)).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn delete_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let comment = get_post_comment(data.id).expect("E.");
    let list = comment.get_list();
    if comment.community_id.is_some() {
        let community = comment.get_community().expect("E.");
        if comment.user_id == data.user_id || community.get_editors_ids().iter().any(|&i| i==data.user_id) {
            let _res = block(move || comment.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
    else {
        if comment.user_id == data.user_id || list.user_id == data.user_id {
            let _res = block(move || comment.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn recover_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let comment = get_post_comment(data.id).expect("E.");
    let list = comment.get_list();
    if comment.community_id.is_some() {
        let community = comment.get_community().expect("E.");
        if comment.user_id == data.user_id || community.get_editors_ids().iter().any(|&i| i==data.user_id) {
            let _res = block(move || comment.restore_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
    else {
        if comment.user_id == data.user_id || list.user_id == data.user_id {
            let _res = block(move || comment.restore_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn send_reaction_comment(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let comment = get_post_comment(data.id).expect("E.");
    if comment.community_id.is_some() {
        let community = comment.get_community().expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || comment.send_reaction(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = comment.get_creator().expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || comment.send_reaction(data)).await?;
            Ok(Json(_res))
        }
    }
}
