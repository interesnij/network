use actix_web::web;

use crate::views::{
    pages,
    owner_progs,
    manager_progs,
    settings_progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_urls)
    .configure(owner_progs::owner_urls)
    .configure(manager_progs::manager_urls)
    .configure(settings_progs::settings_urls)
    ;
}
