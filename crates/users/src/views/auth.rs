use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::Json,
};
use crate::AppState;
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection, gen_jwt,
    get_user_owner_data,
    ErrorParams
};
use bcrypt::{hash, verify};
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use crate::models::{User, NewUser, NewUserInfo};
use crate::errors::Error;


pub fn auth_urls(config: &mut web::ServiceConfig) {
    config.route("/phone_send", web::post().to(phone_send));
    config.route("/phone_verify", web::post().to(phone_verify));
    config.route("/signup", web::post().to(process_signup));
    config.route("/login", web::post().to(login));
    config.route("/logout", web::get().to(logout));
} 

pub async fn logout() -> HttpResponse {
    HttpResponse::Unauthorized().finish()
}

#[derive(Serialize)]
pub struct RespParams {
    pub resp: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub phone:    String,
    pub password: String,
}
#[derive(Serialize, Debug)]
pub struct TokenParams {
    pub token: String,
}

pub async fn login(data: web::Json<LoginUser2>, state: web::Data<AppState>) -> Result<Json<TokenParams>, Error> {
    let _user = User::get_user_by_phone(&data.phone);
    
    if _user.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Пользователь с таким телдефоном не найден".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let _user = _user.unwrap();

        if verify(data.password.as_str(), _user.password.as_str()).unwrap() {
                let token = gen_jwt(_user.id, state.key.as_ref()).await;
                
                match token {
                    Ok(token_str) => {
                        let body = serde_json::to_string(&TokenParams {
                            token: token_str.to_owned(),
                        }).unwrap();
                        Ok(Json(body))
                    },
                    Err(err) => {
                        let body = serde_json::to_string(&ErrorParams {
                            error: err.to_string(),
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
    pub birthday:   Option<String>,
}
#[derive(Serialize)]
pub struct NewUserDetailJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub is_man:     bool,
    pub link:       String,
}

// данные для создания копий пользователя на других сервисах
#[derive(Serialize)]
pub struct NewUserData {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub is_man:     Option<i16>,
    pub link:       Option<String>,
}

pub async fn process_signup(req: HttpRequest, data: Json<NewUserForm>) -> Result<Json<NewUserDetailJson>, Error> {
    use crate::models::{
        NewUserLocation, NewIpUser,
        NewUserPrivate, NewUserNotification,
        PhoneCode,
    };
    use crate::schema::phone_codes::dsl::phone_codes;
    use chrono::{Duration, Datelike};
    use crate::utils::{TOKEN, USERS_SERVICES};

    let _connection = establish_connection();
    let (err, _) = get_user_owner_data(data.token.clone(), None, 0);
    if err.is_some() {
        return Err(Error::BadRequest(err.unwrap()));
    }
    else if data.first_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'first_name' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.last_name.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'last_name' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'password' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.phone.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'phone' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.birthday.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'birthday' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    // проверим, подтвержден ли телефон, который используется для регистрации
    let _phone_code: PhoneCode;
    let _phone_code_res = phone_codes
        .filter(schema::phone_codes::phone.eq(data.phone.as_deref().unwrap()))
        .filter(schema::phone_codes::types.eq(1))
        .filter(schema::phone_codes::created.gt(chrono::Local::now().naive_utc() - Duration::hours(1)))
        .first::<PhoneCode>(&_connection);
    if _phone_code_res.is_ok() {
        _phone_code = _phone_code_res.expect("E.");
        if !_phone_code.accept {
            let body = serde_json::to_string(&ErrorParams {
                error: "This phone not accepted!".to_string(),
            }).unwrap();
            return Err(Error::BadRequest(body));
        }
    }
    else {
         let body = serde_json::to_string(&ErrorParams {
              error: "This phone not found!".to_string(),
        }).unwrap(); 
        return Err(Error::BadRequest(body));
    }
    
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

    let birthday = chrono::NaiveDate::parse_from_str(data.password.as_deref().unwrap(), "%Y-%m-%d").unwrap();
    let types: i16;
    if birthday.year() < 2006 {
        types = 3;
    } 
    else {
        types = 1;
    }
    let count = User::count_users() + 1;
    let link = "/id".to_string() + &count.to_string();
    let form_user = NewUser {
        first_name:    data.first_name.as_deref().unwrap().to_string(),
        last_name:     data.last_name.as_deref().unwrap().to_string(),
        phone:         data.phone.as_deref().unwrap().to_string(),
        types:         types,
        is_man:        is_man,
        password:      hash(data.password.as_deref().unwrap(), 8).unwrap(),
        link:          link,
        last_activity: chrono::Local::now().naive_utc(),
    }; 

    let _new_user = diesel::insert_into(schema::users::table)
        .values(&form_user)
        .get_result::<User>(&_connection)
        .expect("Error saving user."); 
    // удалим телефон из таблицы подтвержденных телефонов, чтобы он больше не использовался
    let _del = diesel::delete(
        phone_codes
            .filter(schema::phone_codes::phone.eq(&data.phone.as_deref().unwrap()))
            .filter(schema::phone_codes::types.eq(1))
        )
        .execute(&_connection)
        .expect("E.");

    let info_user = NewUserInfo {
        user_id:   _new_user.id,
        avatar_id: None,
        language:  "Ru".to_string(),
        email:     None,
        birthday:  chrono::NaiveDate::parse_from_str(data.password.as_deref().unwrap(), "%Y-%m-%d").unwrap(),
        b_avatar:  None,
        status:    None,
        city:      None,
        level:     100,
        cover:     None,
        created:   chrono::Local::now().naive_utc(),
        friends:   0,
        follows:   0,
    }; 

    let _info_user = diesel::insert_into(schema::user_infos::table)
        .values(&info_user)
        .execute(&_connection)
        .expect("Error saving user info."); 

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
    };
    diesel::insert_into(schema::user_notifications::table)
        .values(&_user_notification)
        .execute(&_connection)
        .expect("Error saving user_notification.");

    let copy_user = NewUserData {
        token:      Some(TOKEN.to_string()),
        user_id:    Some(_new_user.id),
        first_name: Some(_new_user.first_name.clone()),
        last_name:  Some(_new_user.last_name.clone()),
        is_man:     data.is_man, 
        link:       Some(_new_user.link.clone()),
    };

    for link in USERS_SERVICES.iter() {
        let client = reqwest::Client::new();
        let _res = client.post(link.to_string() + &"/create_user".to_string())
            .form(&copy_user)
            .send()
            .await;
    }

    Ok(Json(NewUserDetailJson {
        id:         _new_user.id,
        first_name: _new_user.first_name.clone(),
        last_name:  _new_user.last_name.clone(),
        is_man:     _new_user.is_man,
        link:       _new_user.link.clone(),
    })) 
}

#[derive(Deserialize)]
pub struct PhoneJson {
    pub token: Option<String>,
    pub phone: Option<String>,
}
#[derive(Deserialize)]
pub struct PhoneCodeJson {
    pub token: String,
    pub phone: String,
    pub code:  String,
}
#[derive(Deserialize)]
pub struct CodeJson {
    pub code: String,
}

pub async fn phone_send(data: Json<PhoneJson>) -> Json<RespParams> {
    let (err, _user_id) = get_user_owner_data(data.token.clone(), None, 0);
    println!("start");
    if err.is_some() {   
        println!("err token");
        return Json( RespParams {
            resp: err.unwrap()
        });
    }  
    let _phone = data.phone.as_deref().unwrap().to_string();
    println!("_phone: {:?}", _phone);
    if _phone.len() > 8 {
        use crate::models::NewPhoneCode;
        use crate::schema::users::dsl::users;
    
        let _connection = establish_connection();
        if users
            .filter(schema::users::phone.eq(_phone.clone()))
            .select(schema::users::id)
            .first::<i32>(&_connection)
            .is_ok() {
            println!("Пользователь с таким номером уже зарегистрирован");
            Json( RespParams {
                resp: "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.".to_string()
            })
        }
        else {
            let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &_phone;
            let __request = reqwest::get(_url).await.expect("E.");
            let new_request = __request.text().await.unwrap();
            println!("{:?}", new_request);
    
            let phone200: CodeJson = serde_json::from_str(&new_request).unwrap();
            let _code: i32 = phone200.code.parse().unwrap();
            let new_phone_code = NewPhoneCode {
                phone:   _phone,
                code:    _code,
                types:   1,
                accept:  false,
                created: chrono::Local::now().naive_utc(),
            };
            let c = diesel::insert_into(schema::phone_codes::table)
                .values(&new_phone_code)
                .execute(&_connection);
            
            if c.is_ok() {
                Json( RespParams {
                    resp: "1".to_string()
                })
            }
            else {
                Json( RespParams {
                    resp: "1".to_string()
                })
            }
        }
    }
    else {
        println!("phone is small");
        Json( RespParams {
            resp: "Введите, пожалуйста, корректное количество цифр Вашего телефона".to_string()
        })
    }
}

#[derive(Deserialize)]
pub struct OptionPhoneCodeJson {
    pub token: Option<String>,
    pub phone: Option<String>,
    pub code:  Option<String>,
}
pub async fn phone_verify(data: web::Json<OptionPhoneCodeJson>) -> Result<Json<RespParams>, Error> {
    let (err, user_id) = get_user_owner_data(data.token.clone(), None, 0);
    if err.is_some() || (user_id != 0) {
        return Err(Error::BadRequest(err.unwrap()));
    } 
    else if data.phone.is_none() {
        return Ok(Json( RespParams {
            resp: "Field 'phone' is required!".to_string(),
        }));
    }
    else if data.code.is_none() {
        return Ok(Json( RespParams {
            resp: "Field 'code' is required!".to_string(),
        }));
    }
        use crate::schema::phone_codes::dsl::phone_codes;
        use crate::models::PhoneCode;
        use chrono::Duration;

        let _connection = establish_connection();
        let _phone = data.phone.as_deref().unwrap();
        let code: Result<i32, _> = data.code
            .as_deref()
            .unwrap()
            .parse();
        
        if code.is_err() {
            return Ok(Json( RespParams {
                resp: "Field 'code' is incorrect!".to_string(),
            }));
        }
        
        let _phone_code: PhoneCode;
        let _phone_code_res = phone_codes
            .filter(schema::phone_codes::phone.eq(_phone.clone()))
            .filter(schema::phone_codes::code.eq(code.expect("E.")))
            .filter(schema::phone_codes::created.gt(chrono::Local::now().naive_utc() - Duration::hours(1)))
            .first::<PhoneCode>(&_connection);
        if _phone_code_res.is_ok() {
            _phone_code = _phone_code_res.expect("E.");
            let _update = diesel::update(&_phone_code)
                .set(schema::phone_codes::accept.eq(true))
                .execute(&_connection); 
            Ok(Json(RespParams {
                resp: "1".to_string(),
            }))
        }
        else {
            Ok(Json( RespParams {
                resp: "Field 'code' is incorrect!".to_string(),
            }))
        }
    }
