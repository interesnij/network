use actix_web::{
    //HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use actix_identity::Identity;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}

pub async fn index_page(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        println!("Welcome! {}", user.id().unwrap());
    } else {
        println!("Welcome Anonymous!".to_owned());
    }
    HttpResponse::Ok().body(
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I front server.
            </p>
        </div>")
}
