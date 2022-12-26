use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;
use crate::AppState;
use actix_web::{HttpRequest, HttpMessage, web::Json};


struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}


pub fn get_token(state: web::Data<AppState>)-> Option<String> {
    let token = state.token.lock().unwrap().to_string();
    if !token.is_empty() {
        return Some(token);
    }
    let token = web_local_storage_api::get_item("token");
    if token.is_some() {
        return token;
    }
    else {
        return None;
    }
}

pub fn is_authenticate(state: web::Data<AppState>)-> Option<String> {
    return !state.token.lock().unwrap().to_string().is_empty() || web_local_storage_api::get_item("token").is_some();
}

pub fn set_token(token: String, state: web::Data<AppState>) {
    web_local_storage_api::set_item("token", token.clone());
    let token = state.token.lock().unwrap().to_string();
    token = token.clone();
}

pub fn remove_token(state: web::Data<AppState>){
    web_local_storage_api::remove_item("token");
    let token = state.token.lock().unwrap().to_string();
    token = "".to_string();
}


async fn request<U, T> (
    url: String, 
    method: reqwest::Method, 
    body: &U,
    state: web::Data<AppState>,
) -> Result<T, u16>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug ,
{ 
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");


    if let Some(token) = get_token(state){
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

pub async fn request_delete<T>(url: String, state: web::Data<AppState>,) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::DELETE, &(), state).await
}

/// Get request
pub async fn request_get<T>(url: String, state: web::Data<AppState>,) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::GET, &(), state).await
}

/// Post request with a body
pub async fn request_post<U, T>(url: String, body: &U, state: web::Data<AppState>,) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug, 
{
    request(url, reqwest::Method::POST, body, state).await
}

/// Put request with a body
pub async fn request_put<U, T>(url: String, body: &U, state: web::Data<AppState>,) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::PUT, body, state).await
}