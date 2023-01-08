use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_owner_data, get_community, get_memberships_list,
    ErrorParams, EditCommunityPrivateResp, ObjectData,
    EditNameResp, EditLinkResp, EditNotifyResp,
    CardUserJson, RegListData, SearchRegListData,
    RespListJson,
};
use crate::AppState;
use crate::models::{Community, MembershipsList};
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

    config.route("/settings/create_memberships_list/", web::post().to(create_memberships_list));
    config.route("/settings/edit_memberships_list/", web::post().to(edit_memberships_list));
    config.route("/settings/delete_memberships_list/", web::post().to(delete_memberships_list));
    config.route("/settings/restore_memberships_list/", web::post().to(restore_memberships_list));
    config.route("/settings/add_member_in_memberships_list/", web::post().to(add_member_in_memberships_list));    
    config.route("/settings/delete_member_from_memberships_list/", web::post().to(delete_member_from_memberships_list));
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
) -> Result<Json<EditCommunityPrivateResp>, Error> {
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



#[derive(Deserialize)]
pub struct CreateListData {
    pub token:  Option<String>,
    pub name:   Option<String>,
    pub see_el: Option<i16>,
    pub users:  Option<Vec<i32>>,
} 
pub async fn create_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CreateListData>
) -> Result<Json<RespListJson>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.see_el.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'see_el' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let body = block(move || MemnershipsList::create_list (
            data.name.unwrap(),
            user_id,
            data.see_el.unwrap(),
            data.users.clone(),
        )).await?;
        return Ok(Json(body));
    }
}

#[derive(Deserialize)]
pub struct EditListData {
    pub token:    Option<String>,
    pub list_id:  Option<String>,
    pub name:     Option<String>,
    pub see_el:   Option<i16>,
    pub position: Option<i16>,
    pub users:    Option<Vec<i32>>,
}
pub async fn edit_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<EditListData>
) -> Result<Json<RespListJson>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'name' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.see_el.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'see_el' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let owner: MembershipsList;
        let owner_res = get_memberships_list(data.list_id.unwrap());
        if owner_res.is_ok() {
            owner = owner_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "memberships list not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if owner.user_id == user_id {
            let position: i16;
            if data.position.is_some() {
                position = data.position.unwrap();
            }
            else {
                position = 2;
            }
            let body = block(move || owner.edit_list (
                data.list_id.unwrap(),
                data.name.unwrap(),
                data.see_el.unwrap(),
                position,
                data.users.clone(),
            )).await?;
            return Ok(Json(body));
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteListData {
    pub token:   Option<String>,
    pub list_id: Option<i32>,
}
pub async fn delete_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<DeleteListData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = params.community_id.unwrap();
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

        let list: MembershipsList;
        let list_res = get_memberships_list(data.list_id.unwrap());
        if list_res.is_ok() {
            list = list_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "memberships list not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community.is_user_see_settings(user_id) || list.community == community_id {
            let body = block(move || list.delete_item()).await?;
            return Ok(Json(body));
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}
pub async fn restore_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<DeleteListData>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = params.community_id.unwrap();
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

        let list: MembershipsList;
        let list_res = get_memberships_list(data.list_id.unwrap());
        if list_res.is_ok() {
            list = list_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "memberships list not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community.is_user_see_settings(user_id) || list.community_id == community_id {
            let body = block(move || list.restore_item()).await?;
            return Ok(Json(body));
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}

#[derive(Deserialize)]
pub struct ItemListData2 {
    pub token:        Option<String>,
    pub list_id:      Option<i32>,
    pub community_id: Option<i32>,
}
pub async fn add_member_in_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ItemListData2>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = params.community_id.unwrap();
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

        let list: MembershipsList;
        let list_res = get_memberships_list(data.list_id.unwrap());
        if list_res.is_ok() {
            list = list_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
            error: "memberships list not found!".to_string(),
        }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community.is_user_see_settings(user_id) || list.community_id == community_id {
            let body = block(move || list.create_membership_item (
                data.community_id.unwrap(),
            )).await?;
            return Ok(Json(body));
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}
pub async fn delete_member_in_memberships_list (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<ItemListData2>
) -> Result<Json<i16>, Error> {
    let (err, user_id, community_id) = get_owner_data(&req, state, data.token.clone(), 31).await;
    if err.is_some() {
        let body = serde_json::to_string(&ErrorParams {
            error: err.unwrap(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.list_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'list_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if (user_id == 0 && community_id == 0)
        || 
        (community_id == 0 && params.community_id.is_none())
        {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let community: Community;
        let c_id: i32;
        if community_id > 0 {
            c_id = community_id;
        }
        else {
            c_id = params.community_id.unwrap();
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

        let list: MembershipsList;
        let list_res = get_memberships_list(data.list_id.unwrap());
        if list_res.is_ok() {
            list = list_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
            error: "memberships list not found!".to_string(),
        }).unwrap();
            return Err(Error::BadRequest(body));
        }
        if community.is_user_see_settings(user_id) || list.community_id == community_id {
            let body = block(move || MembershipsListItem::delete_memberships_item (
                data.list_id.unwrap(), 
                data.community_id.unwrap(),
            )).await?;
            return Ok(Json(body));
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
}