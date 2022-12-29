//use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_user_owner_data, get_user, verify_jwt,
    get_user_permission,
    get_anon_user_permission,
    ErrorParams, CardUserJson,
    RegListData, SearchRegListData,
    TargetListData, SearchTargetListData,
    UsersData, UserDetailJson,
};
use crate::models::{User, Owner, TokenJson};
use crate::errors::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::AppState;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/test_all_tokens", web::get().to(test_all_tokens));
    config.route("/profile", web::get().to(profile_page));
    config.route("/friends", web::get().to(user_friends_page));
    config.route("/friends-online", web::get().to(user_friends_online_page));
    config.route("/friends-common", web::get().to(user_friends_common_page));
    config.route("/followings", web::get().to(user_followings_page));
    config.route("/blacklist", web::get().to(user_blacklist_page));
    config.route("/follows", web::get().to(user_follows_page));
    config.route("/friends-featured", web::get().to(user_featured_page));
    config.route("/all-users", web::get().to(all_users_page));

    config.route("/search-friends", web::get().to(search_user_friends_page));
    config.route("/search-friends-online", web::get().to(search_user_friends_online_page));
    config.route("/search-friends-common", web::get().to(search_user_friends_common_page));
    config.route("/search-followings", web::get().to(search_user_followings_page));
    config.route("/search-blacklist", web::get().to(search_user_blacklist_page));
    config.route("/search-follows", web::get().to(search_user_follows_page));
    config.route("/search-all-users", web::get().to(search_all_users_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I users server.
            </p>
        </div>")
}

pub async fn test_all_tokens() -> Result<Json<Vec<TokenJson>>, Error> {
    let _res = block(move || Owner::get_all_tokens()).await?;
     Ok(Json(_res))
}

pub async fn all_users_page (
    //_auth: BearerAuth, 
    req: HttpRequest, 
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
    use actix_web::http::header::Header;

    let auth = Authorization::<Bearer>::parse(&req);
    match auth {
        Ok(ok) => println!("user authenticate!"),
        Err(_) => {
            let body = serde_json::to_string(&ErrorParams {
                error: "user not authenticate!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        },
    } 
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    //match verify_jwt(_auth.token().to_string(), state.key.as_ref()).await {
    //    Ok(ok) => println!("id {:?}", ok.id),
    //    Err(err) => println!("err {:?}", err),
    //}
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, _user_id) = get_user_owner_data(params.token.clone(), params.user_id, 0);
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap(); 
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || User::get_users (
                params.limit,
                params.offset
            )).await?;
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

pub async fn profile_page(req: HttpRequest) -> Result<Json<UserDetailJson>, Error> {
    let params_some = web::Query::<UsersData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'target_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_user_detail_json()).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_user_detail_json()).await?;
                    Ok(Json(body))
                }
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

pub async fn user_friends_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<TargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'target_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_friends(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_friends(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
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

pub async fn user_friends_online_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<TargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'target_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_online_friends(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_online_friends(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
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

pub async fn user_friends_common_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<TargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'target_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 && user_id != owner.id {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let user = get_user(user_id).expect("E");
                    let body = block(move || owner.get_common_friends_of_user(&user, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied".to_string(),
                }).unwrap();
                Err(Error::BadRequest(body))
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

pub async fn user_followings_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
            let body = block(move || owner.get_followings(params.limit, params.offset)).await?;
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

pub async fn user_follows_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<TargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'target_id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_followers(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_followers(params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
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

pub async fn user_featured_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
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
            let body = block(move || owner.get_featured_friends(params.limit, params.offset)).await?;
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

pub async fn user_blacklist_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
            let body = block(move || owner.get_blocked_users(params.limit, params.offset)).await?;
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

pub async fn search_all_users_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, _user_id) = get_user_owner_data(params.token.clone(), params.user_id, 0);
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let _res = block(move || User::search_users(&q, params.limit, params.offset)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_friends_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_friends(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_friends(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_friends_online_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_online_friends(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_online_friends(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_friends_common_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
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
            if user_id > 0 && user_id != owner.id {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let user = get_user(user_id).expect("E");
                    let body = block(move || owner.search_common_friends_of_user(&q, &user, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied".to_string(),
                }).unwrap();
                Err(Error::BadRequest(body))
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_followings_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
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
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
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
            let body = block(move || owner.search_followings(&q, params.limit, params.offset)).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_blacklist_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
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
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is required!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
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
            let body = block(move || owner.search_blocked_users(&q, params.limit, params.offset)).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_follows_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
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
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' not required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());

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
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_followers(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    // если пользователь не может просматривать информацию владельца списка
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_followers(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
