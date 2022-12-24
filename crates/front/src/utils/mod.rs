use actix_web::{
    HttpRequest,
    web,
    error::InternalError,
    http::StatusCode,
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use actix_identity::Identity;
use crate::AppState;
use sailfish::TemplateOnce;

pub mod reqwest;
pub use self::{
    reqwest::*,
};

#[derive(Serialize, Deserialize)]
// это объект пользователя
pub struct User {
    pub id:       i32,
    pub name:     String,
    pub link:     String,
    pub s_avatar: String,
}

pub const APIURL: &str = "http:194.58.90.123:8000";
pub const USERURL: &str = "http:194.58.90.123:9001";


fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}

pub fn get_default_image() -> String {
    return "/static/images/hakew.png".to_string();
}

pub fn is_desctop(state: web::Data<AppState>, req: &HttpRequest) -> bool {
    let mut device = state.device.lock().unwrap();
    if *device == 1 {
        return true;
    }
    else if *device == 2 {
        return false;
    }
    else {
        let agent = get_content_type(req).unwrap();
        if agent.contains("Mobile") {
            *device = 2;
            return false;
        }
        *device = 1;
        return true;
    }
}

pub fn get_device_and_ajax(state: web::Data<AppState>, req: &HttpRequest) -> (bool, u8) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<u8>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = 0;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = params.ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
    }

    (is_desctop(state, req), is_ajax)
}

pub fn get_device_and_ajax_and_limit_offset (
    state: web::Data<AppState>, 
    req: &HttpRequest, 
    limit: i64
) -> (bool, u8, i64, i64) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax:   Option<u8>,
        pub limit:  Option<i64>,
        pub offset: Option<i64>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = 0;
    let mut _limit = 0;
    let mut _offset = 0;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = params.ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
        if params.limit.is_some() {
            _limit = params.limit.unwrap();
            if _limit < 0 || _limit > 100 {
                _limit = limit;
            }
        }
        else {
            _limit = limit;
        }
        if params.offset.is_some() {
            _offset = params.offset.unwrap();
        }
        else {
            _offset = 0;
        }
    }

    (is_desctop(state, req), is_ajax, _limit, _offset)
}

pub async fn get_first_load_page (    
    is_authenticate: bool,
    is_desctop:      bool,
    title:           String,
    description:     String,
    uri:             String,
    image:           String,
) -> actix_web::Result<HttpResponse> {
    if is_authenticate {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template {
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/first_load.stpl")]
            struct Template {
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/anon_first_load.stpl")]
            struct Template { 
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/anon_first_load.stpl")]
            struct Template {
                title:        String,
                description:  String,
                image:        String,
                uri:          String,
            }
            let body = Template {
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub fn get_ajax(req: &HttpRequest) -> u8 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<u8>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = 0;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = params.ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
    }

    is_ajax
}