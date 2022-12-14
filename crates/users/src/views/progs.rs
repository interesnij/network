use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user, get_user_owner_data,
    ErrorParams, UsersData,
    TOKEN, USERS_SERVICES,
};
use crate::models::User;
use crate::errors::Error;
use serde::Serialize;
use crate::AppState;


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/progs/block", web::post().to(user_block));
    config.route("/progs/unblock", web::post().to(user_unblock));
    config.route("/progs/friend", web::post().to(user_friend));
    config.route("/progs/unfriend", web::post().to(user_unfriend));
    config.route("/progs/follow", web::post().to(user_follow));
    config.route("/progs/follow_view", web::post().to(user_follow_view));
    config.route("/progs/unfollow", web::post().to(user_unfollow));
}  

#[derive(Serialize)]
pub struct AddTargetParams {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
}

pub async fn user_block (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) { 
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.block_user(target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/create_block_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}

pub async fn user_unblock (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.unblock_user(target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/delete_block_user".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}


pub async fn user_friend (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.frend_user(target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/create_friend".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}

pub async fn user_unfriend (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.unfrend_user (target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/delete_friend".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}


pub async fn user_follow (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.follow_user (target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/create_follow".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}
pub async fn user_follow_view (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.follow_view_user (target_user.id)).await?;
            Ok(Json(_res))
        }
    }
}

pub async fn user_unfollow (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<UsersData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() || (user_id == 0) {
        // если проверка токена не удалась или запрос анонимный...
        Err(Error::BadRequest(err.unwrap()))
    } 
    else if data.target_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'target_id' is required!".to_string(),
        }).unwrap(); 
        Err(Error::BadRequest(body))
    }
    else {
        let request_user: User;
        let target_user: User;
        let request_user_res = get_user(user_id);
        let target_user_res = get_user(data.target_id.unwrap());
        if request_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "request_user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        else if target_user_res.is_err() {
            let body = serde_json::to_string(&ErrorParams {
                   error: "target_user not found!".to_string(),
             }).unwrap();
             return Err(Error::BadRequest(body));
        }
        else {
            request_user = request_user_res.expect("E");
            target_user = target_user_res.expect("E");
            let _res = block(move || request_user.unfollow_user (target_user)).await?;

            let copy_user = AddTargetParams {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                target_id: data.target_id,
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/delete_follow".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }

            Ok(Json(_res))
        }
    }
}