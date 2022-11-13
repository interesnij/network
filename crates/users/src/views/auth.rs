use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    web::block,
    web::Json,
    cookie::time::{Duration, OffsetDateTime},
    http::{header::HeaderName, header::HeaderValue, header},
};
use crate::AppState;
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection, gen_jwt,
    get_user_owner_data,
    Claims, ErrorParams, InfoParams,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use crate::models::{User, NewUser};
use crate::errors::Error;


pub fn auth_urls(config: &mut web::ServiceConfig) {
    config.route("/phone_send/", web::get().to(phone_send));
    config.route("/phone_verify/", web::get().to(phone_verify));
    config.route("/signup/", web::post().to(process_signup));
    config.route("/login/", web::post().to(login));
    //config.route("/logout/", web::get().to(logout));
}

//pub async fn logout(session: Session) -> HttpResponse {
//    session.clear();
//    HttpResponse::Ok().body("ok")
//}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub phone:    String,
    pub password: String,
}

pub async fn login(data: web::Json<LoginUser2>, state: web::Data<AppState>) -> Result<Json<InfoParams>, Error> {
    let _user = User::get_user_by_phone(&data.phone);

    if _user.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Пользователь с таким телефоном не найден!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let _user = _user.unwrap();

        if bcrypt::verify(data.password.as_str(), _user.password.as_str()).unwrap() {
                let token = gen_jwt(_user.id, state.key.as_ref()).await;

                match token {
                    Ok(token_str) => {
                        let body = serde_json::to_string(&InfoParams {
                            info: token_str,
                        }).unwrap();
                        Ok(Json(body))
                    },
                    Err(err) => {
                        let body = serde_json::to_string(&ErrorParams {
                            error: err,
                        }).unwrap();
                        Err(Error::BadRequest(body))
                    }
                }
        } else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Пароль неверный!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
    }
}

#[derive(Deserialize)]
pub struct UserLoc {
    pub city:    CityLoc,
    pub region:  RegionLoc,
    pub country: CountryLoc,
}
#[derive(Deserialize)]
pub struct CityLoc {
    pub name_ru: String,
    pub name_en: String,
}
#[derive(Deserialize)]
pub struct RegionLoc {
    pub name_ru: String,
    pub name_en: String,
}
#[derive(Deserialize)]
pub struct CountryLoc {
    pub name_ru: String,
    pub name_en: String,
}

#[derive(Deserialize)]
pub struct NewUserForm {
    pub token:      Option<String>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub is_man:     Option<i16>,
    pub password:   Option<String>,
    pub phone:      Option<String>,
}
#[derive(Serialize)]
pub struct NewUserDetailJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub is_man:     bool,
    pub link:       String,
}

pub async fn process_signup(req: HttpRequest, data: Json<NewUserForm>) -> Result<Json<NewUserDetailJson>, Error> {
    use crate::models::{
        NewUserLocation, NewUserInfo, NewIpUser,
        NewUserPrivate, NewUserNotification,
    };
    use crate::schema::verified_phones::dsl::verified_phones;

    let (err, _) = get_user_owner_data(data.token.clone(), None);
    if err.is_some() {
        // если проверка токена не удалась...
        Err(Error::BadRequest(err.unwrap()))
    }
    else if data.first_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'first_name' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.last_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'last_name' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'password' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.phone.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'phone' not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if verified_phones
        .filter(schema::verified_phones::phone.eq(data.phone.as_deref().unwrap()))
        .select(schema::verified_phones::id)
        .first::<i32>(&_connection)
        .is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "phone not verified!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
    }
    else {
        let is_man: bool;
        if data.is_man.is_none() {
            is_man = true;
        }
        else {
            if data.is_man.unwrap() != 1 {
                is_man = false;
            }
            else {
                is_man = true;
            }
        }
        let _connection = establish_connection();
        let mut ipaddr: String = String::new();

        if let Some(val) = &req.peer_addr() {
            ipaddr = val.ip().to_string();
        };

        let count = User::count_users() + 1;
        let link = "/id".to_string() + &count.to_string() + &"/".to_string();
        let form_user = NewUser {
            first_name:    data.first_name.as_deref().unwrap(),
            last_name:     data.last_name.as_deref().unwrap(),
            phone:         data.phone.as_deref().unwrap(),
            types:         1,
            is_man:        is_man,
            password:      hash(data.password.as_deref().unwrap(), 8).unwrap(),
            link:          link,
            last_activity: chrono::Local::now().naive_utc(),
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        // записываем местоположение нового пользователя
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_owned() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let _user_location = NewUserLocation {
            user_id: _new_user.id,
            city_ru: Some(location200.city.name_ru),
            city_en: Some(location200.city.name_en),
            region_ru: Some(location200.region.name_ru),
            region_en: Some(location200.region.name_en),
            country_ru: Some(location200.country.name_ru),
            country_en: Some(location200.country.name_en),
        };
        diesel::insert_into(schema::user_locations::table)
            .values(&_user_location)
            .execute(&_connection)
            .expect("Error saving user_location.");

        // записываем IP нового пользователя
        let _user_ip = NewIpUser {
            user_id: _new_user.id,
            ip: ipaddr,
        };
        diesel::insert_into(schema::ip_users::table)
            .values(&_user_ip)
            .execute(&_connection)
            .expect("Error saving user_ip.");

        // записываем приватность нового пользователя
        let _user_private = NewUserPrivate {
            user_id:    _new_user.id,
            see_all:    1,
            see_info:   1,
            see_friend: 1,
        };
        diesel::insert_into(schema::user_privates::table)
            .values(&_user_private)
            .execute(&_connection)
            .expect("Error saving user_private.");

        // записываем уведомления профиля нового пользователя
        let _user_notification = NewUserNotification {
            user_id:              _new_user.id,
            connection_request:   true,
            connection_confirmed: true,
            user_invite:          true,
        };
        diesel::insert_into(schema::user_notifications::table)
            .values(&_user_notification)
            .execute(&_connection)
            .expect("Error saving user_notification.");

        Ok(Json(NewUserDetailJson {
            id:         _new_user.id,
            first_name: _new_user.first_name.clone(),
            last_name:  _new_user.last_name.clone(),
            is_man:     _new_user.is_man,
            link:       _new_user.link.clone(),
        }))
    }
}

