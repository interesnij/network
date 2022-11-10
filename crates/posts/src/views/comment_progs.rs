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
    get_owner_data,
    ItemParams,
    DataNewComment, DataEditComment, RespComment,
    ReactionData, JsonItemReactions,
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
    let (err, user_id, community_id) = get_owner_data(params.token, params.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Добавьте текст или сведения о прикрепляемых объектах".to_string()));
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let list = get_post_list(item.post_list_id).expect("E.");
        if item.community_id.is_some() {
            let c_id = item.community_id.unwrap();
            if community_id > 0 && c_id != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
            else {
                let community = get_community(c_id).expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false || !list.is_user_create_comment(user_id) {
                    Err(Error::BadRequest(_tuple.1))
                }
                else {
                    let _res = block(move || item.create_comment (
                        user_id,
                        data.community_id,
                        data.content,
                        data.parent_id,
                        data.attachments,
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
        else {
            if community_id > 0 || user_id == 0 {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
            else {
                let owner = get_user(item.user_id).expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false || !list.is_user_create_comment(user_id) {
                    Err(Error::BadRequest(_tuple.1))
                }
                else {
                    let _res = block(move || item.create_comment (
                        user_id,
                        data.community_id,
                        data.content,
                        data.parent_id,
                        data.attachments,
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
    }
}

pub async fn edit_comment(data: Json<DataEditComment>) -> Result<Json<RespComment>, Error> {
    let (err, user_id, community_id) = get_owner_data(params.token, params.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Добавьте текст или сведения о прикрепляемых объектах".to_string()));
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        //let list = get_post_list(item.post_list_id).expect("E.");
        if item.community_id.is_some() {
            let c_id = item.community_id.unwrap();
            if community_id > 0 && c_id != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
            else {
                let _res = block(move || item.edit_comment (
                    data.content,
                    data.attachments,
                )).await?;
                Ok(Json(_res))
            }
        }
        else {
            if community_id > 0 || user_id != item.user_id {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
            else {
                let _res = block(move || item.edit_comment (
                    data.content,
                    data.attachments,
                )).await?;
                Ok(Json(_res))
            }
        }
    }
}

pub async fn delete_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(params.token, params.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        let list = item.get_list();
        if item.community_id.is_some() {
            if (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && list.user_id == user_id)
            {
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
        }
        else {
            if  (community_id == 0 && (user_id == item.user_id || user_id == list.user_id))
                ||
                (community_id > 0 && list.community_id.is_some() && list.community_id.unwrap() == community_id)
            {
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
        }
    }
}

pub async fn recover_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(params.token, params.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        let list = item.get_list();
        if item.community_id.is_some() {
            if (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && list.user_id == user_id)
            {
                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
        }
        else {
            if  (community_id == 0 && (user_id == item.user_id || user_id == list.user_id))
                ||
                (community_id > 0 && list.community_id.is_some() && list.community_id.unwrap() == community_id)
            {
                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()));
            }
        }
    }
}

pub async fn send_reaction_comment(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let comment = get_post_comment(data.id.unwrap()).expect("E.");
    if comment.community_id.is_some() {
        let community = comment.get_community().expect("E.");
        let _tuple = get_community_permission(&community, data.user_id.unwrap());
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
        let _tuple = get_user_permission(&owner, data.user_id.unwrap());
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || comment.send_reaction(data)).await?;
            Ok(Json(_res))
        }
    }
}
