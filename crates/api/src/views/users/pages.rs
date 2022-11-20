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
    CardUserJson,
    ErrorParams,
};
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    //config.route("/friends/", web::get().to(user_friends_page));
    //config.route("/friends-online/", web::get().to(user_friends_online_page));
    //config.route("/friends-common/", web::get().to(user_friends_common_page));
    //config.route("/follows/", web::get().to(user_follows_page));
    config.route("/all-users/", web::get().to(all_users_page));
}

pub async fn all_users_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let users = reqwest::Client::new()
            .get("http:194.58.90.123:9001/all-users")
            .send()
            //.await?
            //.json()
            .await;

        println!("{:#?}", users);
        Ok(Json(users))
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
