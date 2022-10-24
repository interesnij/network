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
    PostList,
    Post,
    PostComment,
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

pub fn get_post_list(pk: i32) -> PostList {
    use crate::schema::post_lists::dsl::post_lists;
    let _connection = establish_connection();
    return post_lists
        .filter(schema::post_lists::id.eq(pk))
        .load::<PostList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_post(pk: i32) -> Post {
    use crate::schema::posts::dsl::posts;
    let _connection = establish_connection();
    return posts
        .filter(schema::posts::id.eq(pk))
        .load::<Post>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_post_comment(pk: i32) -> PostComment {
    use crate::schema::post_comments::dsl::post_comments;
    let _connection = establish_connection();
    return post_comments
        .filter(schema::post_comments::id.eq(pk))
        .load::<PostComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
