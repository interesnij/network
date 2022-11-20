//use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_user_owner_data,
    ErrorParams, CardUserJson, RegListData,
};
use crate::models::User;
use crate::errors::Error;


pub fn pages_urls(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    //config.route("/friends", web::get().to(user_friends_page));
    //config.route("/friends-online", web::get().to(user_friends_online_page));
    //config.route("/friends-common", web::get().to(user_friends_common_page));
    //config.route("/follows", web::get().to(user_follows_page));
    config.route("/all-users", web::get().to(all_users_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I users server.
            </p>
        </div>")
}

pub async fn all_users_page(req: HttpRequest) -> Result<Json<Vec<CardUserJson>>, Error> {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, _user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let _res = block(move || {
                let _limit: i64;
                let _offset: i64;
                if params.limit.is_some() {
                    _limit = params.limit.unwrap();
                }
                else {
                    _limit = 20;
                }
                if params.offset.is_some() {
                    _offset = params.offset.unwrap();
                }
                else {
                    _offset = 0;
                }
                User::get_users(_limit, _offset)
            }).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}
