use actix_web::web;
use crate::views::{
    pages,
    load_pages,
    profile,
    progs,
    manager_progs,
    auth_progs,
    settings,
    auth,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(profile::profile_urls)
    .configure(pages::pages_urls)
    .configure(load_pages::load_urls)
    .configure(progs::progs_urls)
    .configure(manager_progs::manager_urls)
    .configure(auth::auth_urls)
    .configure(settings::settings_urls)
    ;
}
