use crate::schema;
use actix_web::{
    HttpResponse,
    web,
    web::Json,
    web::block,
};
use crate::utils::{
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    establish_connection,
    NewListValues,
};
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
) -> Result<Json<User>, Error> {
    let _res = block(move ||
        User::create_user(data)
    ).await?;
    Ok(Json(_res))
}
