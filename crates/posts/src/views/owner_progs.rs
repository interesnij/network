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
    DataListJson, RespListJson,
    DataNewPost, DataEditPost, RespPost,
    DataNewComment, DataEditComment, RespComment,
    ReactionData, JsonItemReactions,
};
use crate::models::{
    User, Community,
    PostList,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    config.route("/create_community/", web::post().to(create_community));
}

pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}
