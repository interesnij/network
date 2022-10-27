use actix_web::web;

use crate::views::{
    pages_routes,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages_routes)
    ;
}
