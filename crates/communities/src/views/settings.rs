use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_owner_data, get_community,
    ErrorParams, EditPrivateResp, 
    EditNameResp, EditLinkResp, MinimalData,
    EditNotifyResp,
    ObjectData, CardUserJson, RegListData, SearchRegListData,
};
use crate::AppState;
use crate::models::Community;
use crate::errors::Error;


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/settings/get_link", web::get().to(edit_link_page));
    config.route("/settings/get_name", web::get().to(edit_name_page));
    config.route("/settings/get_private", web::get().to(edit_private_page));
    config.route("/settings/get_notifies", web::get().to(edit_notifies_page));

    config.route("/settings/blacklist/", web::get().to(blacklist_settings_page));
    config.route("/settings/administrators/", web::get().to(administrators_settings_page));
    config.route("/settings/editors/", web::get().to(editors_settings_page));
    config.route("/settings/moderators/", web::get().to(moderators_settings_page));
    config.route("/settings/advertisers/", web::get().to(advertisers_settings_page));
    config.route("/settings/search-blacklist/", web::get().to(search_blacklist_settings_page));
    config.route("/settings/search-administrators/", web::get().to(search_administrators_settings_page));
    config.route("/settings/search-editors/", web::get().to(search_editors_settings_page));
    config.route("/settings/search-moderators/", web::get().to(search_moderators_settings_page));
    config.route("/settings/search-advertisers/", web::get().to(search_advertisers_settings_page));
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


pub async fn blacklist_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
                let body = block(move || owner.get_banned_user (
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn administrators_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
                let body = block(move || owner.get_administrators (
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn editors_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
                let body = block(move || owner.get_editors (
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn advertisers_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
                let body = block(move || owner.get_advertisers (
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn moderators_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
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
                let body = block(move || owner.get_moderators (
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_blacklist_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.search_banned_user (
                    &q,
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_advertisers_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.search_advertisers (
                    &q,
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_administrators_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.search_administrators (
                    &q,
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_editors_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.search_editors (
                    &q,
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_moderators_settings_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0 || (user_id > 0 && owner.is_user_see_settings(user_id)) {
                let body = block(move || owner.search_moderators (
                    &q,
                    params.limit,
                    params.offset
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}