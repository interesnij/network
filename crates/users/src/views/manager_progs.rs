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
    config.route("/load/follows/", web::get().to(follows_load));
    config.route("/load/include_friends/", web::get().to(include_friends_load));
    config.route("/load/exclude_friends/", web::get().to(exclude_friends_load));
    config.route("/load/include_follows/", web::get().to(include_follows_load));
    config.route("/load/exclude_follows/", web::get().to(exclude_follows_load));
} 
