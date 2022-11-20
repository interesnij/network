use actix_web::web;
use crate::views::{
    user_routes,
    community_routes,
    post_routes,
};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(user_routes)
    .configure(community_routes)
    .configure(post_routes)
    ;
}
