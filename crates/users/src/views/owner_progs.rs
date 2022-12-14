use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use serde::Deserialize;
use crate::models::{
    User, Owner, OwnerService
};
use crate::utils::{
    get_user_owner_data, get_user, get_owner,
    ErrorParams, ObjectData, SmallData,
    EditTokenPageResp, AttachUserResp, TOKEN,
}; 
use crate::models::{TokenDetailJson, TokenJson, };
use crate::errors::Error;
use crate::AppState;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_token", web::get().to(create_token_page));
    config.route("/edit_token", web::get().to(edit_token_page));
    config.route("/get_token", web::get().to(get_token));
    config.route("/get_tokens", web::get().to(get_tokens));

    config.route("/create_user_token", web::post().to(create_user_token));
    config.route("/create_app_token", web::post().to(create_app_token));
    config.route("/edit_token", web::post().to(edit_token));
    config.route("/delete_token", web::post().to(delete_token));

    config.route("/get_app_token", web::get().to(get_app_token));
    config.route("/get_app_tokens", web::get().to(get_app_tokens));
    config.route("/get_all_tokens", web::get().to(get_all_tokens));

    config.route("/get_secret_key", web::get().to(get_secret_key));
    config.route("/get_service_key", web::get().to(get_service_key));

    config.route("/get_attach_users", web::get().to(get_attach_users));
}

 /*
    Обычные токены - полноценные пользовательские. Создаются пользователем напрямую.
    Токены приложения - для приложений, которые работают как наше пользовательское,
    работающее не для себя, а для других в том числе.

    Мы будем генерировать токены и отправлять обратно в api, а оттуда
    копировать во все сервисы, которые будут указаны при создании, 
    вместе с кодами, чтобы все они совпадали.
 */

#[derive(Deserialize)]
pub struct TokenData {
    pub token:   Option<String>,
    pub id:      Option<i32>,
}
#[derive(Deserialize)]
pub struct TokensData {
    pub token:   Option<String>,
}

pub async fn create_token_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<OwnerService>>, Error> {
    let params_some = web::Query::<SmallData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() { 
            // если проверка токена не удалась...
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
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditTokenPageResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<TokenDetailJson>, Error> {
    let params_some = web::Query::<TokenData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                // если список по id не найден...
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
pub async fn get_app_token (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<TokenDetailJson>, Error> {
    let params_some = web::Query::<TokenData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
        else if user_id == 0 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            } 
            let body = block(move || owner.get_app_token_detail(params.id.unwrap())).await?;
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
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<TokenJson>>, Error> {
    let params_some = web::Query::<TokensData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                // если список по id не найден...
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

pub async fn get_app_tokens (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<TokenJson>>, Error> {
    let params_some = web::Query::<TokensData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_app_tokens()).await?;
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

pub async fn get_all_tokens (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<TokenJson>>, Error> {
    let params_some = web::Query::<TokensData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            // если проверка токена не удалась...
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_all_tokens()).await?;
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
    name:         Option<String>,
    description:  Option<String>,
    services_ids: Option<Vec<i32>>, 
}

pub async fn create_user_token (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<AddTokenData>
) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let _res = block(move || Owner::create_user_token (
            user_id,
            data.name.as_deref().unwrap().to_string(),
            data.description.clone(),
            data.services_ids.as_deref().unwrap().to_vec(),
        )).await?;
        Ok(Json(_res))
    }
}
pub async fn create_app_token (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<AddTokenData>
) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if user_id == 0 {
        Err(Error::BadRequest("Permission Denied".to_string()))
    }
    else if data.name.is_none() {
        Err(Error::BadRequest("Field 'name' is required!".to_string()))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let _res = block(move || Owner::create_app_token (
            user_id,
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
        // если проверка токена не удалась или запрос анонимный...
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
            // если список по id не найден...
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
        // если проверка токена не удалась или запрос анонимный...
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
            // если список по id не найден...
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
        // если проверка токена не удалась или запрос анонимный...
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
            // если список по id не найден...
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
        // если проверка токена не удалась или запрос анонимный...
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
            // если список по id не найден...
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

#[derive(Deserialize)]
pub struct VecIdsParams {
    token:   Option<String>,
    pub ids: Option<Vec<i32>>,
}

// manager send!
// выдаем данные для использования объектов пользователей в других сервисах
pub async fn get_attach_users(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachUserResp>>, Error> {
    if data.token.is_none() {
        Err(Error::BadRequest("Field 'token' is required!".to_string()))
    }
    else if data.ids.is_none() {
        Err(Error::BadRequest("Field 'ids' is required!".to_string()))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN {
            let _res = block(move || User::get_users_for_attach(data.ids.as_deref().unwrap().to_vec())).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied!".to_string()))
        }
    }
    
}