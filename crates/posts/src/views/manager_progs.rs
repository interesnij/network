use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::utils::{
    get_community,
    get_user,
    get_post_list,
    get_post,
    get_post_comment,
    get_community_permission,
    get_user_permission,
    ItemParams,
};
use crate::models::{
    User, Community,
    PostList,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;

pub fn manager_urls(config: &mut web::ServiceConfig) {
    //config.route("/create_user/", web::post().to(create_user));
    //config.route("/create_community/", web::post().to(create_community));
}
