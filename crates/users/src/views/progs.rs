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


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/progs/block/", web::post().to(user_block));
    config.route("/progs/unblock/", web::post().to(user_unblock));
    config.route("/progs/friend/", web::post().to(user_friend));
    config.route("/progs/unfriend/", web::post().to(user_unfriend));
    config.route("/progs/follow/", web::post().to(user_follow));
    config.route("/progs/follow_view/", web::post().to(user_follow_view));
    config.route("/progs/unfollow/", web::post().to(user_unfollow));
}
