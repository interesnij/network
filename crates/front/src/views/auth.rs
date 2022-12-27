use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
    error::InternalError,
    http::StatusCode,
    ResponseError,
};
use crate::errors::Error;
use crate::utils::{
    APIURL, USERURL, TOKEN,
    get_first_load_page, get_default_image,
    get_device_and_ajax, request_post,
    remove_token, is_authenticate,
};
use crate::{AppState, UserState};
use sailfish::TemplateOnce;
use crate::views::index_page;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub fn auth_urls(config: &mut web::ServiceConfig) {
    config.route("/signup", web::get().to(mobile_signup));
    config.route("/login", web::get().to(mobile_login));
    config.route("/phone_send", web::post().to(phone_send));
    config.route("/phone_verify", web::post().to(phone_verify));
    //config.route("/signup", web::post().to(process_signup));
    //config.route("/login", web::post().to(login));
    //config.route("/logout", web::get().to(logout));
}  

pub async fn logout ( 
    app_state: web::Data<AppState>,
    user_state: web::Data<UserState>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    remove_token(app_state.clone());
    index_page(app_state, user_state, req).await
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneParams {
    pub token: String,
    pub phone: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RespParams {
    pub resp: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorParams {
    pub error: u16,
}

pub async fn phone_send (
    app_state: web::Data<AppState>,
    mut data: Json<PhoneParams>,
) -> Result<Json<RespParams>, Error> { 
    let res = request_post::<PhoneParams, RespParams> (
        USERURL.to_owned() + &"/phone_send".to_string(),
        &*data.borrow_mut(),
        //&data,
        app_state,
    ).await;
    println!("res {:?}", res);

    match res {
        Ok(ok) => Ok(Json(ok)),
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PhoneCodeParams {
    pub token: String,
    pub phone: String,
    pub code:  String,
}
pub async fn phone_verify (
    app_state: web::Data<AppState>,
    mut data: Json<PhoneCodeParams>,
) -> Result<Json<RespParams>, Error> { 
    let res = request_post::<PhoneCodeParams, RespParams> (
        USERURL.to_owned() + &"/phone_verify".to_string(),
        //&*data.borrow_mut(),
        &data,
        app_state,
    ).await;

    match res {
        Ok(ok) => Ok(Json(ok)),
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}

pub async fn mobile_signup(state: web::Data<AppState>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_ajax;

    let is_ajax = get_ajax(&req);
    if is_authenticate(state) { 
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else if is_ajax == 0 {
        get_first_load_page (
            false,
            false,
            "Трезвый.рус - регистрация".to_string(),
            "Трезвый.рус: Регистрация".to_string(),
            "/signup".to_string(),
            get_default_image(), 
        ).await
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/auth/signup.stpl")]
        struct Template {
            is_ajax: u8,
        }
        
        let body = Template {
            is_ajax: is_ajax,
        } 
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn mobile_login(state: web::Data<AppState>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_ajax;

    let is_ajax = get_ajax(&req);
    if is_authenticate(state) { 
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else if is_ajax == 0 {
        get_first_load_page (
            false,
            false,
            "Трезвый.рус - вход".to_string(),
            "Трезвый.рус: Вход".to_string(),
            "/login".to_string(),
            get_default_image(), 
        ).await
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/auth/auth.stpl")]
        struct Template {
            is_ajax: u8,
        }
        
        let body = Template {
            is_ajax: is_ajax,
        } 
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}