mod community;
//use serde::Serialize;
use diesel::prelude::*;
use crate::schema;
use crate::models::Community;

pub use self::{
    community::*,
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

pub fn get_community(id: i32) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let _connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::id.eq(id))
        .first(&connection));
}
pub fn get_community_with_link(link: String) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let _connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::link.eq("/".to_owned() + &link + &"/".to_string()))
        .first(&connection));
}
