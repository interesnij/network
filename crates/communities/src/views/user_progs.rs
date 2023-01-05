use actix_web::{
    web,
    web::block,
    web::Json,
    HttpRequest,
};
use crate::utils::{
    get_community, get_user, get_user_owner_data,
    is_anon_user_see_community, is_user_see_community,
    ErrorParams, SmallData, TOKEN,
    EditUserPrivateResp, CardCommunityJson,
};
use crate::AppState;
use crate::models::{Community, User};
use crate::errors::Error;
use serde::{Deserialize, Serialize};


pub fn user_urls(config: &mut web::ServiceConfig) {
    config.route("/get_user_private", web::get().to(edit_user_private_page));
    config.route("/edit_user_private", web::post().to(edit_user_private));
    config.route("/edit_user_all_private", web::post().to(edit_user_all_private));
    config.route("/create_invites", web::post().to(create_invites));
    config.route("/delete_invite", web::post().to(delete_invite));
    config.route("/create_follow", web::post().to(create_follow));
    config.route("/delete_follow", web::post().to(delete_follow));
    config.route("/join_community", web::post().to(join_community));
    config.route("/leave_community", web::post().to(leave_community));
    config.route("/get_list_communities", web::get().to(get_communities_for_list));
    config.route("/get_limit_communities", web::get().to(get_limit_communities));
}  

pub async fn edit_user_private_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<EditUserPrivateResp>, Error> {
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

#[derive(Serialize, Deserialize)]
pub struct EditPrivateData {
    pub token: Option<String>,
    pub field: Option<String>,
    pub value: Option<i16>,
    pub users: Option<Vec<i32>>,
}
pub async fn edit_user_private (
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
        
        let body = block(move || owner.edit_private ( 
            data.field.as_deref().unwrap(),
            data.value.unwrap(),
            data.users.clone()
        )).await?;
        Ok(Json(body))
    }
}

#[derive(Deserialize)]
pub struct AllPrivateData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<i32>>,
} 
pub async fn edit_user_all_private (
    data: Json<AllPrivateData>
) -> Result<Json<i16>, Error> {
    if data.token.is_none() || data.value.is_none() || data.user_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Fields 'value', 'user_id', 'token' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        if data.token.as_deref().unwrap() == TOKEN { 
            let request_user = get_user(data.user_id.unwrap()).expect("E.");
            let _res = block (
                move || request_user.edit_private (
                    "see_all",
                    data.value.unwrap(),
                    data.users.clone(),
                )
                ).await?;
            Ok(Json(_res))
        } else {
            Err(Error::BadRequest("Permission Denied".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct InvitesData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub users:        Option<Vec<i32>>,
} 
pub async fn create_invites (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<InvitesData>
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.users.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'users' is required!".to_string(),
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
        
        let body = block(move || owner.invite_in_community ( 
            data.community_id.unwrap(),
            data.users.as_deref().unwrap().to_vec(),
        )).await?;
        Ok(Json(body))
    }
}
#[derive(Deserialize)]
pub struct CData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
} 
pub async fn delete_invite (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CData>
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
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
        
        let body = block(move || owner.uninvite_community ( 
            data.community_id.unwrap(),
        )).await?;
        Ok(Json(body))
    }
}

pub async fn create_follow (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CData>
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
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
        let community: Community;
        let community_res = get_community(data.community_id.unwrap());
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let body = block(move || user.follow_community ( 
            community, 
        )).await?;
        Ok(Json(body))
    }
}
pub async fn delete_follow (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CData>
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
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
        
        let body = block(move || owner.unfollow_community ( 
            data.community_id.unwrap(),
        )).await?;
        Ok(Json(body))
    }
}

pub async fn join_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CData>
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
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
        let community: Community;
        let community_res = get_community(data.community_id.unwrap());
        if community_res.is_ok() {
            community = community_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "community not found!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
        
        let body = block(move || user.join_community ( 
            community,
        )).await?;
        Ok(Json(body))
    }
}
pub async fn leave_community (
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Json<CData> 
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
    else if data.community_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'community_id' is required!".to_string(),
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
        
        let body = block(move || owner.leave_community ( 
            data.community_id.unwrap(),
        )).await?;
        Ok(Json(body))
    }
}


#[derive(Deserialize)]
pub struct GetCommunitiesParams {
    pub token:     Option<String>,
    pub target_id: Option<i32>,
    pub list_id:   Option<i32>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}
pub async fn get_communities_for_list (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardCommunityJson>>, Error> {
    let params_some = web::Query::<GetCommunitiesParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "user not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else if !owner.is_user_see_community(user_id) {
                    let body = serde_json::to_string(&ErrorParams {
                        error: "Permission Denied!".to_string(),
                    }).unwrap();
                    return Err(Error::BadRequest(body));
                }
                else {
                    let body = block(move || owner.get_communities_of_list (
                        params.list_id,
                        params.limit,
                        params.offset,
                    )).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else if !owner.is_anon_user_see_community(user_id) {
                    let body = serde_json::to_string(&ErrorParams {
                        error: "Permission Denied!".to_string(),
                    }).unwrap();
                    return Err(Error::BadRequest(body));
                }
                else {
                    let body = block(move || owner.get_communities_of_list (
                        params.list_id,
                        params.limit,
                        params.offset,
                    )).await?;
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

#[derive(Deserialize)]
pub struct GetCommunitiesLimitParams {
    pub token:     Option<String>,
    pub target_id: Option<i32>,
    pub limit:     Option<i64>,
}
pub async fn get_limit_communities (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardCommunityJson>>, Error> {
    let params_some = web::Query::<GetCommunitiesLimitParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(&req, state, params.token.clone(), 31).await;
        if err.is_some() {
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
        else {
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "user not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else if !owner.is_user_see_community(user_id) {
                    let body = serde_json::to_string(&ErrorParams {
                        error: "Permission Denied!".to_string(),
                    }).unwrap();
                    return Err(Error::BadRequest(body));
                }
                else {
                    let body = block(move || owner.get_limit_communities (
                        params.limit,
                    )).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else if !owner.is_anon_user_see_community(user_id) {
                    let body = serde_json::to_string(&ErrorParams {
                        error: "Permission Denied!".to_string(),
                    }).unwrap();
                    return Err(Error::BadRequest(body));
                }
                else {
                    let body = block(move || owner.get_limit_communities (
                        params.limit,
                    )).await?;
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