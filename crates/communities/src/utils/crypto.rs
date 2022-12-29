use actix_web::web::block;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{result::Result};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
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
    let claims = claims.unwrap().claims;
    if claims.exp < Utc::now().timestamp(){
        return Err(419);
    }
    Ok(claims)
}