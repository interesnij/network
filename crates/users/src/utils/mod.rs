use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use crate::schema;
use crate::errors::Error;

mod lists;
mod profile;
mod crypto;
pub use self::{
    lists::*,
    profile::*,
    crypto::*,
};
use crate::models::{
    User, Owner, Moderated,
};

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

pub fn get_users_from_ids(ids: Vec<i32>) -> Vec<User> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq_any(ids))
        .filter(schema::users::types.lt(31))
        .load::<User>(&_connection)
        .expect("E");
}
pub fn get_card_users_from_ids(ids: Vec<i32>) -> Vec<CardUserJson> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq_any(ids))
        .filter(schema::users::types.lt(31))
        .select((
            schema::users::id,
            schema::users::first_name,
            schema::users::last_name,
            schema::users::link,
            schema::users::s_avatar,
        ))
        .load::<CardUserJson>(&_connection)
        .expect("E");
}

pub fn get_user(pk: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return Ok(users
        .filter(schema::users::id.eq(pk))
        .first::<User>(&_connection)?);
}

pub fn get_moderation(pk: i32) -> Result<Moderated, Error> {
    use crate::schema::moderateds::dsl::moderateds;
    let _connection = establish_connection();
    return Ok(moderateds
        .filter(schema::moderateds::id.eq(pk))
        .first::<Moderated>(&_connection)?);
}

pub fn get_user_owner_data (
    token: Option<String>,
    user_id: Option<i32>
) -> (Option<String>, i32) {
    // проверка токена на допуск к объектам пользователя
    // нам нужно узнать по токену тип владельца.
    // заодним мы выясним id пользователя.
    if token.is_some() {
        use crate::schema::owners::dsl::owners;
        let _connection = establish_connection();
        let _tok = &token;
        let owner_res = owners
            .filter(schema::owners::service_key.eq(token.unwrap()))
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
                return (None, owner.user_id);
            }
            else {
                return (Some("owner not found!".to_string()), 0);
            }
        }
        // test case!!!
        else if _tok == Some("11".to_string()) {
            return (None, 0);
        }
        else {
            return (Some("tokens owner not found!".to_string()), 0);
        }
    }
    else {
        return (Some("parametr 'token' not found!".to_string()), 0);
    }
}
