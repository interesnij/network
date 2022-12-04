use actix_web::{
    web,
    web::block,
    web::Json,
};
use serde::Serialize;
use crate::models::{
    User, Community,
    NewUserJson, NewCommunityJson,
    PostList, Post, PostComment,
};
use crate::errors::Error;
use crate::utils::{
    AttachmentsJson, AttachPostCommentResp,
    AttachPostResp, AttachPostListResp,
};
use serde::Deserialize;


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_service_user/", web::post().to(create_service_user));
    config.route("/create_service_community/", web::post().to(create_service_community));

    config.route("/get_attach_post_lists/", web::get().to(get_attach_post_lists));
    config.route("/get_attach_posts/", web::get().to(get_attach_posts));
    config.route("/get_attach_post_comments/", web::get().to(get_attach_post_comments));
}

// создаем пользователя сервиса, создателя списков, постов, комментов
pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
// создаем сообщество сервиса, создателя списков, постов, комментов
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}

#[derive(Deserialize)]
pub struct VecIdsParams {
    pub ids: Vec<i32>,
}

// выдаем данные для закрепления списков записей в других сервисах
pub async fn get_attach_post_lists(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostListResp>>, Error> {
    let _res = block(move || PostList::get_lists_for_attach(data.ids.clone())).await?;
    Ok(Json(_res))
}
// выдаем данные для закрепления записей в других сервисах
pub async fn get_attach_posts(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostResp>>, Error> {
    let _res = block(move || Post::get_posts_for_attach(data.ids.clone())).await?;
    Ok(Json(_res))
}
// выдаем данные для закрепления комментов в других сервисах
pub async fn get_attach_post_comments(data: Json<VecIdsParams>) -> Result<Json<Vec<AttachPostCommentResp>>, Error> {
    let _res = block(move || PostComment::get_comments_for_attach(data.ids.clone())).await?;
    Ok(Json(_res)) 
}