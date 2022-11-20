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
    let users = reqwest::get("http:194.58.90.123:9001/all-users").await;
    //if let Err(e) = users {
    //    println!("status {}", e.status().unwrap());
    //    Json(e.status().unwrap().to_string())
    //}
    //else {
    //    Json(users.expect("E.").token)
    //}

    //println!("status {:?}", users.status());
    //println!("is_status {:?}", users.is_status());
    //println!("is_timeout {:?}", users.is_timeout());
    match users {
        Ok(_ok) => {
            println!("status {}", _ok.status().as_str());
            Json(_ok.text().await.expect("E."))
        },
        Err(_error) => {
            println!("status {}", _error.status().unwrap().as_str());
            Json(_error.status().expect("REASON").to_string())
        },
    }
}
