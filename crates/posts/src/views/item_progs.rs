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
    get_community_permission,
    get_user_permission,
    get_owner_data,
    ItemParams, ErrorParams,
    DataCopyPost, DataNewPost, DataEditPost, RespPost,
    ReactionData, JsonItemReactions,
};
use crate::errors::Error;


pub fn item_urls(config: &mut web::ServiceConfig) {
    config.route("/user_fixed/", web::post().to(user_fixed));
    config.route("/user_unfixed/", web::post().to(user_unfixed));
    config.route("/community_fixed/", web::post().to(community_fixed));
    config.route("/community_unfixed/", web::post().to(community_unfixed));
    config.route("/delete_post/", web::post().to(delete_post));
    config.route("/recover_post/", web::post().to(recover_post));
    config.route("/on_comment/", web::post().to(on_comment));
    config.route("/off_comment/", web::post().to(off_comment));
    config.route("/add_post_in_list/", web::post().to(add_post_in_list));
    config.route("/edit_post/", web::put().to(edit_post));
    config.route("/send_reaction_post/", web::post().to(send_reaction_post));
    config.route("/copy_post/", web::post().to(copy_post));
}


pub async fn user_fixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {

    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id > 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
        else {
            if item.user_id == user_id {
                let owner = get_user(item.user_id).expect("E.");
                let _res = block(move || item.user_fixed_post(owner)).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}
pub async fn community_fixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if item.community_id.unwrap() == community_id
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {
                    let _res = block(move || item.community_fixed_post(community)).await?;
                    Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn user_unfixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id > 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
        else {
            if item.user_id == user_id {
                let _res = block(move || item.unfixed_post()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}
pub async fn community_unfixed(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if item.community_id.unwrap() == community_id
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id))
                {
                let _res = block(move || item.unfixed_post()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn delete_post(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if  (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {

                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id == 0 && (item.user_id == user_id || list.user_id == user_id) {
                let owner = get_user(item.user_id).expect("E.");
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}
pub async fn recover_post(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if  (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {

                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id == 0 && (item.user_id == user_id || list.user_id == user_id) {
                let owner = get_user(item.user_id).expect("E.");
                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn on_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if  (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {

                let _res = block(move || item.on_comments()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id == 0 && (item.user_id == user_id || list.user_id == user_id) {
                let owner = get_user(item.user_id).expect("E.");
                let _res = block(move || item.on_comments()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn off_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)
                {
                let _res = block(move || item.off_comments()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id == 0 && (item.user_id == user_id || list.user_id == user_id) {
                let owner = get_user(item.user_id).expect("E.");
                let _res = block(move || item.off_comments()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn add_post_in_list(data: Json<DataNewPost>) -> Result<Json<RespPost>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'list_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Добавьте текст или сведения о прикрепляемых объектах".to_string()))
    }
    else {

        let list = get_post_list(data.list_id.unwrap()).expect("E.");

        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            if community_id > 0 && list.community_id.unwrap() == community_id
                ||
                user_id > 0 && (list.is_user_create_el(user_id) || community.is_user_create_el(user_id))
            {
                let _res = block(move || list.create_post(None, Some(community), data)).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            if community_id == 0 || user_id > 0 && (list.is_user_create_el(user_id) || owner.is_user_create_el(user_id)) {
                let _res = block(move || list.create_post(Some(owner), None, data)).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn edit_post(data: Json<DataEditPost>) -> Result<Json<RespPost>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Добавьте текст или сведения о прикрепляемых объектах".to_string()))
    }
    else {
        let item = get_post(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && item.community_id.unwrap() != community_id)
                ||
                (user_id > 0 && !community.get_editors_ids().iter().any(|&i| i==user_id))
                {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || item.edit_post(data)).await?;
                Ok(Json(_res))
            }
        }
        else {
            if community_id == 0 || item.user_id == user_id {
                let _res = block(move || item.edit_post(data)).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn send_reaction_post(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'item_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.reaction_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'reaction_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let c_id = item.community_id.unwrap();
            if community_id > 0 && c_id != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let community = get_community(c_id).expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false || !list.is_user_see_comment(user_id) {
                    Err(Error::BadRequest(_tuple.1))
                }
                else {
                    let _res = block(move || item.send_reaction (
                        user_id,
                        data.reaction_id.unwrap(),
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
        else {
            if community_id > 0 || user_id == 0 {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let owner = get_user(item.user_id).expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false || !list.is_user_see_comment(user_id) {
                    Err(Error::BadRequest(_tuple.1))
                }
                else {
                    let _res = block(move || item.send_reaction (
                        user_id,
                        data.reaction_id.unwrap(),
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
    }
}

pub async fn copy_post(data: Json<DataCopyPost>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() || (user_id == 0 && community_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'item_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if item.community_id.is_some() {
            let community = get_community(item.community_id.unwrap()).expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false || !list.is_user_copy_el(user_id) || !community.is_user_copy_el(user_id) {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || item.copy_item(data.lists.clone())).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = get_user(item.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false || !list.is_user_copy_el(user_id) || !owner.is_user_copy_el(user_id) {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || item.copy_item(data.lists.clone())).await?;
                Ok(Json(_res))
            }
        }
    }
}
