use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Data};
use awc::http::StatusCode;
use clap::Parser;
use env_logger::Env;
use futures::TryStreamExt;
use log::{debug, info, warn};
use serde::{Serialize, Deserialize};
use crate::utils::{
    USERURL,
};
use crate::models::{
    Photo, PhotoList,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::str;
use std::borrow::BorrowMut;


#[derive(Clone, Parser)]
pub struct ConfigToUserServer {
    #[clap(short, long, default_value = "194.58.90.123")]             // наш ip
    pub address: String,
    #[clap(short, long, default_value = "8100")]                      // наш порт
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
    let url = format!(
        "{to}{path}",
        to = USERURL,
        path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
    );
    debug!("=> {url}");
    return match http_client
        .request_from(&url, req.head())
        .send_stream(body)
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            debug!("<= [{status}] {url}", status = status.as_u16());
            let mut resp_builder = HttpResponse::build(status);
            for header in resp.headers() {
                resp_builder.insert_header(header);
            }
            resp_builder.streaming(resp.into_stream())
        }
        Err(err) => {
            warn!("{url}: {err:?}");
            HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
        }
    }
}