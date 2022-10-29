use actix_web::web;

use crate::views::{
    pages,
    //progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    //.configure(progs::progs_routes)
    ;
}
