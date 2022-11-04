use actix_web::web;
use crate::views::{
    pages,
    list_progs,
    item_progs,
    comment_progs,
    manager_progs,
    owner_progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(list_progs::list_urls)
    .configure(item_progs::item_urls)
    .configure(comment_progs::comment_urls)
    .configure(manager_progs::manager_urls)
    .configure(owner_progs::owner_urls)
    ;
}
