//pub mod profile;
//pub mod progs;
//pub mod settings;
pub mod pages;
//pub mod load;

use actix_web::web::ServiceConfig;

pub use self::{
    //profile::*,
    //progs::*,
    //settings::*,
    pages::*,
    //load::*,

};

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg
    //.configure(pages::profile_urls)
    .configure(pages::pages_urls)
    //.configure(pages::progs_urls)
    //.configure(pages::load_urls)
    //.configure(pages::settings_urls)
    ;
}
