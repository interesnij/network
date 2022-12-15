use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Data};
use awc::http::StatusCode;
use clap::Parser;
use env_logger::Env;
use futures::TryStreamExt;


#[derive(Clone, Parser)]
pub struct ConfigToStaticServer {
    #[clap(short, long, default_value = "194.58.90.123")]             // наш ip
    pub address: String,
    #[clap(short, long, default_value = "9004")]                      // наш порт
    pub port: u16,
    #[clap(short, long)] //, default_value = "http://194.58.90.123:9050")] // адрес, на который будем перенаправлять запросы
    pub to: String,
}

pub async fn proxy_to_static_server (
    req: HttpRequest,
    body: web::Payload,
    config: Data<ConfigToStaticServer>,
    http_client: Data<awc::Client>,
) -> impl Responder {
    let url = format!(
        "{to}{path}",
        to = config.to,
        path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("")
    );

    match http_client
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
            HttpResponse::build(StatusCode::BAD_GATEWAY).body("Bad Gateway")
        }
    }
}