use actix_web::{
    web,
    web::block,
    web::Json,
};
use serde::Serialize;
use crate::models::{
    User, Community,
    NewUserJson, NewCommunityJson,
    PostList, Post, PostComment,
    TokenDetailJson, TokenJson,
};
use crate::errors::Error;
use crate::utils::{
    AttachmentsJson, AttachPostCommentResp,
    AttachPostResp, AttachPostListResp,
    EditTokenPageResp, ErrorParams, ObjectData, SmallData,
};
use serde::Deserialize;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_service_user/", web::post().to(create_service_user));
    config.route("/create_service_community/", web::post().to(create_service_community));

    config.route("/get_attach_post_lists/", web::get().to(get_attach_post_lists));
    config.route("/get_attach_posts/", web::get().to(get_attach_posts));
    config.route("/get_attach_post_comments/", web::get().to(get_attach_post_comments));

    config.route("/create_token", web::post().to(create_token));
    config.route("/edit_token", web::post().to(edit_token));
    config.route("/delete_token", web::post().to(delete_token));
} 

// создаем пользователя сервиса, создателя списков, постов, комментов
pub async fn create_service_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
// создаем сообщество сервиса, создателя списков, постов, комментов
pub async fn create_service_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}

#[derive(Deserialize)]
pub struct VecIdsParams {
    pub ids: Vec<i32>,
}

// manager send!
// выдаем данные для закрепления списков записей в других сервисах
pub async fn get_attach_post_lists(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostListResp>>, Error> {
    let _res = block(move || PostList::get_lists_for_attach(data.ids.clone())).await?;
    Ok(Json(_res))
}

// manager send!
// выдаем данные для закрепления записей в других сервисах
pub async fn get_attach_posts(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostResp>>, Error> {
    let _res = block(move || Post::get_posts_for_attach(data.ids.clone())).await?;
    Ok(Json(_res))
}

// manager send!
// выдаем данные для закрепления комментов в других сервисах
pub async fn get_attach_post_comments(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostCommentResp>>, Error> {
    let _res = block(move || PostComment::get_comments_for_attach(data.ids.clone())).await?;
    Ok(Json(_res)) 
}

#[derive(Deserialize)]
pub struct TokenData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub id:      Option<i32>,
}

#[derive(Deserialize)]
pub struct AddTokenData {
    token:        Option<String>,
    user_id:      Option<i32>,
    name:         Option<String>,
    description:  Option<String>,
    secret_key:   Option<String>,
    service_key:  Option<String>,
    types:        Option<i16>,
    services_ids: Option<Vec<i32>>,
}

// manager send!
pub async fn create_token(data: Json<AddTokenData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.types.is_none() {
        Err(Error::BadRequest("Field 'types' is required!".to_string()))
    }
    else if data.secret_key.is_none() {
        Err(Error::BadRequest("Field 'secret_key' is required!".to_string()))
    }
    else if data.service_key.is_none() {
        Err(Error::BadRequest("Field 'service_key' is required!".to_string()))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let _res = block(move || Owner::create (
            user_id,
            data.name.as_deref().unwrap().to_string(),
            data.description.clone(),
            data.types.unwrap(),
            data.secret_key.as_deref().unwrap().to_string(),
            data.service_key.as_deref().unwrap().to_string(),
            data.services_ids.as_deref().unwrap().to_vec(),
        )).await?;
        Ok(Json(_res))
    }
}

// manager send!
pub async fn edit_token(data: Json<AddTokenData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.secret_key.is_none() {
        Err(Error::BadRequest("Field 'secret_key' is required!".to_string()))
    }
    else if data.service_key.is_none() {
        Err(Error::BadRequest("Field 'service_key' is required!".to_string()))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let owner: Owner;
        let owner_res = get_owner(data.id.unwrap());
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if owner.user_id == user_id {
                let _res = block(move || owner.edit (
                    data.name.as_deref().unwrap().to_string(),
                    data.description.clone(),
                    data.secret_key.as_deref().unwrap().to_string(),
                    data.service_key.as_deref().unwrap().to_string(),
                    data.services_ids.as_deref().unwrap().to_vec(),
                )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

// manager send!
pub async fn delete_token(data: Json<ObjectData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        let owner: Owner;
        let owner_res = get_owner(data.id.unwrap());
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if owner.user_id == user_id {
            let _res = block(move || owner.delete ()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}