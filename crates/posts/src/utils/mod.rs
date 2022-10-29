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
        .filter(schema::users::user_id.eq(pk))
        .first::<User>(&_connection)?);
}

pub fn get_community(pk: i32) -> Result<Community, Error> {
    use crate::schema::communitys::dsl::communitys;
    let _connection = establish_connection();
    return Ok(communitys
        .filter(schema::communitys::community_id.eq(pk))
        .first::<Community>(&_connection)?);
}


pub fn get_user_permission(user: &User, request_user: &User)
    -> (bool, String) {

    if request_user.types > 10 {
        if request_user.is_closed() {
            return (false, "Ваша страница заблокирована.".to_string())
        }
        else if request_user.is_deleted() {
            return (false, "Ваша страница удалена.".to_string())
        }
        else if request_user.is_suspended() {
            return (false, "Ваша страница будет разморожена ".to_string() + &request_user.get_longest_penalties();
        }
        else { return (false, "Закрыто".to_string())}
    }

    else if user.types > 10 {
        if user.is_closed() {
            return (false, user.get_full_name() + &": cтраница заблокирована".to_string())
        }
        else if user.is_deleted() {
            return (false, user.get_full_name() + &": cтраница удалена".to_string())
        }
        else if user.is_suspended() {
            return (false, user.get_full_name() + &": cтраница будет разморожена ".to_string() + &user.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string())}
    }

    else if user.is_user_in_block(request_user.id) {
        return (false, user.get_full_name() + &": заблокировал Вас".to_string())
    }
    else if !user.is_user_can_see_all(request_user.id) {
        return (false, user.get_full_name() + &": профиль закрыт, информация недоступна".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}

pub fn get_anon_user_permission(user: &User)
    -> (bool, String) {
    if user.types > 10 {
        if user.is_closed() {
            return (false, user.get_full_name() + &": cтраница заблокирована".to_string())
        }
        else if user.is_deleted() {
            return (false, user.get_full_name() + &": cтраница удалена".to_string())
        }
        else if user.is_suspended() {
            return (false, user.get_full_name() + &": cтраница будет разморожена ".to_string() + &user.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string());}
    }
    else if !user.is_anon_user_can_see_all(request_user.id) {
        return (false, user.get_full_name() + &": профиль закрыт, информация недоступна".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}

pub fn get_community_permission(community: &Community, request_user: &User)
    -> (bool, String) {

    if request_user.types > 10 {
        if request_user.is_closed() {
            return (false, "Ваша страница заблокирована.".to_string())
        }
        else if request_user.is_deleted() {
            return (false, "Ваша страница удалена.".to_string())
        }
        else if request_user.is_suspended() {
            return (false, "Ваша страница будет разморожена ".to_string() + &request_user.get_longest_penalties();
        }
        else { return (false, "Закрыто".to_string());}
    }

    else if community.types > 10 {
        if community.is_closed() {
            return (false, community.name + &": сообщество заблокировано за нарушение правил сайта".to_string())
        }
        else if community.is_deleted() {
            return (false, community.name + &": сообщество удалено".to_string())
        }
        else if community.is_suspended() {
            return (false, community.name + &": сообщество будет разморожено ".to_string() + &community.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string())}
    }
    else if request_user.is_banned_from_community(community.id) {
        return (false, community.name + &": сообщество добавило Вас в чёрный список".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}

pub fn get_anon_community_permission(community: &Community)
    -> (bool, String) {

    if community.types > 10 {
        if community.is_closed() {
            return (false, community.name + &": сообщество заблокировано за нарушение правил сайта".to_string())
        }
        else if community.is_deleted() {
            return (false, community.name + &": сообщество удалено".to_string())
        }
        else if community.is_suspended() {
            return (false, community.name + &": сообщество будет разморожено ".to_string() + &community.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string())}
    }
    else if community.types == 2 && community.types == 3 {
        return (false, community.name + &": ошибка доступа.".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}
