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
    APIURL, USERURL, User, UserState,
    get_first_load_page, get_default_image,
    get_device_and_ajax, get_device_and_ajax_and_limit_offset,
};
use crate::AppState;
use sailfish::TemplateOnce;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/mob_register", web::get().to(mobile_signup));
}


pub async fn news_page (
    token: String, 
    data: web::Data<Mutex<UserState>>, 
    req: HttpRequest 
) -> actix_web::Result<HttpResponse> {
    let mut data = data.lock().unwrap();
    let (is_desctop, is_ajax, limit, offset) = get_device_and_ajax_and_limit_offset(data.clone(), &req, 20);
    let _request_user: User;

    _request_user = User {
        id:           data.user_id,
        name:         data.user_name.to_string(),
        link:         data.user_link.to_string(),
        s_avatar:     data.user_image.to_string(),
        new_follows:  data.new_follows,
        new_messages: data.new_messages,
        new_notifies: data.new_notifies,
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
            request_user:     User,
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
            request_user:     User,
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
    data: web::Data<Mutex<UserState>>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(data.clone(), &req);
    if ide.is_some() {
        return news_page(ide.unwrap().id().unwrap(), data.clone(), req).await
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
            "/mob_register".to_string(),
            get_default_image(), 
        ).await
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/auth/signup.stpl")]
        struct NobileSignupTemplate {
            is_ajax: u8,
        }
        
        let body = NobileSignupTemplate {
            is_ajax: is_ajax,
        } 
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}




