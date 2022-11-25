use actix_web::{
    web,
    web::block,
    web::Json,
};
use serde::Serialize;
use crate::models::{
    User, Community,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;
use crate::utils::{
    AttachmentsJson,
};


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    config.route("/create_community/", web::post().to(create_community));

    config.route("/get_attach_post_lists/", web::get().to(get_attach_post_lists));
    config.route("/get_attach_posts/", web::get().to(get_attach_posts));
    config.route("/get_attach_post_comments/", web::get().to(get_attach_post_comments));
}

// веерное событие
pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
// веерное событие
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}
