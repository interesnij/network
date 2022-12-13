use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Data};
use awc::http::StatusCode;
use clap::Parser;
use env_logger::Env;
//use futures_util::stream::TryStreamExt;
use log::{debug, info, warn};
use futures::TryStreamExt;


#[derive(Clone, Parser)]
pub struct Config {
    #[clap(short, long, default_value = "194.58.90.123")]
    pub address: String,
    #[clap(short, long, default_value = "9004")]
    pub port: u16,
    #[clap(short, long, default_value = "https://google.com")]
    pub to: String,
}

pub async fn proxy (
    req: HttpRequest,
    body: web::Payload,
    config: Data<Config>,
    http_client: Data<awc::Client>,
) -> impl Responder {
    let url = format!(
        "{to}{path}",
        to = config.to,
        path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
    );
    debug!("=> {url}");
    match http_client
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