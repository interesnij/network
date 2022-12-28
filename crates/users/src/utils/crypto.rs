use actix_web::{web::block, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::{result::Result};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

pub async fn gen_jwt (
    id: i32,
    secret: &String
) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_key = secret.clone();

    block(move || {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
        let exp = Utc::now()
            + Duration::days (
                env::var("COOKIE_MAX_AGE")
                .unwrap()
                .parse::<i64>()
                .unwrap()
            );

        let claim = Claims {
            id:  id,
            exp: exp.timestamp(),
        };

        encode(&header, &claim, &encoding_key)
    })
    .await
    .unwrap()
}

pub async fn verify_jwt(_token: String, _secret: &String)-> Result<Claims, u16>{
    let jwt_key = _secret.clone();
    let claims = block(move || {
        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());

        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await
    .unwrap();
    if let Err(_) = claims {
        return Err(403);
    }

    //log::info!("Headers: {:?}", claims.as_ref().unwrap().header);
    let claims = claims.unwrap().claims;

    if claims.exp < Utc::now().timestamp(){
        return Err(419);
    }

    Ok(claims)
}

fn get_auth_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("authorization")?.to_str().ok();
}

pub async fn is_auth(req: &HttpRequest, _secret: &String)-> Result<i32, u16>{
    let jwt_key = _secret.clone();
    let _token: String;
    let _token_some = get_auth_header(&req);
    if _token_some.is_some() {
        _token = _token_some.unwrap();
    }
    else {
        _token = String::new();
    }
    let claims = block(move || {
        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());

        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await
    .unwrap();
    if let Err(_) = claims {
        return Err(403);
    }

    //log::info!("Headers: {:?}", claims.as_ref().unwrap().header);
    let claims = claims.unwrap().claims;

    if claims.exp < Utc::now().timestamp(){
        return Err(419);
    }

    Ok(claims.id)
}

pub async fn get_user_id(_token: String, _secret: &String) -> i32 {
    let jwt_key = _secret.clone();
    let claims = block(move || {
        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());

        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await 
    .unwrap();
    if let Err(_) = claims {
        return 0;
    }

    //log::info!("Headers: {:?}", claims.as_ref().unwrap().header);
    let claims = claims.unwrap().claims;
    println!("claims {:?}", claims.id);
    if claims.exp < Utc::now().timestamp(){
        return 0;
    }

    return claims.id;
}