use actix_web::cookie::time::{
    Duration,
    OffsetDateTime
};
use actix_web::{
    web,
    get,
    post,
    HttpRequest,
    HttpResponse,
    Responder
};
use actix_web::{
    http::header::HeaderName,
    http::header::HeaderValue,
    http::header
};

use crate::handlers::HandlersError;
use crate::models::{UserSignup, UserLogin, UserToken};
use crate::repositories::user_repository;
use crate::AppState;
use crate::config::crypto::{Claims, gen_jwt};


pub fn auth_scope() -> actix_web::Scope{
    web::scope("/progs")
        .service(signup)
        .service(login)
        //.service(info)
}

#[post("/signup")]
async fn signup (
    _req: HttpRequest,
    _data: web::Json<UserSignup>,
    _state: web::Data<AppState>
) -> impl Responder {
    let user_data: UserSignup = _data.into_inner();
    match user_repository::create(&user_data, _state.pg.as_ref()).await {
        Some(_) => {
            HttpResponse::Ok().finish()
        },
        None => {
            HttpResponse::Conflict().finish()
        },
    }
}

#[post("/login")]
async fn login (
    _req:HttpRequest,
    _data: web::Json<UserLogin>,
    _state: web::Data<AppState>
) -> impl Responder {
    log::info!("Try login: {}, {}", _data.phone, _data.password);
    let user = user_repository::find_by_phone(&_data.phone, _state.pg.as_ref()).await;

    if let None = user {
        let msg = format!("User not found by phone {}", _data.phone);
        return HttpResponse::BadRequest().body(HandlersError::new_str(msg))
    }

    let user = user.unwrap();

    if bcrypt::verify(_data.password.as_str(), user.password.as_str()).unwrap() {
            let token = gen_jwt(user.id, _state.key.as_ref()).await;

            match token {
                Ok(token_str) => {
                    let body = serde_json::to_string(&UserToken {
                        token: token_str,
                    }).unwrap();

                    HttpResponse::Ok().body(body)
                },
                Err(err) => {
                    log::error!("Failed create token: {}", err);
                    let msg = format!("Failed create token {}", err);
                    HttpResponse::InternalServerError().body(HandlersError::new_str(msg))
                }
            }
    } else {
        log::info!("Bad password for user {}", user.last_name);
        let msg = format!("Bad password for user {}", user.last_name);
        HttpResponse::BadRequest().body(HandlersError::new_str(msg))
    }
}
