mod items;
mod attach_items;
pub use self::{
    attach_items::*,
    items::*,
};

use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use crate::schema;
use crate::models::{
    PostList,
    Post,
    PostComment,
    User,
    Community,
    Moderated,
    Owner,
};
use crate::errors::Error;


/* 
токен апи-шлюза. Когда надо произвести доп изменения в сервисах, 
причастных к какому-либо изменению в базах данных. Например, создание токенов
приложений, к которым хочет аппелировать owner.
Или изменение названия сообщества, которое потянет такие изменения на всех
сервисах, в которых участвует сообщество. Такие зависимости пользователей и сообществ
пропишутся в сервисе апи шлюза для более удобного взаимодействия П. и С. с сервисами.
*/
pub static TOKEN: &str = "111";

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

pub fn get_users_from_ids(ids: Vec<i32>) -> Vec<User> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq_any(ids))
        .filter(schema::users::types.lt(31))
        .load::<User>(&_connection)
        .expect("E");
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

pub fn get_owner(pk: i32) -> Result<Owner, Error> {
    use crate::schema::owners::dsl::owners;
    let _connection = establish_connection();
    return Ok(owners
        .filter(schema::owners::id.eq(pk))
        .first::<Owner>(&_connection)?);
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

pub fn get_moderation(pk: i32) -> Result<Moderated, Error> {
    use crate::schema::moderateds::dsl::moderateds;
    let _connection = establish_connection();
    return Ok(moderateds
        .filter(schema::moderateds::id.eq(pk))
        .first::<Moderated>(&_connection)?);
}

pub fn get_owner_data (
    token:         Option<String>,
    user_id:       Option<i32>,
    service_types: i16
) -> (Option<String>, i32, i32) {
    // проверка токена на допуск к нейтральным объектам
    // (например, для загрузки списка в окне)
    // нам нужно узнать по токену тип владельца.
    // заодним мы выясним id пользователя.
    // отдаем ошибку, id пользователя, id сообщества.
    if token.is_some() {
        use crate::schema::owners::dsl::owners;
        let _connection = establish_connection();
        let owner_res = owners
            .filter(schema::owners::service_key.eq(token.unwrap()))
            .first::<Owner>(&_connection);
        if owner_res.is_ok() {
            let owner = owner_res.expect("E");
            if service_types < 1 || !owner.is_service_types_ok(service_types) {
                return (Some("This role is not allowed in this service!".to_string()), 0, 0);
            }
            else if owner.types == 1 {
                // токен приложения, которое работает как наше
                if user_id.is_some() {
                    // параметр id текущего пользователя
                    let _id = user_id.unwrap();
                    let _user = get_user(_id);
                    if _user.is_ok() {
                        return (None, _id, 0);
                    }
                    else {
                        return (Some("user not found!".to_string()), 0, 0);
                    }
                }
                else {
                    // параметра user_id нет - значит пользователь анонимный
                    return (None, 0, 0);
                }
            }
            else if owner.types == 2 {
                // токен пользователя
                return (None, owner.user_id, 0);
            }
            else if owner.types == 3 {
                // токен сообщества
                return (None, owner.user_id, owner.community_id.unwrap());
            }
            else {
                return (Some("owner not found!".to_string()), 0, 0);
            }
        }
        else {
            return (Some("tokens owner not found!".to_string()), 0, 0);
        }
    }
    else {
        return (Some("parametr 'token' not found!".to_string()), 0, 0);
    }
}

pub fn get_user_owner_data (
    token: Option<String>,
    user_id: Option<i32>,
    service_types: i16,
) -> (Option<String>, i32) {
    // проверка токена на допуск к объектам пользователя
    // нам нужно узнать по токену тип владельца.
    // заодним мы выясним id пользователя.
    if token.is_some() {
        use crate::schema::owners::dsl::owners;
        let _connection = establish_connection();
        let owner_res = owners
            .filter(schema::owners::service_key.eq(token.unwrap()))
            .first::<Owner>(&_connection);
        if owner_res.is_ok() {
            let owner = owner_res.expect("E");
            if service_types < 1 || !owner.is_service_types_ok(service_types) {
                return (Some("This role is not allowed in this service!".to_string()), 0);
            }
            else if owner.types == 1 {
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
                    return (Some("parametr 'user_id' not found!".to_string()), 0);
                }
            }
            else if owner.types == 2 {
                return (None, owner.user_id);
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

pub fn get_community_owner_id (
    token:         Option<String>,
    community_id:  Option<i32>,
    service_types: i16,
) -> (Option<String>, i32) {
    // проверка токена на допуск к объектам сообщества
    // нам нужно узнать по токену тип владельца.
    // заодним мы выясним id сообщества.
    if token.is_some() {
        use crate::schema::owners::dsl::owners;
        let _connection = establish_connection();
        let owner_res = owners
            .filter(schema::owners::service_key.eq(token.unwrap()))
            .first::<Owner>(&_connection);
        if owner_res.is_ok() {
            let owner = owner_res.expect("E");
            if service_types < 1 || !owner.is_service_types_ok(service_types) {
                return (Some("This role is not allowed in this service!".to_string()), 0);
            }
            else if owner.types == 1 {
                if community_id.is_some() {
                    let _id = community_id.unwrap();
                    let _community = get_community(_id);
                    if _community.is_ok() {
                        return (None, _id);
                    }
                    else {
                        return (Some("community not found!".to_string()), 0);
                    }
                }
                else {
                    return (Some("parametr 'community_id' not found!".to_string()), 0);
                }
            }
            else if owner.types == 3 {
                return (None, owner.community_id.unwrap());
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

pub fn get_user_permission(user: &User, user_id: i32)
    -> (bool, String) {
    
    if user.id == user_id {
        return (true, "Открыто".to_string())
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

    else if user.is_user_in_block(user_id) {
        return (false, user.get_full_name() + &": заблокировал Вас".to_string())
    }
    else if !user.is_user_see_all(user_id) {
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
    else if !user.is_anon_user_see_all() && !user.is_anon_user_see_el() {
        return (false, user.get_full_name() + &": Ошибка доступа".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}

pub fn get_community_permission(community: &Community, user_id: i32)
    -> (bool, String) {

    if community.types > 10 {
        if community.is_closed() {
            return (false, community.name.clone() + &": сообщество заблокировано за нарушение правил сайта".to_string())
        }
        else if community.is_deleted() {
            return (false, community.name.clone() + &": сообщество удалено".to_string())
        }
        else if community.is_suspended() {
            return (false, community.name.clone() + &": сообщество будет разморожено ".to_string() + &community.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string())}
    }
    else if community.is_user_in_ban(user_id) {
        return (false, community.name.clone() + &": сообщество добавило Вас в чёрный список".to_string())
    }
    else if !community.is_user_see_el(user_id) {
        return (false, community.name.clone() + &": Ошибка доступа".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}

pub fn get_anon_community_permission(community: &Community)
    -> (bool, String) {

    if community.types > 10 {
        if community.is_closed() {
            return (false, community.name.clone() + &": сообщество заблокировано за нарушение правил сайта".to_string())
        }
        else if community.is_deleted() {
            return (false, community.name.clone() + &": сообщество удалено".to_string())
        }
        else if community.is_suspended() {
            return (false, community.name.clone() + &": сообщество будет разморожено ".to_string() + &community.get_longest_penalties())
        }
        else { return (false, "Закрыто".to_string())}
    }
    else if !community.is_anon_user_see_el() && community.types == 2 && community.types == 3 {
        return (false, community.name.clone() + &": ошибка доступа.".to_string())
    }
    else {
        return (true, "Открыто".to_string())
    }
}
