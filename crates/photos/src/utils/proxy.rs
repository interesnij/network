use awc::{
    ClientResponse,
    Client, 
};
use actix_web::{
    dev, 
    get, 
    web, 
    HttpResponse,
    Responder,
};


pub trait IntoHttpResponse {
  fn into_http_response(self) -> HttpResponse;

  fn into_wrapped_http_response<E>(self) -> Result<HttpResponse, E>
  where
    Self: Sized,
  {
    Ok(self.into_http_response())
  }
}

impl IntoHttpResponse
  for ClientResponse<dev::Decompress<dev::Payload>>
{
  fn into_http_response(self) -> HttpResponse {
    let mut response = HttpResponse::build(self.status());

    self.headers().iter().for_each(|(k, v)| {
      response.set_header(k, v.clone());
    });
    response.streaming(self)
  }
}

pub mod util {
    use awc::{
        Client, 
    };
    use awc::error::SendRequestError;
    use crate::utils::proxy::IntoHttpResponse;
    use actix_web::{
        get, 
        web, 
        HttpResponse,
    };

    pub fn google_config(cfg: &mut web::ServiceConfig) {
        cfg.data(Client::default())
        .service(google_proxy);
    }


#[get("/{url:.*}")]
pub async fn google_proxy (
    param: web::Path<(String,)>,
    //(url, ): web::Path<(String,)>, 
    client: web::Data<Client>,
//) -> actix_web::Result<HttpResponse, SendRequestError> {
) -> Responder {
    let _url: String = param.0.clone();
    let url = format!("https://www.google.com/{}", _url);
    client
        .get(&url)
        .send()
        .await?
        .into_wrapped_http_response()
}
}