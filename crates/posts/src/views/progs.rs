use crate::schema;
use actix_web::{
    HttpResponse,
    web,
    web::Json,
    web::block,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
    NewListValues,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{
    User, Community,
    PostList, Post, PostComment,

    NewUserJson,
};
use serde::{Deserialize, Serialize};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    
}

pub async fn create_user (
    data: Json<NewUserJson>,
    req: HttpRequest,
) -> Result<Json<User>, Error> {
    let _res = block(move ||
        CookieStat::create(data)
    ).await?;
    Ok(Json(_res))
}
