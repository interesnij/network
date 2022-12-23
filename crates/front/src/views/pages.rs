use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use actix_identity::Identity;
use crate::utils::{APIURL, USERURL};
use crate::AppState;
use sailfish::TemplateOnce;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    config.route("/mob_register/", web::get().to(mobile_signup));
}



pub async fn mobile_signup(ide: Option<Identity>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_ajax;
    if ide.is_some() { 
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    } 
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/auth/signup.stpl")]
        struct NobileSignupTemplate {
            is_ajax: u8,
        }
        
        let is_ajax = get_ajax(&req);
        let body = NobileSignupTemplate {
            is_ajax: is_ajax,
        } 
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}