#[derive(Deserialize, Serialize)]
pub struct PhoneCodeJson {
    pub phone: String,
    pub code:  String,
}
pub async fn phone_send(data: web::Json<PhoneCodeJson>) -> Result<i16, Error> {
    let req_phone = data.phone;
    if req_phone.len() > 8 {
        use crate::models::NewPhoneCode;
        use crate::schema::{
            users::dsl::users,
            verified_phones::dsl::verified_phones,
        };

        let _connection = establish_connection();
        if users
            .filter(schema::users::phone.eq(&req_phone))
            .first::<User>(&_connection)
            .is_ok() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if verified_phones
            .filter(schema::verified_phones::phone.eq(data.phone.clone()))
            .select(schema::verified_phones::id)
            .first::<i32>(&_connection)
            .is_ok() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "phone already verified!".to_string(),
                }).unwrap();
                Err(Error::BadRequest(body))
        }
        else {
            let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &req_phone;
            let __request = reqwest::get(_url).await.expect("E.");
            let new_request = __request.text().await.unwrap();
            println!("{:?}", new_request);

            let phone200: PhoneCodeJson = serde_json::from_str(&new_request).unwrap();
            let code_i32: i32 = phone200.code.parse().unwrap();
            let new_phone_code = NewPhoneCode {
                phone: req_phone.to_string(),
                code:  code_i32,
            };
            let c = diesel::insert_into(schema::phone_codes::table)
                .values(&new_phone_code)
                .execute(&_connection);
            if c.is_ok() {
                Ok(*Json(1))
            }
            else {
                Ok(*Json(0))
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Введите, пожалуйста, корректное количество цифр Вашего телефона".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn phone_verify(data: web::Json<PhoneCodeJson>) -> Result<i16, Error> {
    use crate::schema::phone_codes::dsl::phone_codes;
    use crate::models::NewVerifiedPhone;

    let _connection = establish_connection();
    let _phone = data.phone.clone();
    let _code: i32 = data.code.parse().unwrap();

    let _res = block(move || {
        if phone_codes
            .filter(schema::phone_codes::phone.eq(&_phone))
            .filter(schema::phone_codes::code.eq(&_code))
            .select(schema::phone_codes::id)
            .first::<i32>(&_connection)
            .is_ok() {
            let new_phone_v = NewVerifiedPhone {
                phone: _phone.to_string(),
            };
            diesel::insert_into(schema::verified_phones::table)
                .values(&new_phone_v)
                .execute(&_connection)
                .expect("E.");

            diesel::delete (
            phone_codes
                .filter(schema::phone_codes::phone.eq(&_phone))
                .filter(schema::phone_codes::code.eq(&_code))
            )
            .execute(&_connection)
            .expect("E");
            1
        }
        else {
            0
        }
    }).await?;

    Ok(_res)
}
