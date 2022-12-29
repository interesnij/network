use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user, get_user_owner_data,
    ErrorParams, SmallData, EditPrivateResp, 
    EditNameResp, EditPhoneResp, EditLinkResp,
    KeyValue, EditNotifyResp, USERS_SERVICES, TOKEN, 
};
use crate::AppState;
use crate::models::User;
use crate::errors::Error;
use serde::{Deserialize, Serialize};


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/settings/get_link", web::get().to(edit_link_page));
    config.route("/settings/get_name", web::get().to(edit_name_page));
    config.route("/settings/get_phone", web::get().to(edit_phone_page));
    config.route("/settings/get_private", web::get().to(edit_private_page));
    config.route("/settings/get_delete_account", web::get().to(delete_account_page));
    config.route("/settings/get_notifies", web::get().to(edit_notifies_page));

    config.route("/settings/edit_link", web::post().to(edit_link));
    config.route("/settings/edit_name", web::post().to(edit_name));
    config.route("/settings/edit_password", web::post().to(edit_password));
    config.route("/settings/edit_phone", web::post().to(edit_phone));
    config.route("/settings/edit_private", web::post().to(edit_private));
    config.route("/settings/edit_notify", web::post().to(edit_notify));
    config.route("/settings/delete_account", web::post().to(delete_account));
    config.route("/settings/restore_account", web::post().to(restore_account));
}  

pub async fn edit_notifies_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditNotifyResp>, Error> {
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_notify_json()).await?;
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

pub async fn delete_account_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<KeyValue>>, Error> {
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
            let owner_res = get_user(user_id);
            if owner_res.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            let mut list = Vec::new();
            list.push(KeyValue {
                value: 1,
                info:  "У меня есть другой аккаунт".to_string(),
            });
            list.push(KeyValue {
                value: 2,
                info:  "Соцсеть отнимает много времени".to_string(),
            });
            list.push(KeyValue {
                value: 3,
                info:  "Мало свободы самовыражения".to_string(),
            });
            list.push(KeyValue {
                value: 4,
                info:  "Соцсеть плохо защищает данные".to_string(),
            });
            list.push(KeyValue {
                value: 5,
                info:  "Соцсеть скучная".to_string(),
            });
            list.push(KeyValue {
                value: 6,
                info:  "Соцсеть плохо работает".to_string(),
            });
            list.push(KeyValue {
                value: 7,
                info:  "Другая причина".to_string(),
            });
            Ok(Json(list))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn edit_private_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditPrivateResp>, Error> {
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.get_private_json()).await?;
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

pub async fn edit_link_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditLinkResp>, Error> {
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            Ok(Json(EditLinkResp{link: owner.link}))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
pub async fn edit_name_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditNameResp>, Error> {
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            Ok(Json(
                EditNameResp {
                    first_name: owner.first_name,
                    last_name: owner.last_name
                }))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
pub async fn edit_phone_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditPhoneResp>, Error> {
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
            let owner: User;
            let owner_res = get_user(user_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            Ok(Json(EditPhoneResp{phone: owner.phone}))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}


#[derive(Serialize, Deserialize)]
pub struct EditLinkData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub link:    Option<String>,
}
pub async fn edit_link (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditLinkData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
    else if data.link.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'link' is required!".to_string(),
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
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        let copy_user = EditLinkData {
            token:   Some(TOKEN.to_string()),
            user_id: Some(user_id),
            link:    data.link.clone(),
        };
        let body = block(move || owner.edit_link(data.link.as_deref().unwrap())).await?;
        if body == 1 {
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/edit_user_link".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }
        }
    
        Ok(Json(body))
    }
}

#[derive(Deserialize)]
pub struct EditPhoneData {
    pub token:   Option<String>,
    pub phone:   Option<String>,
}
pub async fn edit_phone (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditPhoneData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
    else if data.phone.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'phone' is required!".to_string(),
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
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        let body = block(move || owner.edit_phone(data.phone.as_deref().unwrap())).await?;
        Ok(Json(body))
    }
}

#[derive(Serialize, Deserialize)] 
pub struct EditNameData {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}

pub async fn edit_name (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditNameData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
    else if data.first_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'first_name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.last_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'last_name' is required!".to_string(),
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
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        let first_name = data.first_name.clone();
        let last_name = data.last_name.clone();
        let body = block(move || owner.edit_name (
            data.first_name.as_deref().unwrap(),
            data.last_name.as_deref().unwrap()
        )).await?;

        let copy_user = EditNameData {
            token:      Some(TOKEN.to_string()),
            user_id:    Some(user_id),
            first_name: first_name,
            last_name:  last_name,
        };
    
        for link in USERS_SERVICES.iter() {
            let client = reqwest::Client::new();
            let _res = client.post(link.to_string() + &"/edit_user_name".to_string())
                .form(&copy_user)
                .send()
                .await;
        }

        Ok(Json(body))
    }
}

