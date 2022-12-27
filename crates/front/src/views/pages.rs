use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    APIURL, USERURL, RequestUser,
    get_first_load_page, get_default_image, is_authenticate,
    get_device_and_ajax, get_device_and_ajax_and_limit_offset,
};
use crate::{AppState, UserState};
use sailfish::TemplateOnce;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
} 


pub async fn news_page (
    app_state: web::Data<AppState>,
    user_state: web::Data<UserState>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(app_state.clone(), &req, 20);
    let _request_user: RequestUser;
    _request_user = RequestUser {
        id:           *user_state.id.lock().unwrap(),
        name:         (*user_state.name.lock().unwrap()).to_string(),
        link:         (*user_state.link.lock().unwrap()).to_string(),
        s_avatar:     (*user_state.image.lock().unwrap()).to_string(),
        new_follows:  *user_state.new_follows.lock().unwrap(),
        new_messages: *user_state.new_messages.lock().unwrap(),
        new_notifies: *user_state.new_notifies.lock().unwrap(),
    }; 
    
    //let object_list: Vec<WallObject> = Vec::new();
    if is_ajax == 0 {
        get_first_load_page (
            false,
            false,
            "Новости".to_string(),
            "Трезвый.рус: новости".to_string(),
            "/".to_string(),
            get_default_image(), 
        ).await
    }
    else if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/empty_page.stpl")]
        struct Template {
            request_user:     RequestUser,
            //count:            usize,
            //next_page_number: i32,
            //object_list:      Vec<WallObject>,
            is_ajax:          u8,
        }
        let body = Template {
            request_user:     _request_user,
            //count:            count,
            //next_page_number: next_page_number,
            //object_list:      object_list,
            is_ajax:          is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/empty_page.stpl")]
        struct Template {
            request_user:     RequestUser,
            //count:            usize,
            //next_page_number: i32,
           // object_list:      Vec<WallObject>,
            is_ajax:          u8,
        }
        let body = Template {
            request_user:     _request_user,
            //count:            count,
            //next_page_number: next_page_number,
            //object_list:      object_list,
            is_ajax:          is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn index_page (
    app_state: web::Data<AppState>,
    user_state: web::Data<UserState>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(app_state.clone(), &req); 
    if is_authenticate(app_state.clone()) { 
        return news_page(app_state.clone(), user_state.clone(), req).await
    }
    else if is_ajax == 0 {
        get_first_load_page (
            false,
            false,
            "Трезвый.рус - вход".to_string(),
            "Трезвый.рус: Вход".to_string(),
            "/".to_string(),
            get_default_image(), 
        ).await
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/auth/auth.stpl")]
            struct DesctopAuthTemplate {
                is_ajax: u8,
                token:   String,
            }
            let body = DesctopAuthTemplate {
                is_ajax: is_ajax,
                token:   app_state.token.lock().unwrap().to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/auth/auth.stpl")]
            struct MobileAuthTemplate {
                is_ajax: u8,
            }
            let body = MobileAuthTemplate {is_ajax: is_ajax,}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}