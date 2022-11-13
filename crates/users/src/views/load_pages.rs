use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    web::block,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection, get_user,
    get_user_owner_data,
    ErrorParams, CardUserJson, RegListData,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use crate::models::{User, NewUser};
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
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _limit: i64;
                let _offset: i64;
                if params.limit.is_some() {
                    _limit = params.limit.unwrap();
                }
                else {
                    _limit = 20;
                }
                if params.offset.is_some() {
                    _offset = params.offset.unwrap();
                }
                else {
                    _offset = 0;
                }
                _user.get_friends(_limit, _offset)
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
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _limit: i64;
                let _offset: i64;
                if params.limit.is_some() {
                    _limit = params.limit.unwrap();
                }
                else {
                    _limit = 20;
                }
                if params.offset.is_some() {
                    _offset = params.offset.unwrap();
                }
                else {
                    _offset = 0;
                }
                _user.get_followers(_limit, _offset)
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
pub struct IEListData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub user_id:        Option<i32>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub friends_limit:  Option<i64>,
    pub friends_offset: Option<i64>,
}

#[derive(Serialize)]
pub struct FriendsResponse {
    pub users:   Vec<CardUserJson>,
    pub friends: Vec<CardUserJson>,
}
#[derive(Serialize)]
pub struct FollowsResponse {
    pub users:   Vec<CardUserJson>,
    pub follows: Vec<CardUserJson>,
}

pub async fn include_friends_load(req: HttpRequest) -> Result<Json<FriendsResponse>, Error> {
    let params_some = web::Query::<IEListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
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
            let _users_limit: i64;
            let _users_offset: i64;
            let _friends_limit: i64;
            let _friends_offset: i64;

            if params.users_limit.is_some() {
                _users_limit = params.users_limit.unwrap();
            }
            else {
                _users_limit = 20;
            }
            if params.users_offset.is_some() {
                _users_offset = params.users_offset.unwrap();
            }
            else {
                _users_offset = 0;
            }

            if params.friends_limit.is_some() {
                _friends_limit = params.friends_limit.unwrap();
            }
            else {
                _friends_limit = 20;
            }
            if params.friends_offset.is_some() {
                _friends_offset = params.friends_offset.unwrap();
            }
            else {
                _friends_offset = 0;
            }

            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_all_include_friends(_users_limit, _users_offset),
                    2 => _user.get_limit_see_info_include_friends(_users_limit, _users_offset),
                    3 => _user.get_limit_see_friend_include_friends(_users_limit, _users_offset),
                    _ => Vec::new(),
                };
                FriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(_friends_limit, _friends_offset),
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

pub async fn exclude_friends_load(req: HttpRequest) -> Result<Json<FriendsResponse>, Error> {
    let params_some = web::Query::<IEListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
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
            let _users_limit: i64;
            let _users_offset: i64;
            let _friends_limit: i64;
            let _friends_offset: i64;

            if params.users_limit.is_some() {
                _users_limit = params.users_limit.unwrap();
            }
            else {
                _users_limit = 20;
            }
            if params.users_offset.is_some() {
                _users_offset = params.users_offset.unwrap();
            }
            else {
                _users_offset = 0;
            }

            if params.friends_limit.is_some() {
                _friends_limit = params.friends_limit.unwrap();
            }
            else {
                _friends_limit = 20;
            }
            if params.friends_offset.is_some() {
                _friends_offset = params.friends_offset.unwrap();
            }
            else {
                _friends_offset = 0;
            }

            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_all_exclude_friends(_users_limit, _users_offset),
                    12 => _user.get_limit_see_info_exclude_friends(_users_limit, _users_offset),
                    13 => _user.get_limit_see_friend_exclude_friends(_users_limit, _users_offset),
                    _ => Vec::new(),
                };
                FriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(_friends_limit, _friends_offset),
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


pub async fn include_follows_load(req: HttpRequest) -> Result<Json<FollowsResponse>, Error> {
    let params_some = web::Query::<IEListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
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
            let _users_limit: i64;
            let _users_offset: i64;
            let _follows_limit: i64;
            let _follows_offset: i64;

            if params.users_limit.is_some() {
                _users_limit = params.users_limit.unwrap();
            }
            else {
                _users_limit = 20;
            }
            if params.users_offset.is_some() {
                _users_offset = params.users_offset.unwrap();
            }
            else {
                _users_offset = 0;
            }

            if params.follows_limit.is_some() {
                _follows_limit = params.follows_limit.unwrap();
            }
            else {
                _follows_limit = 20;
            }
            if params.follows_offset.is_some() {
                _follows_offset = params.follows_offset.unwrap();
            }
            else {
                _follows_offset = 0;
            }

            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_all_include_follows(_users_limit, _users_offset),
                    2 => _user.get_limit_see_info_include_follows(_users_limit, _users_offset),
                    3 => _user.get_limit_see_friend_include_follows(_users_limit, _users_offset),
                    _ => Vec::new(),
                };
                FollowsResponse {
                    users:   _users,
                    follows: _user.get_followers(_follows_limit, _follows_offset),
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

pub async fn exclude_follows_load(req: HttpRequest) -> Result<Json<FollowsResponse>, Error> {
    let params_some = web::Query::<IEListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
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
            let _users_limit: i64;
            let _users_offset: i64;
            let _follows_limit: i64;
            let _follows_offset: i64;

            if params.users_limit.is_some() {
                _users_limit = params.users_limit.unwrap();
            }
            else {
                _users_limit = 20;
            }
            if params.users_offset.is_some() {
                _users_offset = params.users_offset.unwrap();
            }
            else {
                _users_offset = 0;
            }

            if params.follows_limit.is_some() {
                _follows_limit = params.follows_limit.unwrap();
            }
            else {
                _follows_limit = 20;
            }
            if params.follows_offset.is_some() {
                _follows_offset = params.follows_offset.unwrap();
            }
            else {
                _follows_offset = 0;
            }

            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_all_exclude_follows(_users_limit, _users_offset),
                    12 => _user.get_limit_see_info_exclude_follows(_users_limit, _users_offset),
                    13 => _user.get_limit_see_friend_exclude_follows(_users_limit, _users_offset),
                    _ => Vec::new(),
                };
                FollowsResponse {
                    users:   _users,
                    follows: _user.get_followers(_follows_limit, _follows_offset),
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
