use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    RegListData,
    CardCommunityJson,
    ErrorParams,
    COMMUNITIES_URL,
};
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    config.route("/all-communities/", web::get().to(all_communities_page));
}

pub async fn all_communities_page(req: HttpRequest) -> Json<String> {
    let communities = reqwest::get(COMMUNITIES_URL.to_owned() + &"/all-communities".to_string())
        .await
        .expect("E.")
        .text()
        .await;
    Json(communities.expect("E."))
}
