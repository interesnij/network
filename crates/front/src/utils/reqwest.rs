use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;
use crate::AppState;
use actix_identity::Identity;
use actix_web::{HttpRequest, HttpMessage, web::Json};



struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}


pub fn get_token(ide: Option<Identity>)-> Option<String>{
    if let Some(user) = ide {
        return Some(user.id().unwrap());
    } else {
        return None;
    }
}

pub fn set_token(request: HttpRequest, token: String) {
    Identity::login(&request.extensions(), token).unwrap();
}

pub fn remove_token(ide: Identity){
    ide.logout();
}


async fn request<U, T> (
    url: String, 
    method: reqwest::Method, 
    body: &U,
    ide: Option<Identity>,
) -> Result<T, u16>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug ,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, format!("http://localhost:8080{}", url))
        .header("Content-Type", "application/json");


    if let Some(token) = get_token(ide){
        req = req.bearer_auth(token);
    }

    if allow_body{ 
        req = req.json(body);
    }

    log::info!("Request: {:?}", req);
    let res_resp = req.send().await;
    log::info!("Response: {:?}", res_resp);

    match res_resp {
        Ok(resp) => {

        match resp.status().is_success(){
            true => {
                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        log::info!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => Err(resp.status().as_u16())
        }
    },
        Err(err) => {
            Err(0)
        }
    }
}

pub async fn request_delete<T>(url: String, ide: Identity) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::DELETE, &(), Some(ide)).await
}

/// Get request
pub async fn request_get<T>(url: String, ide: Identity) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::GET, &(), Some(ide)).await
}

/// Post request with a body
pub async fn request_post<U, T>(url: String, body: &U, ide: Identity) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::POST, body, Some(ide)).await
}

/// Put request with a body
pub async fn request_put<U, T>(url: String, body: &U, ide: Identity) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::PUT, body, Some(ide)).await
}
