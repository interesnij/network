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

pub async fn all_postlists_page(req: HttpRequest) -> Result<Json<Vec<CardPostListJson>>, Error> {
    //let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    //if params_some.is_ok() {
    //    let params = params_some.unwrap();
        let postlists = reqwest::get("http:194.58.90.123:9003/all-postlists")
            .send()
            .await?
            .text()
            .await?;

        Ok(Json(postlists))
    //}
    //else {
    //    let body = serde_json::to_string(&ErrorParams {
    //        error: "parametrs not found!".to_string(),
    //    }).unwrap();
    //    Err(Error::BadRequest(body))
    //}
}