#[derive(Deserialize)]
pub struct EditPasswordData {
    pub token:        Option<String>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
}
#[derive(Serialize)]
pub struct EditPasswordResp {
    pub token:    Option<String>,
    pub password: Option<String>,
}

pub async fn edit_password (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditPasswordData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
    else if data.old_password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'old_password' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.new_password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'new_password' is required!".to_string(),
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
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        use bcrypt::hash;

        let old = hash(data.old_password.as_deref().unwrap(), 8).unwrap();
        let new = hash(data.new_password.as_deref().unwrap(), 8).unwrap();
        let new_2 = old.clone();

        if owner.password == old && old != new {
            let body = block(move || owner.edit_password(&new_2)).await?;

            let copy_user = EditPasswordResp {
                token:     Some(TOKEN.to_string()),
                user_id:   Some(user_id),
                password:  Some(new.clone()),
            };
    
            for link in USERS_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/edit_user_password".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EditPrivateData {
    pub token:   Option<String>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<i32>>,
}
pub async fn edit_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditPrivateData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
    else if data.value.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'value' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.field.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'field' is required!".to_string(),
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
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let field = data.field.as_deref().unwrap();
        let value = data.value;
        let _users = data.users.clone();
        if &field == &"see_all" {
            let copy_user = EditPrivateData {
                token:   Some(TOKEN.to_string()),
                user_id: Some(user_id),
                field:   Some("see_all".to_string()),
                value:   value.clone(),
                users:   _users.clone(),
            };
    
            for link in USERS_SERVICES.iter() { 
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/edit_user_all_private".to_string())
                    .form(&copy_user)
                    .send()
                    .await;
            }
        }
        
        let body = block(move || owner.edit_private ( 
            data.field.as_deref().unwrap(),
            data.value.unwrap(),
            data.users.clone()
        )).await?;
        Ok(Json(body))
    }
}

#[derive(Serialize, Deserialize)]
pub struct MinimalData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
}
pub async fn delete_account (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<MinimalData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
        let owner: User;
        let owner_res = get_user(user_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let body = block(move || owner.delete_item()).await?;

        let copy_user = MinimalData {
            token:   Some(TOKEN.to_string()),
            user_id: data.user_id,
        };
    
        for link in USERS_SERVICES.iter() {
            let client = reqwest::Client::new();
            let _res = client.post(link.to_string() + &"/delete_user".to_string())
                .form(&copy_user)
                .send()
                .await;
        }
        Ok(Json(body))
    }
}
pub async fn restore_account (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<MinimalData>
) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
        let owner: User;
        let owner_res = get_user(user_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let body = block(move || owner.restore_item()).await?;

        let copy_user = MinimalData {
            token:   Some(TOKEN.to_string()),
            user_id: Some(user_id),
        };
    
        for link in USERS_SERVICES.iter() {
            let client = reqwest::Client::new();
            let _res = client.post(link.to_string() + &"/delete_user".to_string())
                .form(&copy_user)
                .send()
                .await;
        }

        Ok(Json(body))
    }
}

pub async fn edit_notify (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<MinimalData>
) -> Result<Json<EditNotifyResp>, Error> {
    let (err, user_id) = get_user_owner_data(&req, state, data.token.clone(), 31).await;
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
        let owner: User;
        let owner_res = get_user(user_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "owner not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let body = block(move || owner.get_notify_json()).await?;
        Ok(Json(body))
    }
}