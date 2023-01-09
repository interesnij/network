use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::block,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::Error;
use crate::models::Community;
use crate::utils::{
    CardCommunityJson, RegListData, ErrorParams, ObjectData,
    CommunityDetailJson, CardUserJson, SearchRegListData,
    get_owner_data, get_anon_community_permission, 
    get_community_permission, get_community, get_user,
    get_user_owner_data,
};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/all-communities", web::get().to(all_communities_page));
    config.route("/profile", web::get().to(profile_page));
    config.route("/members", web::get().to(members_page));
    config.route("/common-members", web::get().to(common_members_page));

    config.route("/search-all-communities", web::get().to(search_all_communities_page));
    config.route("/search-members", web::get().to(search_members_page));
    config.route("/search-common-members", web::get().to(search_common_members_page));

    config.route("/load/include_friends", web::get().to(include_friends_load));
    config.route("/load/exclude_friends", web::get().to(exclude_friends_load));
    config.route("/load/include_follows", web::get().to(include_follows_load));
    config.route("/load/exclude_follows", web::get().to(exclude_follows_load));
    //config.route("/load/include_friends_list", web::get().to(include_friends_list_load));
    //config.route("/load/exclude_friends_list", web::get().to(exclude_friends_list_load));
    //config.route("/load/include_follows_list", web::get().to(include_follows_list_load));
    //config.route("/load/exclude_follows_list", web::get().to(exclude_follows_list_load));

    config.route("/load/include_members", web::get().to(include_members_load));
    config.route("/load/exclude_members", web::get().to(exclude_members_load));
    //config.route("/load/include_members_list", web::get().to(include_members_list_load));
    //config.route("/load/exclude_members_list", web::get().to(exclude_members_list_load));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body(
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I communities server.
            </p>
        </div>")
}

pub async fn all_communities_page (
    req: HttpRequest, 
    state: web::Data<AppState>
) -> Result<Json<Vec<CardCommunityJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, _user_id, _community_id) = get_owner_data(&req, state, params.token.clone(), 0).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap(); 
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || Community::get_all_communities (
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

pub async fn profile_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<CommunityDetailJson>, Error> {
    let params_some = web::Query::<ObjectData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 1).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if community_id == 0 && params.community_id.is_none() {
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
            if community_id > 0 && owner.id == community_id {
                let body = block(move || owner.get_community_detail_json()).await?;
                Ok(Json(body))
            }
            else if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_community_detail_json()).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_community_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_community_detail_json()).await?;
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

pub async fn members_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 1).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        } 
        else if community_id == 0 && params.community_id.is_none() {
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
            if community_id > 0 && owner.id == community_id {
                let body = block(move || owner.get_members (
                    params.limit,
                    params.offset
                )).await?;
                Ok(Json(body)) 
            }
            else if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_members (
                        params.limit,
                        params.offset
                    )).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_community_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.get_members (
                        params.limit,
                        params.offset
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

pub async fn common_members_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 1).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'community_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let owner_res = get_community(params.community_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if community_id > 0  {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                Err(Error::BadRequest(body))
            }
            else if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let user = get_user(user_id).expect("E.");
                    let body = block(move || user.get_common_friends_of_community (
                        owner.id,
                        params.limit,
                        params.offset
                    )).await?;
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

pub async fn search_all_communities_page (
    req: HttpRequest, 
    state: web::Data<AppState>
) -> Result<Json<Vec<CardCommunityJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, _user_id, _community_id) = get_owner_data(&req, state, params.token.clone(), 0).await;
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
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let _res = block(move || Community::search_all_communities (
                &q,
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

pub async fn search_members_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 1).await;
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
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'community_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let owner_res = get_community(params.community_id.unwrap());
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
                Err(Error::BadRequest(body))
            }
            else if community_id > 0 && owner.id == community_id {
                let body = block(move || owner.search_members (
                    &q,
                    params.limit,
                    params.offset
                )).await?;
                Ok(Json(body)) 
            }
            else if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_members (
                        &q,
                        params.limit,
                        params.offset
                    )).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_community_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_members (
                        &q,
                        params.limit,
                        params.offset
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

pub async fn search_common_members_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 1).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'community_id' is required!".to_string(),
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
            else if community_id > 0  {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
                }).unwrap();
                Err(Error::BadRequest(body))
            }
            else if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let user = get_user(user_id).expect("E.");
                    let body = block(move || user.search_common_friends_of_community (
                        owner.id, 
                        &q,
                        params.limit,
                        params.offset
                    )).await?;
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

