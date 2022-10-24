//use diesel::prelude::*;
//use crate::schema;
use actix_web::{
    //HttpRequest,
    HttpResponse,
    Responder,
    web,
};
//use serde::Deserialize;
//use crate::utils::{
//    establish_connection,
//};

pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body(
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I communities server.
            </p>
        </div>")
}
