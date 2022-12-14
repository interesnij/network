use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    get_error_response,
    RegListData,
    CardUserJson,
    ErrorParams,
    USERS_URL,
};
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    //config.route("/friends", web::get().to(user_friends_page));
    //config.route("/friends-online", web::get().to(user_friends_online_page));
    //config.route("/friends-common", web::get().to(user_friends_common_page));
    //config.route("/follows", web::get().to(user_follows_page));
    config.route("/all-users", web::get().to(all_users_page));
}

pub async fn all_users_page(req: HttpRequest) -> Json<String> {
    let users = reqwest::get(USERS_URL.to_owned() + &"/all-users?".to_string() + &req.query_string()).await;
    match users {
        Ok(_ok) => {
            println!("status {}", _ok.status().as_str());
            Json(_ok.text().await.expect("E."))
        },
        Err(_error) => {
            Json(get_error_response(_error))
        },
    }
}
