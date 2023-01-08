use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use serde::{Deserialize, Serialize};
use crate::models::{
    TokenDetailJson, TokenJson, Community, Owner, OwnerService,
    NewUserJson, User, FollowsList, FriendsList,
};
use crate::utils::{ 
    get_user_owner_data, 
    get_community, 
    get_friends_list,
    get_follows_list,
    get_owner, 
    get_user, 
    ErrorParams, ObjectData, SmallData,
    EditTokenPageResp, ItemParams, 
}; 
use crate::errors::Error;
use crate::AppState;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_token", web::get().to(create_token_page));
    config.route("/edit_token", web::get().to(edit_token_page));
    config.route("/get_token", web::get().to(get_token));
    config.route("/get_tokens", web::get().to(get_tokens));
    config.route("/get_secret_key", web::get().to(get_secret_key));
    config.route("/get_service_key", web::get().to(get_service_key));

    config.route("/create_token", web::post().to(create_token));
    config.route("/edit_token", web::post().to(edit_token));
    config.route("/delete_token", web::post().to(delete_token));

    config.route("/create_user", web::post().to(create_user));
    config.route("/delete_user", web::post().to(delete_user));
    config.route("/restore_user", web::post().to(restore_user));
    config.route("/edit_user_name", web::post().to(edit_user_name));
    config.route("/update_last_activity", web::post().to(update_last_activity));
    config.route("/edit_user_link", web::post().to(edit_user_link));
    config.route("/edit_user_avatar", web::post().to(edit_user_avatar));
    config.route("/edit_user_password", web::post().to(edit_user_password)); 

    config.route("/create_friend", web::post().to(create_friend));
    config.route("/create_follow", web::post().to(create_follow));
    config.route("/delete_friend", web::post().to(delete_friend));
    config.route("/delete_follow", web::post().to(delete_follow));
    config.route("/create_friends_list", web::post().to(create_friends_list));
    config.route("/create_follows_list", web::post().to(create_follows_list));
    config.route("/delete_friends_list", web::post().to(delete_friends_list));
    config.route("/delete_follows_list", web::post().to(delete_follows_list));
    config.route("/restore_friends_list", web::post().to(restore_friends_list));
    config.route("/restore_follows_list", web::post().to(restore_follows_list));

    config.route("/create_block_user", web::post().to(create_block_user));
    config.route("/delete_block_user", web::post().to(delete_block_user));
}

 /*
    Обычные токены - полноценные токены сообщества. Создаются пользователем напрямую.
    Токены приложения - для приложений, которые работают как наше пользовательское,
    работающее не для себя, а для других в том числе.
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
    else if data.password.is_none() {
        Err(Error::BadRequest("Field 'password' is required!".to_string()))
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
    else if data.friends_list.is_none() {
        Err(Error::BadRequest("Field 'friends_list' is required!".to_string()))
    }
    else if data.follows_list.is_none() {
        Err(Error::BadRequest("Field 'follows_list' is required!".to_string()))
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
            let _res = block(move || User::create_user (
                data.user_id.unwrap(),
                data.first_name.as_deref().unwrap().to_string(),
                data.last_name.as_deref().unwrap().to_string(),
                data.types.unwrap(),
                is_man,
                data.password.as_deref().unwrap().to_string(),
                data.link.as_deref().unwrap().to_string(),
                data.friends_list.unwrap(),
                data.follows_list.unwrap(),
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
pub async fn restore_user(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.id.unwrap()).expect("E.");
            let _res = block(move || user.restore_item()).await?;
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
            let _res = block(move || user.edit_link (
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

#[derive(Deserialize)]
pub struct AddTargetParams {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
    pub list_id:   Option<i32>,
}

// manager send!
pub async fn create_friend(data: Json<AddTargetParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.list_id.is_none() {
        Err(Error::BadRequest("Field 'list_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.frend_user (
                data.list_id.unwrap(),
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
    else if data.list_id.is_none() {
        Err(Error::BadRequest("Field 'list_id' is required!".to_string()))
    }
    else if data.target_id.is_none() {
        Err(Error::BadRequest("Field 'target_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block(move || user.follow_user (
                data.list_id.unwrap(),
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
            let _res = block(move || user.block_user (
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
            let _res = block(move || user.unfrend_user (
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
            let _res = block(move || user.unfollow_user (
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
            let _res = block(move || user.unblock_user (
                data.target_id.unwrap(),
            )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct AddListParams {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub list_id: Option<i32>,
}
// manager send!
pub async fn create_friends_list(data: Json<AddListParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.list_id.is_none() {
        Err(Error::BadRequest("Field 'list_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || FriendsList::create_list (
                data.list_id.unwrap(),
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
pub async fn create_follows_list(data: Json<AddListParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.user_id.is_none() {
        Err(Error::BadRequest("Field 'user_id' is required!".to_string()))
    }
    else if data.list_id.is_none() {
        Err(Error::BadRequest("Field 'list_id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || FollowsList::create_list (
                data.list_id.unwrap(),
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
pub async fn delete_follows_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            use crate::utils::get_follows_list;

            let list = get_follows_list(data.id.unwrap()).expect("E.");
            let _res = block(move || list.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
pub async fn restore_follows_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            use crate::utils::get_follows_list;

            let list = get_follows_list(data.id.unwrap()).expect("E.");
            let _res = block(move || list.restore_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
pub async fn delete_friends_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            use crate::utils::get_friends_list;

            let list = get_friends_list(data.id.unwrap()).expect("E.");
            let _res = block(move || list.delete_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}
// manager send!
pub async fn restore_friends_list(data: Json<ItemParams>) -> Result<Json<i16>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.id.is_none() {
        Err(Error::BadRequest("Field 'id' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            use crate::utils::get_friends_list;

            let list = get_friends_list(data.id.unwrap()).expect("E.");
            let _res = block(move || list.restore_item()).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct TokenData {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}
#[derive(Deserialize)]
pub struct TokensData {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
}

pub async fn create_token_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<Vec<OwnerService>>, Error> {
    let params_some = web::Query::<SmallData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() { 
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let body = block(move || OwnerService::get_all()).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn edit_token_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<EditTokenPageResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Owner;
            let owner_res = get_owner(params.id.unwrap());
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
                    let _res = block(move || owner.get_edit_data()).await?;
                Ok(Json(_res))
            }
            else {
                Err(Error::BadRequest("Permission Denied".to_string()))
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn get_token (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<TokenDetailJson>, Error> {
    let params_some = web::Query::<TokenData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'community_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let owner_res = get_community(params.community_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_token_detail(params.id.unwrap())).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn get_tokens (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<Vec<TokenJson>>, Error> {
    let params_some = web::Query::<TokensData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'community_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let owner_res = get_community(params.community_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_tokens()).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

#[derive(Deserialize)]
pub struct AddTokenData {
    token:        Option<String>,
    community_id: Option<i32>,
    name:         Option<String>,
    description:  Option<String>,
    services_ids: Option<Vec<i32>>,
}

pub async fn create_token (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<AddTokenData>
) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'community_id' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let _res = block(move || Owner::create (
            user_id,
            data.community_id,
            data.name.as_deref().unwrap().to_string(),
            data.description.clone(),
            data.services_ids.as_deref().unwrap().to_vec(),
        )).await?;
        Ok(Json(_res))
    }
}

#[derive(Deserialize)]
pub struct EditTokenData {
    token:        Option<String>,
    id:           Option<i32>,
    name:         Option<String>,
    description:  Option<String>,
    services_ids: Option<Vec<i32>>,
}
pub async fn edit_token (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditTokenData>
) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
                    data.services_ids.as_deref().unwrap().to_vec()
                )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn delete_token (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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

pub async fn get_secret_key (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
) -> Result<Json<String>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
            Ok(Json(owner.secret_key))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}
pub async fn get_service_key (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
) -> Result<Json<String>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
            Ok(Json(owner.service_key))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}