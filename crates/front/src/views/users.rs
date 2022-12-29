use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    APIURL, USERURL, RequestUser, TOKEN,
    get_first_load_page, get_default_image, is_authenticate,
    get_device_and_ajax, get_device_and_ajax_and_limit_offset,
    get_request_data, request_get,
};
use sailfish::TemplateOnce;


pub fn users_urls(config: &mut web::ServiceConfig) {
    config.route("/users/all-users", web::get().to(all_users_page));
} 

#[derive(Debug, Deserialize)]
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}
pub async fn all_users_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax_and_limit_offset, get_ajax};
 
    //let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(&req, 20);
    let is_desctop = true;
    let is_ajax = get_ajax(&req);
    let limit = 20;
    let offset = 0;
    let object_list: Vec<CardUserJson>; 
    if is_ajax == 0 {
        get_first_load_page (
            is_authenticate(),
            is_desctop,
            "Все пользователи".to_string(),
            "Трезвый.рус: Все пользователи".to_string(),
            "/users/all-users".to_string(),
            get_default_image(), 
        ).await
    }
    else if is_authenticate() {
        let _request_user = get_request_data();
        let _object_list = request_get::<Vec<CardUserJson>> (
            USERURL.to_owned() 
            + &"/all-users?token=".to_string() + &TOKEN
            + &"&limit=" + &limit.to_string()
            + &"&offset=" + &offset.to_string(),
            true
        ).await;
        if _object_list.is_ok() {
            object_list = _object_list.expect("E.");
        }
        else {
            object_list = Vec::new();
        }
        
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/all_users.stpl")]
            struct Template {
                request_user: RequestUser,
                object_list:  Vec<CardUserJson>,
                is_ajax:      u8,
            }

            let body = Template {
                request_user: _request_user,
                object_list:  object_list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/all_users.stpl")]
            struct Template {
                request_user: RequestUser,
                object_list:  Vec<CardUserJson>,
                is_ajax:      u8,
            }

            let body = Template {
                request_user: _request_user,
                object_list:  object_list,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    } else {
        let _object_list = request_get::<Vec<CardUserJson>> (
            USERURL.to_owned() 
            + &"/all-users?token=".to_string() + &TOKEN
            + &"&limit=" + &limit.to_string()
            + &"&offset=" + &offset.to_string(),
            false
        ).await;
        if _object_list.is_ok() {
            object_list = _object_list.expect("E.");
        }
        else {
            object_list = Vec::new();
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/users/lists/anon_all_users.stpl")]
            struct Template {
                object_list: Vec<CardUserJson>,
                is_ajax:     u8,
            }
            let body = Template {
                object_list: object_list,
                is_ajax:     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/users/lists/anon_all_users.stpl")]
            struct Template {
                object_list: Vec<CardUserJson>,
                is_ajax:     u8,
            }
            let body = Template {
                object_list: object_list,
                is_ajax:     is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}