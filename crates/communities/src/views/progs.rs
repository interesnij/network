use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_owner_data, get_community, get_user,
    ErrorParams, SmallData, MinimalData, EditCommunityPrivateData,
    EditNotifyResp, COMMUNITIES_SERVICES, TOKEN,
    ObjectData,
};
use crate::AppState;
use crate::models::{Community, User};
use crate::errors::Error;
use serde::{Deserialize, Serialize};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/settings/edit_private", web::post().to(edit_community_private));
    config.route("/settings/edit_link", web::post().to(edit_link));
    config.route("/settings/edit_name", web::post().to(edit_name));
    config.route("/settings/edit_notify", web::post().to(edit_notify));
    config.route("/settings/delete_community", web::post().to(delete_community));
    config.route("/settings/restore_community", web::post().to(restore_community));
    config.route("/settings/edit_status", web::post().to(edit_status));
    
    config.route("/settings/create_ban", web::post().to(create_ban));
    config.route("/settings/delete_ban", web::post().to(delete_ban));
    config.route("/settings/create_administrator", web::post().to(create_administrator));
    config.route("/settings/create_advertiser", web::post().to(create_advertiser));
    config.route("/settings/create_editor", web::post().to(create_editor));
    config.route("/settings/create_moderator", web::post().to(create_moderator));
    config.route("/settings/delete_staff", web::post().to(delete_staff));
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
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub name:         Option<String>,
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
pub struct EditStatusData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub status:       Option<String>,
}

pub async fn edit_status (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditStatusData>
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
    else if data.status.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'status' is required!".to_string(),
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
            let body = block(move || owner.edit_status (
                data.status.as_deref().unwrap(),
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

pub async fn edit_community_private (
    req: HttpRequest,
    state: web::Data<AppState>, 
    data: Json<EditCommunityPrivateData>
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


#[derive(Serialize, Deserialize)] 
pub struct BanData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub user_id:      Option<i32>,
    pub types:        Option<i16>,
}
pub async fn create_ban (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<BanData>
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
    else if data.user_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        let user: User;
        let user_res = get_user(user_id);
        if user_res.is_ok() {
            user = user_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let types = data.types;
            let body = block(move || community.create_banned_user (
                user,
                data.types.unwrap()
            )).await?;

            let copy_community = BanData { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      Some(user_id),
                types:        types,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/create_block_user".to_string())
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

pub async fn delete_ban (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        let user: User;
        let user_res = get_user(data.id.unwrap());
        if user_res.is_ok() {
            user = user_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "user not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let types = data.types;
            let target_id = data.id;
            let body = block(move || community.delete_banned_user (
                user,
            )).await?;

            let copy_community = ObjectData { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/delete_block_user".to_string())
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

#[derive(Serialize)] 
pub struct UpdateLevel {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub user_id:      Option<i32>,
    pub level:        Option<i16>,
}
pub async fn create_administrator (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let target_id = data.id;
            let body = block(move || community.create_administrator (
                data.id.unwrap(),
            )).await?;

            let copy_community = UpdateLevel { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
                level:        5,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/update_staff_member".to_string())
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
pub async fn create_editor (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let target_id = data.id;
            let body = block(move || community.create_editor (
                data.id.unwrap(),
            )).await?;

            let copy_community = UpdateLevel { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
                level:        3,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/update_staff_member".to_string())
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
pub async fn create_advertiser (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let target_id = data.id;
            let body = block(move || community.create_advertiser (
                data.id.unwrap(),
            )).await?;

            let copy_community = UpdateLevel { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
                level:        4,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/update_staff_member".to_string())
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
pub async fn create_moderator (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let target_id = data.id;
            let body = block(move || community.create_moderator (
                data.id.unwrap(),
            )).await?;

            let copy_community = UpdateLevel { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
                level:        2,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/update_staff_member".to_string())
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
pub async fn delete_staff (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ObjectData>
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
    else if data.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'user_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = data.community_id.unwrap();
        }
        let community_res = get_community(c_id);
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }

        if community_id > 0 || community.is_user_see_settings(user_id) {
            let target_id = data.id;
            let body = block(move || community.delete_staff_member (
                data.id.unwrap(),
            )).await?;

            let copy_community = UpdateLevel { 
                token:        Some(TOKEN.to_string()),
                community_id: Some(c_id),
                user_id:      target_id,
                level:        0,
            };
    
            for link in COMMUNITIES_SERVICES.iter() {
                let client = reqwest::Client::new(); 
                let _res = client.post(link.to_string() + &"/update_staff_member".to_string())
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