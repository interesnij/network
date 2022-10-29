use actix_web::web;

use crate::views::{
    //pages,
    progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    //.configure(pages_urls)
    .configure(progs_urls)
    ;
}
