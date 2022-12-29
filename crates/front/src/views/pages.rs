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
    get_request_data,
};
use sailfish::TemplateOnce;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
} 


pub async fn news_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(&req, 20);
    let _request_user: RequestUser;
    _request_user = get_request_data(); 
    
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
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req); 
    use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
    use actix_web::http::header::Header;

    let auth = Authorization::<Bearer>::parse(&req);
    match auth {
        Ok(_) => println!("auth ok!"),
        Err(_) => println!("not auth!"),
    } 

    if is_authenticate() { 
        return news_page(req).await
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
                is_ajax:   u8,
                ogg_image: String,
            }
            let body = DesctopAuthTemplate {
                is_ajax:   is_ajax,
                ogg_image: get_default_image(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/auth/auth.stpl")]
            struct MobileAuthTemplate {
                is_ajax:   u8,
                ogg_image: String,
            }
            let body = MobileAuthTemplate {
                is_ajax:   is_ajax,
                ogg_image: get_default_image(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}