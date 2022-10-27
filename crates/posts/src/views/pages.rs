use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    get_community,
    get_user,
    get_post_list,
    get_post,
    get_post_comment,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    establish_connection,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, PostList, Post, PostComment, Community};
use crate::diesel::RunQueryDsl;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/add_user_list/", web::get().to(add_user_list_page));
    config.route("/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    config.route("/add_community_list//{id}", web::get().to(add_community_list_page));
    config.route("/edit_community_list/{id}/", web::get().to(edit_community_list_page));
    config.route("/edit_post/{id}/", web::get().to(edit_post_page));

    config.route("/load_list/{list_id}/", web::get().to(load_list_page));
    config.route("/load_post/{id}/", web::get().to(load_post_page));
    config.route("/load_comments/{id}/", web::get().to(load_comments_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I'm posts server.
            </p>
        </div>")
}