#[derive(Deserialize)]
pub struct IEFriendsData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub friends_limit:  Option<i64>,
    pub friends_offset: Option<i64>,
}
#[derive(Deserialize)]
pub struct IEFollowsData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub follows_limit:  Option<i64>,
    pub follows_offset: Option<i64>,
}
#[derive(Deserialize)]
pub struct IEMembersData {
    pub token:          Option<String>,
    pub types:          Option<i16>,
    pub users_limit:    Option<i64>,
    pub users_offset:   Option<i64>,
    pub members_limit:  Option<i64>,
    pub members_offset: Option<i64>,
}

#[derive(Serialize)]
pub struct IEFriendsResponse {
    pub users:   Vec<CardUserJson>,
    pub friends: Vec<CardUserJson>,
}
#[derive(Serialize)]
pub struct IEFollowsResponse {
    pub users:   Vec<CardUserJson>,
    pub follows: Vec<CardUserJson>,
}
#[derive(Serialize)]
pub struct IEMembersResponse {
    pub users:   Vec<CardUserJson>,
    pub members: Vec<CardUserJson>,
}

pub async fn include_friends_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEFriendsResponse>, Error> {
    let params_some = web::Query::<IEFriendsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_community_include_friends(params.users_limit, params.users_offset),
                    2 => _user.get_limit_invite_include_friends(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(params.friends_limit, params.friends_offset),
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

pub async fn exclude_friends_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEFriendsResponse>, Error> {
    let params_some = web::Query::<IEFriendsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_community_exclude_friends(params.users_limit, params.users_offset),
                    12 => _user.get_limit_invite_exclude_friends(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_friends(params.friends_limit, params.friends_offset),
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

pub async fn include_follows_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEFollowsResponse>, Error> {
    let params_some = web::Query::<IEFollowsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => _user.get_limit_see_community_include_follows(params.users_limit, params.users_offset),
                    2 => _user.get_limit_invite_include_follows(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_follows(params.friends_limit, params.friends_offset),
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

pub async fn exclude_follows_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEFollowsResponse>, Error> {
    let params_some = web::Query::<IEFollowsData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => _user.get_limit_see_community_exclude_follows(params.users_limit, params.users_offset),
                    12 => _user.get_limit_invite_exclude_follows(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEFriendsResponse {
                    users:   _users,
                    friends: _user.get_follows(params.friends_limit, params.friends_offset),
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

pub async fn include_members_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEMembersResponse>, Error> {
    let params_some = web::Query::<IEMembersData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
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
                let _res = block(move || {
                let community = get_community(community.id).expect("E.");

                let _users = match params.types.unwrap() {
                    1 => community.get_limit_see_member_include_members(params.users_limit, params.users_offset),
                    2 => community.get_limit_see_info_include_members(params.users_limit, params.users_offset),
                    3 => community.get_limit_see_settings_include_members(params.users_limit, params.users_offset),
                    4 => community.get_limit_see_log_include_members(params.users_limit, params.users_offset),
                    5 => community.get_limit_see_stat_include_members(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEMembersResponse {
                    users:   _users,
                    members: _user.get_members(params.members_limit, params.members_offset),
                }
                }).await?;
                Ok(Json(_res))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
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

pub async fn exclude_members_load (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<IEMembersResponse>, Error> {
    let params_some = web::Query::<IEMembersData>::from_query(&req.query_string());
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
        else if params.types.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'types' not found!".to_string(),
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
                let _res = block(move || {
                let community = get_community(community.id).expect("E.");

                let _users = match params.types.unwrap() {
                    11 => community.get_limit_see_member_exclude_members(params.users_limit, params.users_offset),
                    12 => community.get_limit_see_info_exclude_members(params.users_limit, params.users_offset),
                    13 => community.get_limit_see_settings_exclude_members(params.users_limit, params.users_offset),
                    14 => community.get_limit_see_log_exclude_members(params.users_limit, params.users_offset),
                    15 => community.get_limit_see_stat_exclude_members(params.users_limit, params.users_offset),
                    _ => Vec::new(),
                };
                IEMembersResponse {
                    users:   _users,
                    members: _user.get_members(params.members_limit, params.members_offset),
                }
                }).await?;
                Ok(Json(_res))
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied!".to_string(),
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