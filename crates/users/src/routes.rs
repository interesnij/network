use actix_web::web;
use crate::views::{
    pages,
    load_pages,
    //profile,
    //progs,
    //manager_progs,
    //settings,
    auth,
    owner_progs,
};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    //.configure(profile::profile_urls)
    .configure(pages::pages_urls)
    .configure(load_pages::load_urls)
    .configure(owner_progs::owner_urls)
    //.configure(progs::progs_urls)
    //.configure(manager_progs::manager_urls)
    .configure(auth::auth_urls)
    //.configure(settings::settings_urls)
    ;
}
