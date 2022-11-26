use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user,
    get_user_owner_data,
    ErrorParams, SmallData,
    EditNameResp, EditPhoneResp, EditLinkResp,
};
use crate::models::{User, };
use crate::errors::Error;
use serde::Deserialize;


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/settings/edit_link", web::get().to(edit_link_page));
    config.route("/settings/edit_name", web::get().to(edit_name_page));
    config.route("/settings/edit_phone", web::get().to(edit_phone_page));

    config.route("/settings/edit_link", web::post().to(edit_link));
    config.route("/settings/edit_name", web::post().to(edit_name));
    config.route("/settings/edit_password", web::post().to(edit_password));
    config.route("/settings/edit_phone", web::post().to(edit_phone));
    //config.route("/settings/remove_profile", web::post().to(remove_profile));
}  


#[derive(Deserialize)] 
pub struct EditNameData {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}

#[derive(Deserialize)]
pub struct EditPhoneData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub phone:   Option<String>,
}

#[derive(Deserialize)]
pub struct EditPasswordData {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Deserialize)]
pub struct EditLinkData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub link:    Option<String>,
}


pub async fn edit_link_page(req: HttpRequest) -> Result<Json<EditLinkResp>, Error> {
    let params_some = web::Query::<SmallData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 31);
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
pub async fn edit_name_page(req: HttpRequest) -> Result<Json<EditNameResp>, Error> {
    let params_some = web::Query::<SmallData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 31);
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
pub async fn edit_phone_page(req: HttpRequest) -> Result<Json<EditPhoneResp>, Error> {
    let params_some = web::Query::<SmallData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id, 31);
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

pub async fn edit_link(data: Json<EditLinkData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
        Ok(Json(owner.edit_link(data.link.as_deref().unwrap())))
    }
}

pub async fn edit_phone(data: Json<EditPhoneData>) -> Result<Json<i16>, Error> {
    use crate::models::PhoneCode;

    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
        
        Ok(Json(owner.edit_phone(data.phone.as_deref().unwrap())))
    }
}

pub async fn edit_name(data: Json<EditNameData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
        Ok(Json(owner.edit_name(
            data.first_name.as_deref().unwrap(),
            data.last_name.as_deref().unwrap()
        )))
    }
}

pub async fn edit_password(data: Json<EditPasswordData>) -> Result<Json<i16>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), data.user_id, 31);
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
        Ok(Json(owner.edit_password (
            data.old_password.as_deref().unwrap(),
            data.new_password.as_deref().unwrap()
        ))
    }
}