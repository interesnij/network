use actix_web::web;

use crate::views::{
    pages,
    progs,
    owner_progs,
    user_progs,
    manager_progs,
    settings,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_urls)
    .configure(progs::progs_urls)
    .configure(user_progs::user_urls)
    .configure(owner_progs::owner_urls)
    .configure(manager_progs::manager_urls)
    .configure(settings::settings_urls)
    ;
}
