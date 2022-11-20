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
    CardPostListJson,
    ErrorParams,
};
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    config.route("/all-postlists/", web::get().to(all_postlists_page));
}

pub async fn all_postlists_page(req: HttpRequest) -> Json<String> {
    let postlists = reqwest::get("http:194.58.90.123:9003/all-postlists")
        .await
        .expect("E.")
        .text()
        .await;
    Json(postlists.expect("E."))
}
