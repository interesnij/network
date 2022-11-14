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


pub fn settings_urls(config: &mut web::ServiceConfig) {
    config.route("/followings/", web::get().to(followings_page));
    config.route("/blacklist/", web::get().to(blacklist_page));
    config.route("/settings/edit_link/", web::get().to(edit_link_page));
    config.route("/settings/edit_name/", web::get().to(edit_name_page));
    config.route("/settings/edit_phone/", web::get().to(edit_phone_page));
    config.route("/settings/remove_profile/", web::get().to(remove_profile_page));

    config.route("/settings/change_phone_send/", web::post().to(change_phone_send));
    config.route("/settings/change_phone_verify/", web::post().to(change_phone_verify));
    config.route("/settings/edit_link/", web::post().to(edit_link));
    config.route("/settings/edit_name/", web::post().to(edit_name));
    config.route("/settings/edit_password/", web::post().to(edit_password));
    config.route("/settings/edit_phone/", web::post().to(edit_phone));
    config.route("/settings/remove_profile/", web::post().to(remove_profile));
}
