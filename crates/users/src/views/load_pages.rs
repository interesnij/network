use actix_web::{
    HttpRequest,
    //Responder,
    //HttpResponse,
    web,
    web::block,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_user, get_user_owner_data,
    ErrorParams, CardUserJson, RegListData,
};
use crate::errors::Error;


pub fn load_urls(config: &mut web::ServiceConfig) {
    config.route("/load/friends/", web::get().to(friends_load));
    config.route("/load/follows/", web::get().to(follows_load));
    config.route("/load/include_friends/", web::get().to(include_friends_load));
    config.route("/load/exclude_friends/", web::get().to(exclude_friends_load));
    config.route("/load/include_follows/", web::get().to(include_follows_load));
    config.route("/load/exclude_follows/", web::get().to(exclude_follows_load));
}


pub async fn friends_load(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 1);
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
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");
                _user.get_friends(params.limit, params.offset)
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn follows_load(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 1);
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
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");
                _user.get_followers(params.limit, params.offset)
            }).await?;
            Ok(Json(_res))
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
pub struct IEFriendsData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub user_id:        Option<i32>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub friends_limit:  Option<i64>,
    pub friends_offset: Option<i64>,
}
#[derive(Deserialize)]
pub struct IEFollowsData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub user_id:        Option<i32>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub follows_limit:  Option<i64>,
    pub follows_offset: Option<i64>,
}

#[derive(Serialize)]
pub struct IEFriendsResponse {
    pub users:   Vec<CardUserJson>,
    pub friends: Vec<CardUserJson>,
}
#[derive(Serialize)]
pub struct IEFollowsResponse {
    pub users:   Vec<CardUserJson>,
    pub follows: Vec<CardUserJson>,
}

pub async fn include_friends_load(req: HttpRequest) -> Result<Json<IEFriendsResponse>, Error> {
    let params_some = web::Query::<IEFriendsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_all_include_friends(params.users_limit, params.users_offset),
                    2 => _user.get_limit_see_info_include_friends(params.users_limit, params.users_offset),
                    3 => _user.get_limit_see_friend_include_friends(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(params.friends_limit, params.friends_offset),
                }
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn exclude_friends_load(req: HttpRequest) -> Result<Json<IEFriendsResponse>, Error> {
    let params_some = web::Query::<IEFriendsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_all_exclude_friends(params.users_limit, params.users_offset),
                    12 => _user.get_limit_see_info_exclude_friends(params.users_limit, params.users_offset),
                    13 => _user.get_limit_see_friend_exclude_friends(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(params.friends_limit, params.friends_offset),
                }
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}


pub async fn include_follows_load(req: HttpRequest) -> Result<Json<IEFollowsResponse>, Error> {
    let params_some = web::Query::<IEFollowsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_all_include_follows(params.users_limit, params.users_offset),
                    2 => _user.get_limit_see_info_include_follows(params.users_limit, params.users_offset),
                    3 => _user.get_limit_see_friend_include_follows(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFollowsResponse {
                    users:   _users,
                    follows: _user.get_followers(params.follows_limit, params.follows_offset),
                }
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn exclude_follows_load(req: HttpRequest) -> Result<Json<IEFollowsResponse>, Error> {
    let params_some = web::Query::<IEFollowsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_all_exclude_follows(params.users_limit, params.users_offset),
                    12 => _user.get_limit_see_info_exclude_follows(params.users_limit, params.users_offset),
                    13 => _user.get_limit_see_friend_exclude_follows(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFollowsResponse {
                    users:   _users,
                    follows: _user.get_followers(params.follows_limit, params.follows_offset),
                }
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
