mod community;

use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use crate::schema;
use crate::models::{
    Community, User, 
    Owner, Moderation,
};
use crate::errors::Error;
pub use self::{
    community::*,
};


pub fn get_limit (
    limit: Option<i64>,
    default_limit: i64
) -> i64 {
    let _limit: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    _limit
}

pub fn get_limit_offset (
    limit: Option<i64>,
    offset: Option<i64>,
    default_limit: i64
) -> (i64, i64) {
    let _limit: i64;
    let _offset: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    if offset.is_some() {
        _offset = offset.unwrap();
    }
    else {
        _offset = 0;
    }
    (_limit, _offset)
}

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_count_for_ru(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}
pub fn get_count_for_ru_alt(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    if a == 1 && b != 11 {
        return word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return word2;
    }
    else {
        return word3;
    }
}

pub fn get_community(id: i32) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::id.eq(id))
        .first(&connection)?);
}
pub fn get_community_with_link(link: String) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::link.eq("/".to_owned() + &link + &"/".to_string()))
        .first(&connection)?);
}
pub fn get_user(id: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::users;
    let connection = establish_connection();
    return Ok(users
        .filter(schema::users::id.eq(id))
        .first(&connection)?);
}
pub fn get_owner(id: i32) -> Result<Owner, Error> {
    use crate::schema::owners::dsl::owners;
    let connection = establish_connection();
    return Ok(owners
        .filter(schema::owners::id.eq(id))
        .first(&connection)?);
}
pub fn get_moderation(id: i32) -> Result<Moderation, Error> {
    use crate::schema::moderateds::dsl::moderateds;
    let connection = establish_connection();
    return Ok(moderations
        .filter(schema::moderateds::id.eq(id))
        .first(&connection)?);
}

pub fn get_user_owner_data ( 
    token:         Option<String>,  // токен
    user_id:       Option<i32>,     // возможный id request_user'а,
    service_types: i16              // тип сервиса и роли в нем
) -> (Option<String>, i32) {
    // проверка токена на допуск к объектам пользователя
    // нам нужно узнать по токену тип владельца.
    // заодним мы выясним id пользователя.
    if token.is_some() {
        use crate::schema::owners::dsl::owners;
        let _connection = establish_connection();
        let _token = token.as_deref().unwrap();
        let owner_res = owners
            .filter(schema::owners::service_key.eq(_token))
            .first::<Owner>(&_connection);
        if owner_res.is_ok() {
            let owner = owner_res.expect("E");
            if owner.types == 1 {
                if user_id.is_some() {
                    let _id = user_id.unwrap();
                    let _user = get_user(_id);
                    if _user.is_ok() {
                        return (None, _id);
                    } 
                    else {
                        return (Some("user not found!".to_string()), 0);
                    }
                }
                else {
                    // параметра user_id нет - значит пользователь анонимный
                    return (None, 0);
                }
            }
            else if owner.types == 2 {
                // это токен пользователя
                if service_types > 0 && owner.is_service_types_ok(service_types) {
                    // проверим, есть ли запрашиваемый сервис и роль в нем
                    // у этого токена
                    return (None, owner.user_id);
                }
                return (Some("This role is not allowed in this service!".to_string()), 0);
            }
            else {
                return (Some("owner not found!".to_string()), 0);
            }
        }
        else {
            return (Some("tokens owner not found!".to_string()), 0);
        }
    }
    else {
        return (Some("parametr 'token' not found!".to_string()), 0);
    }
}