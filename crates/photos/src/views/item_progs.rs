use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_community,
    get_user,
    get_photo_list,
    get_photo,
    get_community_permission,
    get_user_permission,
    get_owner_data,
    ItemParams, ErrorParams,
    DataCopyPhoto, DataNewPhotos, DataEditPhoto, RespPhoto,
    ReactionData, JsonItemReactions,
};
use crate::errors::Error;


pub fn item_urls(config: &mut web::ServiceConfig) {
    config.route("/delete_photo", web::post().to(delete_photo));
    config.route("/recover_photo", web::post().to(recover_photo));
    config.route("/on_comment", web::post().to(on_comment));
    config.route("/off_comment", web::post().to(off_comment));
    config.route("/add_photos_in_list", web::post().to(add_photos_in_list));
    config.route("/edit_photo", web::put().to(edit_photo));
    config.route("/send_reaction_photo", web::post().to(send_reaction_photo));
    config.route("/copy_photo", web::post().to(copy_photo));
}


pub async fn delete_photo(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.id.unwrap()).expect("E.");
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
            if community_id < 1 && (item.user_id == user_id || list.user_id == user_id) {
                let _res = block(move || item.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}
pub async fn recover_photo(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.id.unwrap()).expect("E.");
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
            if community_id < 1 && (item.user_id == user_id || list.user_id == user_id) {
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
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.id.unwrap()).expect("E.");
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
            if community_id < 1 && (item.user_id == user_id || list.user_id == user_id) {
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
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.id.unwrap()).expect("E.");
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
            if community_id < 1 && (item.user_id == user_id || list.user_id == user_id) {
                let _res = block(move || item.off_comments()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn add_photos_in_list(data: Json<DataNewPhotos>) -> Result<Json<Vec<RespPhoto>>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), Some(data.user_id), 21);
    if err.is_some() { 
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let list = get_photo_list(data.list_id).expect("E.");
        let c_id: Option<i32>;
        if community_id > 0 {
            c_id = Some(community_id);
        }
        else {
            c_id = list.community_id;
        }

        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            if community_id > 0 && list.community_id.unwrap() == community_id
                ||
                user_id > 0 && (list.is_user_create_el(user_id) || community.is_user_create_el(user_id))
            {
                let _res = block(move || list.create_photos (
                    c_id,
                    user_id,
                    data.server_id,
                    data.files.clone(),
                )).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            if community_id < 1 || user_id > 0 && (list.is_user_create_el(user_id) || owner.is_user_create_el(user_id)) {
                let _res = block(move || list.create_photos (
                    c_id,
                    user_id,
                    data.server_id,
                    data.files.clone(),
                )).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn edit_photo(data: Json<DataEditPhoto>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() { 
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.id.unwrap()).expect("E.");
        if item.community_id.is_some() {
            let community = item.get_community().expect("E.");
            if (community_id > 0 && item.community_id.unwrap() != community_id)
                ||
                (user_id > 0 && !community.get_editors_ids().iter().any(|&i| i==user_id))
                {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || item.edit_photo(data.description.clone())).await?;
                Ok(Json(_res))
            }
        }
        else {
            if community_id < 1 && item.user_id == user_id {
                let _res = block(move || item.edit_photo(data.description.clone())).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn send_reaction_photo(data: Json<ReactionData>) -> Result<Json<JsonItemReactions>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.reaction_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'reaction_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.item_id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if list.community_id.is_some() {
            let c_id = list.community_id.unwrap();
            if community_id > 0 && c_id != community_id {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let community = get_community(c_id).expect("E.");
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
            if community_id > 0 || user_id == 0 {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let owner = get_user(list.user_id).expect("E.");
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

pub async fn copy_photo(data: Json<DataCopyPhoto>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.item_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'item_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let item = get_photo(data.item_id.unwrap()).expect("E.");
        let list = item.get_list().expect("E.");
        if list.community_id.is_some() {
            let community = get_community(list.community_id.unwrap()).expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else if !list.is_user_copy_el(user_id) || !community.is_user_copy_el(user_id) {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || item.copy_item(data.lists.clone())).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = get_user(item.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else if !list.is_user_copy_el(user_id) || !owner.is_user_copy_el(user_id) {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || item.copy_item(data.lists.clone())).await?;
                Ok(Json(_res))
            }
        }
    }
}
