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
    CardCommunityJson, RegListData,
    get_owner_data, ErrorParams,
};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/all-communities", web::get().to(all_communities_page));
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


