use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    web::block,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection, get_user,
    get_user_owner_data,
    ErrorParams, CardUserJson, RegListData,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use crate::models::{User, NewUser};
use crate::errors::Error;


pub fn load_urls(config: &mut web::ServiceConfig) {
    config.route("/load/friends/", web::get().to(friends_load));
    //config.route("/load/follows/", web::get().to(follows_load));
    //config.route("/load/include_users/", web::get().to(include_users_load));
    //config.route("/load/exclude_users/", web::get().to(exclude_users_load));
}


pub async fn friends_load(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<RegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id) = get_user_owner_data(params.token.clone(), params.user_id);
        if err.is_some() || (user_id == 0) {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let _res = block(move || {
                let _user = get_user(user_id).expect("E.");

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
                _user.get_friends(_limit, _offset)
            }).await;
            let resp = match _res {
                Ok(_ok) => _ok,
                Err(_) => Err(403),
            };
            HttpResponse::Ok().body(resp)
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}
