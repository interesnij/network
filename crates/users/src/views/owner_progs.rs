use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use serde::{Serialize, Deserialize};
use crate::models::{
    User, Owner,
};
use crate::utils::{
    get_user_owner_data, get_user, get_owner,
    InfoParams, ErrorParams, ObjectData,
};
use crate::models::{TokenDetailJson, TokenJson};
use crate::errors::Error;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/get_token/", web::get().to(get_token));
    config.route("/get_tokens/", web::get().to(get_tokens));
    config.route("/create_token/", web::post().to(create_token));
    config.route("/edit_token/", web::post().to(edit_token));
    config.route("/delete_token/", web::post().to(delete_token));

    //config.route("/get_app_token/", web::get().to(get_token));
    //config.route("/get_app_tokens/", web::get().to(get_tokens));
    //config.route("/create_app_token/", web::post().to(create_app_token));
    //config.route("/edit_app_token/", web::post().to(edit_token));
    //config.route("/delete_app_token/", web::post().to(delete_app_token));
}

 /*
    Обычные токены - полноценные пользовательские. Создаются пользователем напрямую.
    Токены приложения - для приложений, которые работают как наше пользовательское,
    работающее не для себя, а для других в том числе.
 */

#[derive(Deserialize)]
pub struct TokenData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub id:      Option<i32>,
}
#[derive(Deserialize)]
pub struct TokensData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
}

pub async fn get_token(req: HttpRequest) -> Result<Json<TokenDetailJson>, Error> {
    let params_some = web::Query::<TokenData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 31);
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

pub async fn get_tokens(req: HttpRequest) -> Result<Json<Vec<TokenJson>>, Error> {
    let params_some = web::Query::<TokensData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 31);
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

#[derive(Deserialize)]
pub struct AddTokenData {
    token:        Option<String>,
    user_id:      Option<i32>,
    name:         Option<String>,
    description:  Option<String>,
    types:        Option<i16>,
    services_ids: Vec<i32>
}

pub async fn create_token(data: Json<AddTokenData>) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
    else if data.types.is_none() {
        Err(Error::BadRequest("Field 'types' is required!".to_string()))
    }
    else if data.services_ids.is_none() {
        Err(Error::BadRequest("Field 'services_ids' is required!".to_string()))
    }
    else {
        let _res = block(move || Owner::create (
            user_id,
            name,
            description,
            types,
            services_ids,
        )).await?;
        Ok(Json(_res))
    }
}

#[derive(Deserialize)]
pub struct EditTokenData {
    token:        Option<String>,
    id:           Option<i32>,
    user_id:      Option<i32>,
    name:         Option<String>,
    description:  Option<String>,
    types:        Option<i16>,
    services_ids: Vec<i32>
}
pub async fn edit_token(data: Json<EditTokenData>) -> Result<Json<TokenDetailJson>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
    else if data.types.is_none() {
        Err(Error::BadRequest("Field 'types' is required!".to_string()))
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
                    name,
                    description,
                    services_ids
                )).await?;
            Ok(Json(_res))
        }
        else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

pub async fn delete_token(data: Json<ObjectData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
