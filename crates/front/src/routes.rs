use actix_web::web;
use crate::views::{
    pages_routes,
    //community_routes,
    //post_routes,
};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages_routes)
    //.configure(community_routes)
    //.configure(post_routes)
    ;
}
