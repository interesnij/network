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
    .configure(community_urls)
    //.configure(progs_urls)
    //.configure(manage_urls)
    ;
}
