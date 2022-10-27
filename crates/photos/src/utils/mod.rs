mod items;
mod attach_items;
pub use self::{
    attach_items::*,
    items::*,
};

use diesel::{
    //Queryable,
    //Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    //NullableExpressionMethods,
    PgConnection,
    Connection,
};
use crate::schema;
//use serde::{Serialize, Deserialize};
use crate::models::{
    PhotoList,
    Photo,
    PhotoComment,
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

pub fn get_photo_list(pk: i32) -> PhotoList {
    use crate::schema::photo_lists::dsl::photo_lists;
    let _connection = establish_connection();
    return photo_lists
        .filter(schema::photo_lists::id.eq(pk))
        .load::<PhotoList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_photo(pk: i32) -> Photo {
    use crate::schema::photos::dsl::photos;
    let _connection = establish_connection();
    return photos
        .filter(schema::photos::id.eq(pk))
        .load::<Photo>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_photo_comment(pk: i32) -> PhotoComment {
    use crate::schema::photo_comments::dsl::photo_comments;
    let _connection = establish_connection();
    return photo_comments
        .filter(schema::photo_comments::id.eq(pk))
        .load::<PhotoComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
