use actix_web::{
    web,
    web::block,
    web::Json,
};

use crate::models::{
    User, Community, Owner,
    NewUserJson, NewCommunityJson,
    PhotoList, Photo, PhotoComment,
};
use crate::errors::Error;
use crate::utils::{
    get_owner,
    get_user, get_community,
    AttachPhotoCommentResp,
    AttachPhotoResp, AttachPhotoListResp,
    ErrorParams, ObjectData, ItemParams,
};
use serde::{Serialize, Deserialize};


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user", web::post().to(create_user));
    config.route("/delete_user", web::post().to(delete_user));
    config.route("/edit_user_name", web::post().to(edit_user_name));
    config.route("/update_last_activity", web::post().to(update_last_activity));
    config.route("/edit_user_link", web::post().to(edit_user_link));
    config.route("/edit_user_avatar", web::post().to(edit_user_avatar));
    config.route("/edit_user_password", web::post().to(edit_user_password)); 
    config.route("/create_friend", web::post().to(create_friend));
    config.route("/create_follow", web::post().to(create_follow));
    config.route("/create_block_user", web::post().to(create_block_user));
    config.route("/delete_friend", web::post().to(delete_friend));
    config.route("/delete_follow", web::post().to(delete_follow));
    config.route("/delete_block_user", web::post().to(delete_block_user));
    
    config.route("/create_community", web::post().to(create_community));
    config.route("/delete_community", web::post().to(delete_community));
    config.route("/edit_community_name", web::post().to(edit_community_name));
    config.route("/edit_community_link", web::post().to(edit_community_link));
    config.route("/edit_community_avatar", web::post().to(edit_community_avatar));
    config.route("/create_member", web::post().to(create_member));
    config.route("/create_ban_user", web::post().to(create_ban_user));
    config.route("/delete_member", web::post().to(delete_member));
    config.route("/delete_ban_user", web::post().to(delete_ban_user));

    config.route("/get_attach_photo_lists", web::get().to(get_attach_photo_lists));
    config.route("/get_attach_photos", web::get().to(get_attach_photos));
    config.route("/get_attach_photo_comments", web::get().to(get_attach_photo_comments));

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

#[derive(Deserialize)]
pub struct AddTargetParams {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
}

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
    else if data.password.is_none() {
        Err(Error::BadRequest("Field 'password' is required!".to_string()))
    }
    else if data.last_name.is_none() {
        Err(Error::BadRequest("Field 'last_name' is required!".to_string()))
    }
    else if data.link.is_none() {
        Err(Error::BadRequest("Field 'link' is required!".to_string()))
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
                is_man,
                data.password.as_deref().unwrap().to_string(),
                data.link.as_deref().unwrap().to_string(),
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

#[derive(Deserialize)]
pub struct UserNameParams {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}
// manager send!
pub async fn edit_user_name(data: Json<UserNameParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.first_name.is_none() {
        Err(Error::BadRequest("Field 'first_name' is required!".to_string()))
    }
    else if data.last_name.is_none() {
        Err(Error::BadRequest("Field 'last_name' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.edit_name(
                data.first_name.as_deref().unwrap(),
                data.last_name.as_deref().unwrap()
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn update_last_activity(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.id.unwrap()).expect("E.");
            let _res = block(move || user.update_last_activity()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct UserLinkParams {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub link:    Option<String>,
}
// manager send!
pub async fn edit_user_link(data: Json<UserLinkParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.link.is_none() {
        Err(Error::BadRequest("Field 'link' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.edit_link(
                data.link.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct UserAvatarParams {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub avatar:  Option<String>,
}
// manager send!
pub async fn edit_user_avatar(data: Json<UserAvatarParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.avatar.is_none() {
        Err(Error::BadRequest("Field 'avatar' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.edit_link(
                data.avatar.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserPasswordParams {
    pub token:    Option<String>,
    pub user_id:  Option<i32>,
    pub password: Option<String>,
}
// manager send!
pub async fn edit_user_password(data: Json<UserPasswordParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.password.is_none() {
        Err(Error::BadRequest("Field 'password' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.edit_password (
                data.password.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}


// manager send!
pub async fn create_friend(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.frend_user(
                data.target_id.unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn create_follow(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.follow_user(
                data.target_id.unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn create_block_user(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.block_user(
                data.target_id.unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_friend(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.unfrend_user(
                data.target_id.unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_follow(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.unfollow_user(
                data.target_id.unwrap(), 
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_block_user(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.unblock_user(
                data.target_id.unwrap(),
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
            )).await?;
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
pub struct CommunityNameParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub name:         Option<String>,
}
// manager send!
pub async fn edit_community_name(data: Json<CommunityNameParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'community_id' is required!".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            let _res = block(move || community.edit_name(
                data.name.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityLinkParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub link:         Option<String>,
}
// manager send!
pub async fn edit_community_link(data: Json<CommunityLinkParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'community_id' is required!".to_string()))
    }
    else if data.link.is_none() {
        Err(Error::BadRequest("Field 'link' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            let _res = block(move || community.edit_link(
                data.link.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityAvatarParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub avatar:       Option<String>,
}
// manager send!
pub async fn edit_community_avatar(data: Json<CommunityAvatarParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'community_id' is required!".to_string()))
    }
    else if data.avatar.is_none() {
        Err(Error::BadRequest("Field 'avatar' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            let _res = block(move || community.edit_link(
                data.avatar.as_deref().unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct AddCTargetParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
}

// manager send!
pub async fn create_member(data: Json<AddCTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.join_community(
                data.community_id.unwrap(),
            )).await?;
            Ok(Json(_res)) 
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
pub async fn create_ban_user(data: Json<AddCTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            let _res = block(move || community.create_banned_user(
                data.user_id.unwrap(),
            )).await?;
            Ok(Json(_res)) 
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
pub async fn delete_member(data: Json<AddCTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.leave_community(
                data.community_id.unwrap(),
            )).await?;
            Ok(Json(_res)) 
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
pub async fn delete_ban_user(data: Json<AddCTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let community = get_community(data.community_id.unwrap()).expect("E.");
            let _res = block(move || community.delete_banned_user(
                data.user_id.unwrap(),
            )).await?;
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
pub async fn get_attach_photo_lists(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPhotoListResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || PhotoList::get_lists_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
    
}

// manager send!
// выдаем данные для закрепления записей в других сервисах
pub async fn get_attach_photos(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPhotoResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || Photo::get_photos_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

// manager send!
// выдаем данные для закрепления комментов в других сервисах
pub async fn get_attach_photo_comments(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPhotoCommentResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || PhotoComment::get_comments_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
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