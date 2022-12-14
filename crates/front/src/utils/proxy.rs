use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Data};
use awc::http::StatusCode;
use clap::Parser;
use env_logger::Env;
use futures::TryStreamExt;
use log::{debug, info, warn};
use serde::{Serialize, Deserialize};
use crate::utils::{
    USERURL, FRONTURL, FRONTPORT, 
};
use futures::StreamExt;
use std::str;
use std::borrow::BorrowMut;


#[derive(Clone, Parser)]
pub struct ConfigToUserServer {
    #[clap(short, long, default_value = FRONTURL)]             // наш ip
    pub address: String,
    #[clap(short, long, default_value = FRONTPORT)]                      // наш порт
    pub port: u16,
    #[clap(short, long, default_value = USERURL)] // адрес, на который будем перенаправлять запросы
    pub to: String,
}

pub async fn user_proxy (
    req:         HttpRequest,
    body:        web::Payload,
    path:        web::Path<String>,
    http_client: Data<awc::Client>,
) -> impl Responder {
    let _url = format!( 
        "{to}{path}",
        to = USERURL,
        path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
    );
    let url = _url.replace("/users", "");
    println!("url {}", url);
    return match http_client
        .request_from(&url, req.head())
        .send_stream(body)
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let mut resp_builder = HttpResponse::build(status);
            for header in resp.headers() {
                resp_builder.insert_header(header);
            }
            resp_builder.streaming(resp.into_stream())
        }
        Err(err) => {
            println!("err {}", err);
            HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
        }
    }
}