use actix_web::{
    //HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    RegListData,
    CardCommunityJson,
}


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    config.route("/all-communities/", web::get().to(all_communities_page));
}

pub async fn all_communities_page(req: HttpRequest) -> Result<Json<Vec<CardCommunityJson>>, Error> {
    //let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    //if params_some.is_ok() {
    //    let params = params_some.unwrap();
        let users: Vec<CardCommunityJson> = reqwest::Client::new()
            .get("http:194.58.90.123:9002/all-communities")
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", todos);
        Ok(users)
    //}
    //else {
    //    let body = serde_json::to_string(&ErrorParams {
    //        error: "parametrs not found!".to_string(),
    //    }).unwrap();
    //    Err(Error::BadRequest(body))
    //}
}
