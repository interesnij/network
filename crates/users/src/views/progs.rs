use actix_web::{
    HttpRequest,
    web,
    web::Json,
};
use crate::models::{User, GetSessionFields};
use serde::{Serialize, Deserialize};
use crate::utils::{
    verify,
};

pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/find_user/{phone}/{password}/", web::get().to(find_user));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: u64,
    pub phone: String,
}
pub async fn find_user (
    _state: web::Data<AppState>,
    param: web::Path<(String, String)>
) -> Json<SessionUser> {
    let user: Result<User, _> = _state.rb.fetch_by_column("phone", param.0).await;
    match user {
        Ok(user_data) => {
            if let Ok(matching) = verify(&user.password, &param.1) {
                if matching {
                    let body = serde_json::to_string(&SessionUser {
                        id: user_data.id,
                        phone: user_data.phone,
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let body = serde_json::to_string(&SessionUser {
                    id:    0,
                    phone: "".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
        },
        Err(_) => {
            let body = serde_json::to_string(&SessionUser {
                id:    0,
                phone: "".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        },
    }
}
