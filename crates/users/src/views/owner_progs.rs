use actix_web::{
    web,
    web::block,
    web::Json,
};
use serde::Serialize;
use crate::models::{
    User, Community,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;
use crate::utils::{
    AttachmentsJson,
    TokenDetailJson, TokenJson,
    InfoParams, ErrorParams,
};


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/get_token/", web::get().to(get_token));        // инфо выбранного токена пользователя
    config.route("/get_tokens/", web::get().to(get_tokens));      // инфо выбранных токенов пользователя
    //config.route("/create_token/", web::post().to(create_token)); // создание токена пользователя
    //config.route("/edit_token/", web::post().to(edit_token));     // изменение токена пользователя
    //config.route("/delete_token/", web::post().to(delete_token)); // удаление токена пользователя

    //config.route("/get_app_token/", web::get().to(get_token));        // инфо выбранного токена приложения
    //config.route("/get_app_tokens/", web::get().to(get_tokens));      // инфо выбранных токенов приложения
    //config.route("/create_app_token/", web::post().to(create_app_token)); // создание токена приложения
    //config.route("/edit_app_token/", web::post().to(edit_token));         // изменение токена приложения
    //config.route("/delete_app_token/", web::post().to(delete_app_token)); // удаление токена приложения
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

pub async fn get_token(data: Json<TokenData>) -> Result<Json<TokenDetailJson>, Error> {
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
            let body = block(move || owner.get_token(params.id.unwrap())).await?;
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

pub async fn get_tokens(data: Json<TokensData>) -> Result<Json<Vec<TokenJson>>, Error> {
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
