use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;
use actix_web::{HttpRequest, HttpMessage, web::Json};


struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}


pub fn get_token()-> Option<String> {
    let token = web_local_storage_api::get_item("token").expect("E.");
    if token.is_some() {
        return token;
    }
    else {
        return None;
    }
}

pub fn is_authenticate()-> bool {
    return web_local_storage_api::get_item("token").expect("E.").is_some();
} 

pub fn set_token(token: String) {
    let local_token = web_local_storage_api::set_item("token", &token);
    if local_token.is_ok() {
        println!("local_token is_some!");
    }
}

pub fn remove_token(){
    web_local_storage_api::remove_item("token");
}


async fn request<U, T> (
    url: String, 
    method: reqwest::Method, 
    body: &U,
    is_auth: bool
) -> Result<T, u16>
where
    //T: DeserializeOwned + Debug + Send,
    //U: Serialize + Debug,
    T: DeserializeOwned + Send,
    U: Serialize,
{ 
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if is_auth {
        if let Some(token) = get_token(){
            req = req.bearer_auth(token);
        }
    }

    if allow_body{ 
        req = req.json(body);
    }

    //println!("Request: {:?}", req);
    let res_resp = req.send().await;
    //println!("Response: {:?}", res_resp);

    match res_resp {
        Ok(resp) => {

        match resp.status().is_success(){
            true => {
                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        //println!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => {
                Err(resp.status().as_u16())
            }
        }
    },
        Err(err) => {
            //println!("err: {:?}", err);
            Err(0)
        }
    }
}

pub async fn request_delete<T>(url: String, is_auth: bool) -> Result<T, u16>
where
    //T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    T: DeserializeOwned + 'static + Send,
{
    request(url, reqwest::Method::DELETE, &(), is_auth).await
}

/// Get request
pub async fn request_get<T>(url: String, is_auth: bool) -> Result<T, u16>
where
    //T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    T: DeserializeOwned + 'static + Send,
{
    request(url, reqwest::Method::GET, &(), is_auth).await
}

/// Post request with a body
pub async fn request_post<U, T>(url: String, body: &U, is_auth: bool) -> Result<T, u16>
where
    //T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    //U: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + Send,
    U: Serialize,
{
    request(url, reqwest::Method::POST, body, is_auth).await
}

/// Put request with a body
pub async fn request_put<U, T>(url: String, body: &U, is_auth: bool) -> Result<T, u16>
where
    //T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    //U: Serialize + std::fmt::Debug,
    T: DeserializeOwned + 'static + Send,
    U: Serialize,
{
    request(url, reqwest::Method::PUT, body, is_auth).await
}