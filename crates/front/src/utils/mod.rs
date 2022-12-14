use actix_web::{
    HttpRequest,
    web,
    error::InternalError,
    http::StatusCode,
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sailfish::TemplateOnce;

pub mod reqwest;
//pub mod proxy;
pub use self::{
    reqwest::*,
    //proxy::*,
};

pub const FRONTURL: &str = "http://194.58.90.123";
pub const FRONTPORT: &str = "8100";
pub const TOKEN: &str = "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";
pub const APIURL: &str = "http://194.58.90.123:8000";
pub const USERURL: &str = "http://194.58.90.123:9001";
pub const COMMUNITYURL: &str = "http://194.58.90.123:9002";
pub const POSTURL: &str = "http://194.58.90.123:9002";


#[derive(Serialize, Deserialize)]
// это объект авторизованного пользователя
pub struct RequestUser {
    pub id:           String,
    pub name:         String,
    pub link:         String,
    pub s_avatar:     String,
    pub new_follows:  String,
    pub new_messages: String,
    pub new_notifies: String,
}

pub fn get_request_data() -> RequestUser {
    let id = web_local_storage_api::get_item("id").expect("E.").unwrap();
    let name = web_local_storage_api::get_item("name").expect("E.").unwrap();
    let link = web_local_storage_api::get_item("link").expect("E.").unwrap();
    let s_avatar = web_local_storage_api::get_item("s_avatar").expect("E.").unwrap();

    let new_follows: String;
    let new_messages: String;
    let new_notifies: String;
    let _new_follows = web_local_storage_api::get_item("new_follows").expect("E.");
    let _new_messages = web_local_storage_api::get_item("new_messages").expect("E.");
    let _new_notifies = web_local_storage_api::get_item("new_notifies").expect("E.");

    if _new_follows.is_some() {
        new_follows = _new_follows.unwrap();
    }
    else {
        new_follows = String::new();
    }
    if _new_messages.is_some() {
        new_messages = _new_messages.unwrap();
    }
    else {
        new_messages = String::new();
    }
    if _new_notifies.is_some() {
        new_notifies = _new_notifies.unwrap();
    }
    else {
        new_notifies = String::new();
    }

    return RequestUser {
        id:           id,
        name:         name,
        link:         link,
        s_avatar:     s_avatar,
        new_follows:  new_follows,
        new_messages: new_messages,
        new_notifies: new_notifies,
    }
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}

pub fn get_default_image() -> String {
    return "/static/images/hakew.png".to_string();
}

pub fn is_desctop(req: &HttpRequest) -> bool {
    if get_content_type(req).unwrap().contains("Mobile") {
        return false;
    };
    return true;
}

pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, u8) {
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

    (is_desctop(req), is_ajax)
}

pub fn get_device_and_ajax_and_limit_offset (
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

    (is_desctop(req), is_ajax, _limit, _offset)
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