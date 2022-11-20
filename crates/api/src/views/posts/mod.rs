pub mod pages;
//pub mod progs;
use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
    //progs::*,
};

pub fn post_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages_urls)
    //.configure(progs_urls)
    ;
}
