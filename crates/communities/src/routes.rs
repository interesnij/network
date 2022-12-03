use actix_web::web;

use crate::views::{
    pages,
    owner_progs,
    manager_progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_urls)
    .configure(manager_progs::owner_urls)
    .configure(pagmanager_progses::manager_urls)
    ;
}
