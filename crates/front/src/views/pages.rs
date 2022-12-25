use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use cookie::Cookie;
use actix_identity::Identity;
use crate::utils::{
    APIURL, USERURL, RequestUser,
    get_first_load_page, get_default_image,
    get_device_and_ajax, get_device_and_ajax_and_limit_offset,
};
use crate::AppState;
use sailfish::TemplateOnce;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
} 


pub async fn news_page (
    state: web::Data<AppState>, 
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(state.clone(), &req, 20);
    let _request_user: RequestUser;
    _request_user = RequestUser {
        id:           *state.user_id.lock().unwrap(),
        name:         (*state.user_name.lock().unwrap()).to_string(),
        link:         (*state.user_link.lock().unwrap()).to_string(),
        s_avatar:     (*state.user_image.lock().unwrap()).to_string(),
        new_follows:  *state.new_follows.lock().unwrap(),
        new_messages: *state.new_messages.lock().unwrap(),
        new_notifies: *state.new_notifies.lock().unwrap(),
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
    ide: Option<Identity>, 
    state: web::Data<AppState>, 
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(state.clone(), &req);
    if ide.is_some() {
        return news_page(state.clone(), req).await
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
        let cookie = Cookie::new("name", "1");
        let secure_cookie = Cookie::build("secure_name", "1")
            .domain("194.58.90.123:8100")
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        
            for header in req.headers().into_iter() {
                if header.0 == "cookie" {
                    let str_cookie = header.1.to_str().unwrap();
                    let _cookie: Vec<&str> = str_cookie.split(";").collect();
                    for c in _cookie.iter() {
                        let split_c: Vec<&str> = c.split("=").collect();
                        println!("name {:?}", split_c[0].trim());
                        println!("value {:?}", split_c[1]);
                    }
                }
            };
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/auth/auth.stpl")]
            struct DesctopAuthTemplate {
                is_ajax: u8,
            }
            let body = DesctopAuthTemplate {
                is_ajax: is_ajax,
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