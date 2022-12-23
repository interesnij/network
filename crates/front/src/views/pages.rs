use actix_web::{
    //HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use actix_identity::Identity;
use crate::utils::{APIURL, USERSURL};


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}




