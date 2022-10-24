use actix_web::{
    web,
    HttpRequest,
    Responder,
    HttpResponse,
    get,
    HttpResponseBuilder,
    http::StatusCode
};
use actix_web_httpauth::{extractors::bearer::BearerAuth};
use chrono::Utc;
use crate::{
    AppState,
    config::crypto::{unwrap_jwt, verify_jwt},
    models::{UserDetail, User}
};

pub fn user_scope() -> actix_web::Scope{
    web::scope("/users")
        .service(user_profile)
}

#[get("/{user_id}")]
async fn user_profile(_state: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder{
    println!("user_profile!");
    use std::str;

    let user: Result<User, _> = _state.rb.fetch_by_column("id", *user_id).await;

    match user {
        Ok(user_data) => {
            let body = serde_json::to_string(&UserDetail {
                id: user_data.id,
                first_name: user_data.first_name,
                last_name: user_data.last_name,
                types: user_data.types,
                gender: user_data.gender,
                device: user_data.device,
                language: user_data.language,
                perm: user_data.perm,
                link: user_data.link,
                city: user_data.city,
                status: user_data.status,
                image: user_data.b_avatar,
                birthday: user_data.birthday.to_string(),
                last_activity: user_data.last_activity.to_string(),
            }).unwrap();

            HttpResponse::Ok().body(body)
        },
        Err(_) => {
            HttpResponse::Ok().body("user not found")
        },
    }
}
