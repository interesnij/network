use actix_web::{
    Responder,
    body::BoxBody,
    HttpResponse,
    http::header::ContentType
};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct HandlersError {
    pub msg: String,
}

impl HandlersError {
    pub fn new_str(msg: String)-> String {
        serde_json::to_string(&HandlersError {
            msg: msg.to_owned()
        }).unwrap()
    }
}

//pub mod user_handlers;
pub mod auth_handlers;
