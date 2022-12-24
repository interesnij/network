use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use actix_identity::Identity;
use crate::utils::{
    APIURL, USERURL,
    get_first_load_page, get_default_image,
    get_device_and_ajax,
};
use crate::AppState;
use sailfish::TemplateOnce;


pub fn auth_urls(config: &mut web::ServiceConfig) {
    config.route("/signup", web::get().to(mobile_signup));
    config.route("/login", web::get().to(mobile_login));
} 

pub async fn mobile_signup(ide: Option<Identity>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_ajax;

    let is_ajax = get_ajax(&req);
    if ide.is_some() { 
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

pub async fn mobile_login(ide: Option<Identity>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_ajax;

    let is_ajax = get_ajax(&req);
    if ide.is_some() { 
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