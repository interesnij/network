use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    APIURL, USERURL, RequestUser, TOKEN,
    get_first_load_page, get_default_image, is_authenticate,
    get_device_and_ajax, get_device_and_ajax_and_limit_offset,
    get_request_data, request_get,
};
use sailfish::TemplateOnce;
use actix_web_httpauth::extractors::bearer::BearerAuth;


pub fn users_urls(config: &mut web::ServiceConfig) {
    config.route("/users/all-users", web::get().to(all_users_page));
} 

#[derive(Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}
pub async fn all_users_page(_auth: BearerAuth, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax_and_limit_offset;
 
    let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(&req, 20);
    let object_list: Vec<CardUserJson>;
    let _object_list = request_get<CardUserJson> (
        USERURL.to_owned() 
        + &"/all-users?token=".to_string() + &TOKEN 
        + &"&limit=" + &limit 
        + &"&offset=" + &offset;
    )
    if _object_list.is_ok() {
        object_list = _object_list.expect("E.");
    }
    else {
        object_list = Vec::new();
    }

    if !_auth.token().is_empty() {
        let _request_user = get_request_data();
        
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