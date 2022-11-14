//use crate::schema;
use actix_web::{
    //HttpRequest,
    HttpResponse,
    Responder,
    web,
};
//use serde::Deserialize;

pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/friends/", web::get().to(user_friends_page));
    config.route("/friends-online/", web::get().to(user_friends_online_page));
    config.route("/friends-common/", web::get().to(user_friends_common_page));
    config.route("/follows/", web::get().to(user_follows_page));
    config.route("/all-users/", web::get().to(all_users_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I users server.
            </p>
        </div>")
}
