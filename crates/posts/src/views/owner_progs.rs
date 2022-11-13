use actix_web::{
    web,
    web::block,
    web::Json,
};
use crate::models::{
    User, Community,
    NewUserJson, NewCommunityJson,
};
use crate::errors::Error;
//use crate::utils::{
//    get_user_owner_data,
//    ErrorParams,
//};


pub fn owner_urls(config: &mut web::ServiceConfig) {
    config.route("/create_user/", web::post().to(create_user));
    config.route("/create_community/", web::post().to(create_community));
}

// веерное событие
pub async fn create_user(data: Json<NewUserJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || User::create_user(data)).await?;
    Ok(Json(_res))
}
// веерное событие
pub async fn create_community(data: Json<NewCommunityJson>) -> Result<Json<bool>, Error> {
    let _res = block(move || Community::create_community(data)).await?;
    Ok(Json(_res))
}
