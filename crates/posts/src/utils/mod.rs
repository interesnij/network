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
    User,
    Community,
};
use crate::errors::Error;


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

pub fn get_post_list(pk: i32) -> Result<PostList, Error> {
    use crate::schema::post_lists::dsl::post_lists;
    let _connection = establish_connection();
    return Ok(post_lists
        .filter(schema::post_lists::id.eq(pk))
        .first::<PostList>(&_connection)?);
}

pub fn get_post(pk: i32) -> Result<Post, Error> {
    use crate::schema::posts::dsl::posts;
    let _connection = establish_connection();
    return Ok(posts
        .filter(schema::posts::id.eq(pk))
        .first::<Post>(&_connection)?);
}

pub fn get_post_comment(pk: i32) -> Result<PostComment, Error> {
    use crate::schema::post_comments::dsl::post_comments;
    let _connection = establish_connection();
    return Ok(post_comments
        .filter(schema::post_comments::id.eq(pk))
        .first::<PostComment>(&_connection)?);
}

pub fn get_user(pk: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return Ok(users
        .filter(schema::users::id.eq(pk))
        .first::<User>(&_connection)?);
}

pub fn get_community(pk: i32) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let _connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::id.eq(pk))
        .first::<Community>(&_connection)?);
}
