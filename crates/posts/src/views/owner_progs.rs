use actix_web::{
    web,
    web::block,
    web::Json,
};

use crate::models::{
    User, Community, Owner,
    NewUserJson, NewCommunityJson,
    PostList, Post, PostComment,
};
use crate::errors::Error;
use crate::utils::{
    get_owner,
    get_user, get_community,
    AttachPostCommentResp,
    AttachPostResp, AttachPostListResp,
    ErrorParams, ObjectData, ItemParams,
};
use serde::Deserialize;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    config.route("/create_community/", web::post().to(create_community));
    config.route("/delete_user/", web::post().to(delete_user));
    config.route("/delete_community/", web::post().to(delete_community));

    config.route("/get_attach_post_lists/", web::get().to(get_attach_post_lists));
    config.route("/get_attach_posts/", web::get().to(get_attach_posts));
    config.route("/get_attach_post_comments/", web::get().to(get_attach_post_comments));

    config.route("/create_token", web::post().to(create_token));
    config.route("/edit_token", web::post().to(edit_token));
    config.route("/delete_token", web::post().to(delete_token));
} 

/* 
токен апи-шлюза. Когда надо произвести доп изменения в сервисах, 
причастных к какому-либо изменению в базах данных. Например, создание токенов
приложений, к которым хочет аппелировать owner.
Или изменение названия сообщества, которое потянет такие изменения на всех
сервисах, в которых участвует сообщество. Такие зависимости пользователей и сообществ
пропишутся в сервисе апи шлюза для более удобного взаимодействия П. и С. с сервисами.
*/
static TOKEN: &str = "111";

// manager send!
// создаем пользователя сервиса, создателя списков, постов, комментов
pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.first_name.is_none() {
        Err(Error::BadRequest("Field 'first_name' is required!".to_string()))
    }
    else if data.is_man.is_none() {
        Err(Error::BadRequest("Field 'is_man' is required!".to_string()))
    }
    else if data.last_name.is_none() {
        Err(Error::BadRequest("Field 'last_name' is required!".to_string()))
    }
    else if data.types.is_none() {
        Err(Error::BadRequest("Field 'types' is required!".to_string()))
    }
    else if data.link.is_none() {
        Err(Error::BadRequest("Field 'link' is required!".to_string()))
    }
    else if data.see_all.is_none() {
        Err(Error::BadRequest("Field 'see_all' is required!".to_string()))
    }
    else {
        let is_man: bool;
        if data.is_man.unwrap() != 1 {
            is_man = false;
        }
        else {
            is_man = true;
        }
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || User::create_user(
                data.user_id.unwrap(),
                data.first_name.as_deref().unwrap().to_string(),
                data.last_name.as_deref().unwrap().to_string(),
                data.types.unwrap(),
                is_man,
                data.link.as_deref().unwrap().to_string(),
                data.s_avatar.clone(),
                data.see_all.unwrap(),
                data.friends.clone(),
                data.friends.clone(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
// создаем сообщество сервиса, создателя списков, постов, комментов
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'community_id' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.types.is_none() {
        Err(Error::BadRequest("Field 'types' is required!".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.link.is_none() {
        Err(Error::BadRequest("Field 'link' is required!".to_string()))
    } 
    else { 
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || Community::create_community (
                data.community_id.unwrap(),
                data.user_id.unwrap(),
                data.name.as_deref().unwrap().to_string(),
                data.types.unwrap(),
                data.link.as_deref().unwrap().to_string(),
                data.s_avatar.clone(),
                data.follows.clone(), 
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_user(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.id.unwrap()).expect("E.");
            let _res = block(move || user.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_community(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.id.unwrap()).expect("E.");
            let _res = block(move || community.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}


#[derive(Deserialize)]
pub struct VecIdsParams {
    token:   Option<String>,
    pub ids: Option<Vec<i32>>,
}

// manager send!
// выдаем данные для закрепления списков записей в других сервисах
pub async fn get_attach_post_lists(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostListResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || PostList::get_lists_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
    
}

// manager send!
// выдаем данные для закрепления записей в других сервисах
pub async fn get_attach_posts(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || Post::get_posts_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
// выдаем данные для закрепления комментов в других сервисах
pub async fn get_attach_post_comments(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostCommentResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || PostComment::get_comments_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct AddTokenData {
    token:        Option<String>,
    id:           Option<i32>,
    user_id:      Option<i32>,
    community_id: Option<i32>,
    name:         Option<String>,
    secret_key:   Option<String>,
    service_key:  Option<String>,
    types:        Option<i16>,
    services_ids: Option<Vec<i32>>,
}

// manager send!
pub async fn create_token(data: Json<AddTokenData>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
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
        if data.token.as_deref().unwrap() == TOKEN {
                let _res = block(move || Owner::create (
                data.user_id.unwrap(),
                data.community_id,
                data.name.as_deref().unwrap().to_string(),
                data.secret_key.as_deref().unwrap().to_string(),
                data.service_key.as_deref().unwrap().to_string(),
                data.types.unwrap(),
                data.services_ids.as_deref().unwrap().to_vec(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        } 
    }
}

// manager send!
pub async fn edit_token(data: Json<AddTokenData>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
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
        if data.token.as_deref().unwrap() == TOKEN && owner.user_id == data.user_id.unwrap() {
                let _res = block(move || owner.edit (
                    data.name.as_deref().unwrap().to_string(),
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
    if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
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
        if data.token.as_deref().unwrap() == TOKEN && owner.user_id == data.user_id.unwrap() {
            let _res = block(move || owner.delete ()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}