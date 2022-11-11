use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_community,
    get_user,
    get_post_list,
    get_community_permission,
    get_user_permission,
    get_owner_data,
    ItemParams, DataCopyList,
    DataListJson, RespListJson,
};
use crate::models::PostList;
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
    config.route("/delete_user_list/", web::post().to(delete_user_list));
    config.route("/recover_user_list/", web::post().to(recover_user_list));
    config.route("/delete_community_list/", web::post().to(delete_community_list));
    config.route("/recover_community_list/", web::post().to(recover_community_list));
    config.route("/copy_list/", web::put().to(copy_list));
}

pub async fn add_list_in_user_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    if list.community_id.is_some() {
        let community = list.get_community().expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.add_in_user_collections(data.user_id)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = list.get_creator().expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.add_in_user_collections(data.user_id)).await?;
            Ok(Json(_res))
        }
    }
}
pub async fn delete_list_from_user_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    let _res = block(move || list.remove_in_user_collections(data.user_id)).await?;
    Ok(Json(_res))
}

pub async fn add_list_in_community_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    let target_community = get_community(data.community_id.unwrap()).expect("E.");
    if !target_community.get_administrators_ids().iter().any(|&i| i==data.user_id) {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if list.community_id.is_some() {
        let community = list.get_community().expect("E.");
        let data_community_id = list.community_id.unwrap();
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.add_in_community_collections(data_community_id)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = list.get_creator().expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.add_in_community_collections(data.community_id.unwrap())).await?;
            Ok(Json(_res))
        }
    }
}
pub async fn delete_list_from_community_collection(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    let target_community = get_community(data.community_id.unwrap()).expect("E.");
    if !target_community.get_administrators_ids().iter().any(|&i| i==data.user_id) {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let _res = block(move || list.remove_in_community_collections(data.community_id.unwrap())).await?;
        Ok(Json(_res))
    }
}

pub async fn add_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let _res = block(move || PostList::create_list(data)).await?;
    Ok(Json(_res))
}
pub async fn edit_user_list(data: Json<DataListJson>) -> Result<Json<RespListJson>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    if list.user_id != data.user_id || list.community_id.is_some() {
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
        if !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest("Permission Denied".to_string()))
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
        if !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest("Permission Denied".to_string()))
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
    let list = get_post_list(data.id.unwrap()).expect("E.");
    if list.user_id != data.user_id || list.community_id.is_some() {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else {
        let _res = block(move || list.delete_item()).await?;
        Ok(Json(_res))
    }
}
pub async fn recover_user_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
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
        let list = get_post_list(data.id.unwrap()).expect("E.");
        let community = get_community(list.community_id.unwrap()).expect("E.");
        if !community.is_user_create_list(data.user_id) {
            Err(Error::BadRequest("Permission Denied".to_string()))
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
        let list = get_post_list(data.id.unwrap()).expect("E.");
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

pub async fn copy_list(data: Json<DataCopyList>) -> Result<Json<i16>, Error> {
    let list = get_post_list(data.id.unwrap()).expect("E.");
    if list.community_id.is_some() {
        let community = get_community(list.community_id.unwrap()).expect("E.");
        let _tuple = get_community_permission(&community, data.user_id);
        if _tuple.0 == false || !list.is_user_copy_el(data.user_id) {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.copy_list(data)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let owner = get_user(list.user_id).expect("E.");
        let _tuple = get_user_permission(&owner, data.user_id);
        if _tuple.0 == false {
            Err(Error::BadRequest(_tuple.1))
        }
        else {
            let _res = block(move || list.copy_list(data)).await?;
            Ok(Json(_res))
        }
    }
}
