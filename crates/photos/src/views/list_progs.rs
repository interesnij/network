use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_community,
    get_user,
    get_photo_list,
    get_community_permission,
    get_user_permission,
    get_owner_data,
    ItemParams, DataCopyList, ErrorParams,
    DataListJson, RespListJson,
};
use crate::models::PhotoList;
use crate::errors::Error;


pub fn list_urls(config: &mut web::ServiceConfig) {
    config.route("/add_list_in_user_collection/", web::post().to(add_list_in_user_collection));
    config.route("/add_list_in_community_collection/", web::post().to(add_list_in_community_collection));
    config.route("/delete_list_from_user_collection/", web::post().to(delete_list_from_user_collection));
    config.route("/delete_list_from_community_collection/", web::post().to(delete_list_from_community_collection));
    config.route("/add_user_list/", web::post().to(add_user_list));
    config.route("/edit_user_list/", web::post().to(edit_user_list));
    config.route("/add_community_list/", web::post().to(add_community_list));
    config.route("/edit_community_list/", web::post().to(edit_community_list));
    config.route("/delete_list/", web::post().to(delete_list));
    config.route("/recover_list/", web::post().to(recover_list));
    config.route("/copy_list/", web::put().to(copy_list));
}

pub async fn add_list_in_user_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id > 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            let _tuple = get_community_permission(&community, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || list.add_in_user_collections(user_id)).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = list.get_creator().expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else {
                let _res = block(move || list.add_in_user_collections(user_id)).await?;
                Ok(Json(_res))
            }
        }
    }
}
pub async fn delete_list_from_user_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id > 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        let _res = block(move || list.remove_in_user_collections(user_id)).await?;
        Ok(Json(_res))
    }
}

pub async fn add_list_in_community_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
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
    else if community_id < 1 && data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if list.community_id.is_some() {
            let target_community = list.get_community().expect("E.");
            if community_id > 0 {
                let community = get_community(community_id).expect("E.");
                let _res = block(move || list.add_in_community_collections(community.id)).await?;
                Ok(Json(_res))
            }
            else if data.community_id.is_some() {
                let community = get_community(data.community_id.unwrap()).expect("E.");
                if user_id > 0 && target_community.get_administrators_ids().iter().any(|&i| i==user_id) {
                    let _res = block(move || list.add_in_community_collections(community.id)).await?;
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
        else {
            let owner = list.get_creator().expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else if user_id == 0 || data.community_id.is_none() {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || list.add_in_community_collections(data.community_id.unwrap())).await?;
                Ok(Json(_res))
            }
        }
    }
}
pub async fn delete_list_from_community_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
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
    else if community_id < 1 || data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'community_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if community_id > 0 {
            let _res = block(move || list.remove_in_community_collections(community_id)).await?;
            Ok(Json(_res))
        }
        else if data.community_id.is_some() {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            if user_id > 0 && community.get_administrators_ids().iter().any(|&i| i==user_id) {
                let _res = block(move || list.remove_in_community_collections(community.id)).await?;
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

pub async fn add_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id > 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let _res = block(move || PhotoList::create_list(data)).await?;
        Ok(Json(_res))
    }
}
pub async fn edit_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id > 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if list.user_id != user_id || list.community_id.is_some() {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
        else {
            let _res = block(move || PhotoList::edit_list(data)).await?;
            Ok(Json(_res))
        }
    }
}
pub async fn add_community_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let (err, user_id, community_id) = get_owner_data(data.token.clone(), data.user_id, 21);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id < 1 && community_id < 1 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if community_id < 1 && data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if community_id > 0 {
        let _res = block(move || PhotoList::create_list(data)).await?;
        Ok(Json(_res))
    }
    else if data.community_id.is_some() {
        let community = get_community(data.community_id.unwrap()).expect("E.");
        if user_id > 0 && community.is_user_create_list(user_id) {
            let _res = block(move || PhotoList::create_list(data)).await?;
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
pub async fn edit_community_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
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
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if community_id > 1 && data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if community_id > 0 {
        let _res = block(move || PhotoList::edit_list(data)).await?;
        Ok(Json(_res))
    }
    else if data.community_id.is_some() {
        let community = get_community(data.community_id.unwrap()).expect("E.");
        if user_id > 0 && community.get_administrators_ids().iter().any(|&i| i==user_id) {
            let _res = block(move || PhotoList::edit_list(data)).await?;
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

pub async fn delete_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
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
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            if  (community_id > 0 && list.community_id.unwrap() == community_id)
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {

                let _res = block(move || list.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id < 1 && list.user_id == user_id {
                let _res = block(move || list.delete_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn recover_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
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
        let list = get_photo_list(data.id.unwrap()).expect("E.");
        if list.community_id.is_some() {
            let community = list.get_community().expect("E.");
            if  (community_id > 0 && list.community_id.unwrap() == community_id)
                ||
                (user_id > 0 && community.get_editors_ids().iter().any(|&i| i==user_id)) {

                let _res = block(move || list.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
        else {
            if community_id < 1 && (list.user_id == user_id || list.user_id == user_id) {
                let _res = block(move || list.restore_item()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
}

pub async fn copy_list(data: Json<DataCopyList>) -> Result<Json<i16>, Error> {
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
        let list = get_photo_list(data.item_id.unwrap()).expect("E.");
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
                let _res = block(move || list.copy_list(user_id, data.owners.clone())).await?;
                Ok(Json(_res))
            }
        }
        else {
            let owner = get_user(list.user_id).expect("E.");
            let _tuple = get_user_permission(&owner, user_id);
            if _tuple.0 == false {
                Err(Error::BadRequest(_tuple.1))
            }
            else if !list.is_user_copy_el(user_id) || !owner.is_user_copy_el(user_id) {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
            else {
                let _res = block(move || list.copy_list(user_id, data.owners.clone())).await?;
                Ok(Json(_res))
            }
        }
    }
}
