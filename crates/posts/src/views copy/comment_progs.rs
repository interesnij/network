use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_user,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    get_owner_data,
    ItemParams, ErrorParams,
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
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() { 
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 || community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Fields 'content' or 'attachments' is required!".to_string()))
    }
    else {
        let item = get_post(data.item_id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if list.community_id.is_some() {
            if community_id > 0 && list.community_id.unwrap() != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else 
                let community = list.get_community().expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    Err(Error::BadRequest(_tuple.1))
                }
                else if !list.is_user_create_comment(user_id) || !community.is_user_create_comment(user_id) {
                    Err(Error::BadRequest("Permission Denied".to_string()))
                }
                else {
                    let _res = block(move || item.create_comment (
                        user_id,
                        data.community_id,
                        data.content.clone(),
                        data.parent_id,
                        data.attachments.clone(),
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
        else {
            if community_id > 0 || user_id < 1 {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let owner = get_user(item.user_id).expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    Err(Error::BadRequest(_tuple.1))
                }
                else if !list.is_user_create_comment(user_id) || !owner.is_user_create_comment(user_id) {
                    Err(Error::BadRequest(_tuple.1))
                }
                else {
                    let _res = block(move || item.create_comment (
                        user_id,
                        data.community_id,
                        data.content.clone(),
                        data.parent_id,
                        data.attachments.clone(),
                    )).await?;
                    Ok(Json(_res))
                }
            }
        }
    }
}

pub async fn edit_comment(data: Json<DataEditComment>) -> Result<Json<RespComment>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 || community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.content.is_none() && data.attachments.is_none() {
        Err(Error::BadRequest("Fields 'content' or 'attachments' is required!".to_string()))
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && item.community_id.unwrap() != community_id)
                ||
                (user_id > 0 && !community.get_editors_ids().iter().any(|&i| i==user_id))
                {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || item.edit_comment (
                    data.content.clone(),
                    data.attachments.clone(),
                )).await?;
                Ok(Json(_res))
            }
        }
        else {
            if community_id < 1 || user_id == item.user_id {
                let _res = block(move || item.edit_comment (
                    data.content.clone(),
                    data.attachments.clone(),
                )).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn delete_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 || community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        let list = item.get_list();
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && community.get_moderators_ids().iter().any(|&i| i==user_id))
            {
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if  (community_id < 1 && (user_id == item.user_id || user_id == list.user_id))
                ||
                (community_id > 0 && list.community_id.is_some() && list.community_id.unwrap() == community_id)
            {
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn recover_comment(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 || community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let item = get_post_comment(data.id.unwrap()).expect("E.");
        let list = item.get_list();
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && (item.community_id.unwrap() == community_id || list.community_id.unwrap() == community_id))
                ||
                (user_id > 0 && community.get_moderators_ids().iter().any(|&i| i==user_id))
            {
                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if  (community_id < 1 && (user_id == item.user_id || user_id == list.user_id))
                ||
                (community_id > 0 && list.community_id.is_some() && list.community_id.unwrap() == community_id)
            {
                let _res = block(move || item.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}


pub async fn send_reaction_comment(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 || community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.reaction_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'reaction_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_post_comment(data.item_id.unwrap()).expect("E.");
        let list = item.get_list();
        if item.community_id.is_some() {
            if community_id > 0 && list.community_id.unwrap() != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let community = list.get_community().expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    Err(Error::BadRequest(_tuple.1))
                }
                else if !list.is_user_see_comment(user_id) || !community.is_user_see_comment(user_id) {
                    Err(Error::BadRequest("Permission Denied".to_string()))
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
            if (community_id > 0 && list.community_id.unwrap() != community_id) || user_id == 0 {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let owner = get_user(item.user_id).expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    Err(Error::BadRequest(_tuple.1))
                }
                else if !list.is_user_see_comment(user_id) || !owner.is_user_see_comment(user_id) {
                    Err(Error::BadRequest("Permission Denied".to_string()))
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
