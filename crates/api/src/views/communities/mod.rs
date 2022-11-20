pub mod pages;
//pub mod progs;
//pub mod manage;
use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
    //progs::*,
    //manage::*,
};

pub fn community_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages::pages_urls)
    //.configure(pages::progs_urls)
    //.configure(pages::manage_urls)
    ;
}
