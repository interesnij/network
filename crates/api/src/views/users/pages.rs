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
    config.route("/all-users", web::get().to(all_users_page));
}

pub async fn all_users_page(req: HttpRequest) -> Json<String> {
    let users = reqwest::get("http:194.58.90.123:9001/all-users")
        .await
        .expect("E.")
        .text()
        .await;
    println!("status {:?}", users.status);
    println!("is_status {:?}", users.is_status);
    println!("is_timeout {:?}", users.is_timeout);
    match users {
        Ok(_ok) => Json(_ok),
        Err(_error) => Json(_error.to_string()),
    }
}
