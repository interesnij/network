use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::block,
    web::Json,
};
use serde::Deserialize;
use crate::AppState;
use crate::errors::Error;
use crate::models::{User, Community, Owner, TokenJson};
use crate::utils::{
    CardCommunityJson, RegListData, ErrorParams, ObjectData,
    CommunityDetailJson,
    get_owner_data, get_anon_community_permission, 
    get_community_permission, get_community,
};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/all-communities", web::get().to(all_communities_page));
    config.route("/profile", web::get().to(profile_page));
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
        else if params.id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'id' not found!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let owner: Community;
            let owner_res = get_community(params.id.unwrap());
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

