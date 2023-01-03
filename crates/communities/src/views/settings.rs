use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_user, get_owner_data, get_community,
    ErrorParams, SmallData, EditPrivateResp, 
    EditNameResp, EditLinkResp, MinimalData,
    KeyValue, EditNotifyResp, COMMUNITIES_SERVICES, TOKEN,
    ObjectData,
};
use crate::AppState;
use crate::models::{User, Community};
use crate::errors::Error;
use serde::{Deserialize, Serialize};


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/settings/get_link", web::get().to(edit_link_page));
    config.route("/settings/get_name", web::get().to(edit_name_page));
    config.route("/settings/get_private", web::get().to(edit_private_page));
    config.route("/settings/get_notifies", web::get().to(edit_notifies_page));

    config.route("/settings/edit_link", web::post().to(edit_link));
    config.route("/settings/edit_name", web::post().to(edit_name));
    config.route("/settings/edit_private", web::post().to(edit_private));
    config.route("/settings/edit_notify", web::post().to(edit_notify));
    config.route("/settings/delete_community", web::post().to(delete_community));
    config.route("/settings/restore_community", web::post().to(restore_community));
}  

pub async fn edit_notifies_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditNotifyResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if (user_id == 0 && community_id == 0)
            || 
            (community_id == 0 && params.community_id.is_none())
             {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let c_id: i32;
            if community_id > 0 {
                c_id = community_id;
            }
            else {
                c_id = params.community_id.unwrap();
            }
            let owner_res = get_community(c_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.get_notify_json()).await?;
                Ok(Json(body))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
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

pub async fn edit_private_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditPrivateResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if (user_id == 0 && community_id == 0)
            || 
            (community_id == 0 && params.community_id.is_none())
             {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let c_id: i32;
            if community_id > 0 {
                c_id = community_id;
            }
            else {
                c_id = params.community_id.unwrap();
            }
            let owner_res = get_community(c_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.get_private_json()).await?;
                Ok(Json(body))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
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

pub async fn edit_link_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditLinkResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if (user_id == 0 && community_id == 0)
            || 
            (community_id == 0 && params.community_id.is_none())
             {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let c_id: i32;
            if community_id > 0 {
                c_id = community_id;
            }
            else {
                c_id = params.community_id.unwrap();
            }
            let owner_res = get_community(c_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                Ok(Json(EditLinkResp {
                    community_id: owner.id,
                    link: owner.link
                }))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
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
pub async fn edit_name_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditNameResp>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if (user_id == 0 && community_id == 0)
            || 
            (community_id == 0 && params.community_id.is_none())
             {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let c_id: i32;
            if community_id > 0 {
                c_id = community_id;
            }
            else {
                c_id = params.community_id.unwrap();
            }
            let owner_res = get_community(c_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                Ok(Json(
                    EditNameResp {
                        community_id: owner.id,
                        name: owner.name,
                }))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
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


#[derive(Serialize, Deserialize)]
pub struct EditLinkData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub link:         Option<String>,
}
pub async fn edit_link (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditLinkData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
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
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let link = data.link.clone();
            let body = block(move || owner.edit_link(data.link.as_deref().unwrap())).await?;
            if body == 1 {
                let copy_community = EditLinkData {
                    token:        Some(TOKEN.to_string()),
                    community_id: Some(c_id),
                    link:         link,
                };
                for link in COMMUNITIES_SERVICES.iter() {
                    let client = reqwest::Client::new();
                    let _res = client.post(link.to_string() + &"/edit_community_link".to_string())
                        .form(&copy_community)
                        .send()
                        .await;
                }
            }
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

#[derive(Serialize, Deserialize)] 
pub struct EditNameData {
    pub token:      Option<String>,
    pub community_id: Option<i32>,
    pub name: Option<String>,
}

pub async fn edit_name (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditNameData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let name = data.name.clone();
            let body = block(move || owner.edit_name (
                data.name.as_deref().unwrap(),
            )).await?;

            let copy_community = EditNameData {
                token:      Some(TOKEN.to_string()),
                community_id: Some(c_id),
                name: name,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/edit_community_name".to_string())
                    .form(&copy_community)
                    .send()
                    .await;
            }
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EditPrivateData {
    pub token:   Option<String>,
    pub community_id: Option<i32>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<i32>>,
}
pub async fn edit_private (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditPrivateData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body)) 
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
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
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let body = block(move || owner.edit_private ( 
                data.field.as_deref().unwrap(),
                data.value.unwrap(),
                data.users.clone()
            )).await?;
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

pub async fn delete_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<SmallData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body)) 
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let body = block(move || owner.delete_item()).await?;
            let copy_community = SmallData {
                token:   Some(TOKEN.to_string()),
                community_id: Some(c_id),
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new();
                let _res = client.post(link.to_string() + &"/delete_community".to_string())
                    .form(&copy_community)
                    .send()
                    .await;
            }
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}
pub async fn restore_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<MinimalData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body)) 
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let body = block(move || owner.restore_item()).await?;

            let copy_community = MinimalData { 
                token:   Some(TOKEN.to_string()),
                community_id: Some(c_id),
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/restore_community".to_string())
                    .form(&copy_community)
                    .send()
                    .await;
            }

            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

pub async fn edit_notify (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<MinimalData>
) -> Result<Json<EditNotifyResp>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
     if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        Err(Error::BadRequest(body)) 
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && data.community_id.is_none())
            {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let owner: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let owner_res = get_community(c_id);
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
            let body = block(move || owner.get_notify_json()).await?;
            Ok(Json(body))
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}