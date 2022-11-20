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
};
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    config.route("/all-communities/", web::get().to(all_communities_page));
}

pub async fn all_communities_page(req: HttpRequest) -> Json<String> {
    let communities = reqwest::get("http:194.58.90.123:9002/all-communities")
        .await
        .expect("E.")
        .text()
        .await;
    Json(communities.expect("E."))
}
