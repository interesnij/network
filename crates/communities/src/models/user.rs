use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use crate::schema::{
    users,
    friends,
    follows,
    user_visible_perms,
};
use crate::errors::Error;
//use actix_web::web::Json;

///// Типы пользоватетеля
    // 1 стандартный тип пользователя
    // 6 пославший запрос на идентификацию
    // 7 идентифицированный

    // 11 удаленный стандартный
    // 16 удаленный пославший запрос на идентификацию
    // 17 удаленный идентифицированный

    // 21 закрытый стандартный
    // 26 закрытый пославший запрос на идентификацию
    // 27 закрытый идентифицированный

    // 31 приостановленный стандартный
    // 36 приостановленный пославший запрос на идентификацию
    // 37 приостановленный идентифицированный

    // 41 закрытый баннером стандартный
    // 46 закрытый баннером пославший запрос на идентификацию
    // 47 закрытый баннером идентифицированный


// эти объекты нужны только для связывания пользователей
// сервиса сообществ с пользователями сервиса пользователей.
// Эти объекты копируются при надобности с объектов
// пользователей сервиса пользователей
// 1 Все пользователи
// 2 Все друзья и все подписчики
// 3 Все друзья и подписчики, кроме
// 4 Все друзья и некоторые подписчики
// 5 Все подписчики и друзья, кроме
// 6 Все подписчики и некоторые друзья
// 7 Все друзья

// 8 Все подписчики
// 9 Друзья, кроме
// 10 Некоторые друзья
// 11 Подписчики, кроме
// 12 Некоторые подписчики
// 13 Только я
#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:            i32,
    pub user_id:       i32,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         i16,
    pub is_man:        bool,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub see_all:       i16,
    pub see_community: i16,
    pub communities:   i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_id:       i32,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         i16,
    pub is_man:        bool,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub see_all:       i16,
    pub see_community: i16,
    pub communities:   i32,
}
#[derive(Deserialize)]
pub struct NewUserJson {
    pub user_id:    i32,
    pub first_name: String,
    pub last_name:  String,
    pub types:      i16,
    pub is_man:     bool,
    pub link:       String,
    pub s_avatar:   Option<String>,
    pub see_all:    bool,
    pub friends:    Option<Vec<i32>>,  // список id друзей пользователя
    pub follows:    Option<Vec<i32>>,  // список id подписчтков пользователя
}

impl User {
    pub fn is_supermanager(&self) -> bool {
        return self.types == 25;
    }
    pub fn is_administrator(&self) -> bool {
        return self.types > 20 && self.types < 30;
    }
    pub fn is_advertiser(&self) -> bool {
        return self.types > 17 && self.types < 30;
    }
    pub fn is_manager(&self) -> bool {
        return self.types > 13 && self.types < 30;
    }
    pub fn is_moderator(&self) -> bool {
        return self.types > 9 && self.types < 30;
    }
    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }

    pub fn count_communities(&self) -> i32 {
        return self.communities;
    }
    pub fn count_communities_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn count_communities_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn plus_communities(&self, count: i32) -> bool {
        let _connection = establish_connection();
        let _u = diesel::update(self)
            .set(schema::users::communities.eq(self.communities + count))
            .execute(&_connection);
        if _u.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn minus_communities(&self, count: i32) -> bool {
        if self.communities > 0 {
            let _connection = establish_connection();
            let _u = diesel::update(self)
                .set(schema::users::communities.eq(self.communities - count))
                .execute(&_connection);
            if _u.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }
        return false;
    }

    pub fn get_or_create_user(user: NewUserJson) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if users
            .filter(schema::users::user_id.eq(user.user_id))
            .limit(1)
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len() > 0 {
                return Ok(users
                    .filter(schema::users::user_id.eq(user.user_id))
                    .first(&_connection)?);
        }
        let new_form = NewUser {
            user_id:       user.user_id,
            first_name:    user.first_name.clone(),
            last_name:     user.last_name.clone(),
            types:         user.types,
            is_man:        user.is_man,
            link:          user.link.clone(),
            s_avatar:      user.s_avatar.clone(),
            last_activity: chrono::Local::now().naive_utc(),
            see_all:       user.see_all,
            see_community: 1,
            communities:   0,
        };
        let new_user = diesel::insert_into(schema::users::table)
            .values(&new_form)
            .get_result::<User>(&_connection);

        let new_user_id = match &new_user {
             Ok(_ok) => _ok.id,
             Err(_error) => 0,
        };

        if new_user_id > 0 && user.friends.is_some() {
            use crate::schema::friends::dsl::friends;

            // user_id кто дружит
            // target_id с кем дружит
            for user_id in user.friends.unwrap() {
                if friends
                    .filter(schema::friends::user_id.eq(new_user_id))
                    .filter(schema::friends::target_id.eq(user_id))
                    .select(schema::friends::id)
                    .first::<i32>(&_connection)
                    .is_err() {
                        let new_form = NewFriend {
                            user_id:   new_user_id,
                            target_id: user_id,
                        };
                        diesel::insert_into(schema::friends::table)
                            .values(&new_form)
                            .get_result::<Friend>(&_connection)
                            .expect("Error.");
                }
            }
        }
        if new_user_id > 0 && user.follows.is_some() {
            use crate::schema::follows::dsl::follows;
            // user_id на кого подписан
            // target_id кто подписан
            for user_id in user.follows.unwrap() {
                if follows
                    .filter(schema::follows::user_id.eq(new_user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                    .select(schema::follows::id)
                    .first::<i32>(&_connection)
                    .is_err() {
                        let new_form = NewFollow {
                            user_id:   new_user_id,
                            target_id: user_id,
                        };
                        diesel::insert_into(schema::follows::table)
                            .values(&new_form)
                            .get_result::<Follow>(&_connection)
                            .expect("Error.");
                }
            }
        }
        return new_user;
    }
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_str_id(&self) -> String {
        return self.user_id.to_string();
    }
    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.get_full_name() + &"</a>".to_string();
    }
    pub fn get_code(&self) -> String {
        return "use".to_string() + &self.get_str_id();
    }

    pub fn get_see_community_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_community_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_community_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_community_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn is_user_see_community_exclude(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(11))
            .select(schema::user_visible_perms::target_id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_see_community_include(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(1))
            .select(schema::user_visible_perms::target_id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_see_all_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(10))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(0))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(10))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(0))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn is_user_see_all_exclude(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(10))
            .select(schema::user_visible_perms::target_id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_see_all_include(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(0))
            .select(schema::user_visible_perms::target_id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn is_user_see_community(&self, user_id: i32) -> bool {
        // все запросы происходят от id пользователя основного,
        // с сервиса пользователей, и сравниваются потому с user_id
        // местной таблицы пользователей. Потому сравниваем с self.user_id
        if self.user_id == user_id {
            return true;
        }
        return match self.see_community {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_community_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
            4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_community_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
            5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_community_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
            6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_community_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.get_see_community_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
            10 => self.get_see_community_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
            11 => !self.get_see_community_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
            12 => self.get_see_community_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
            _ => false,
        };
    }

    pub fn is_user_see_all(&self, user_id: i32) -> bool {
        // все запросы происходят от id пользователя основного,
        // с сервиса пользователей, и сравниваются потому с user_id
        // местной таблицы пользователей. Потому сравниваем с self.user_id
        if self.user_id == user_id {
            return true;
        }
        return match self.see_all {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_all_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
            4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_all_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
            5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_all_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
            6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_all_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.get_see_all_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
            10 => self.get_see_all_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
            11 => !self.get_see_all_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
            12 => self.get_see_all_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
            _ => false,
        };
    }

    pub fn get_friends_ids(&self) -> Vec<i32> {
        // в местные таблицы друзей и подписчиков мы записываем
        // id пользователей с сервиса пользователей, чтобы было
        // корректнее их сравнивать.
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.user_id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_follows_ids(&self) -> Vec<i32> {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follows = follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _follows;
    }
    pub fn set_user_visible_perms(&self, users: String, types: i16) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let mut users_ids = Vec::new();
        let v: Vec<&str> = users.split(", ").collect();
        for item in v.iter() {
            if !item.is_empty() {
                let pk: i32 = item.parse().unwrap();
                users_ids.push(pk);
            }
        }

        // нужно удалить из списка тех, кто был туда внесен
        // с противоположными правами.
        match types {
            1 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(11))
                )
                .execute(&_connection)
                .expect("E"),
            11 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(1))
                )
                .execute(&_connection)
                .expect("E"),
            _ => 0,
        };
        for user_id in users_ids.iter() {
            let _new_perm = NewUserVisiblePerm {
                user_id:   self.user_id,
                target_id: *user_id,
                types:     types,
            };
            diesel::insert_into(schema::user_visible_perms::table)
                .values(&_new_perm)
                .get_result::<UserVisiblePerm>(&_connection)
                .expect("Error.");
        }
        return true;
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.types {
            1 => 21,
            2 => 22,
            3 => 23,
            7 => 27,
            8 => 28,
            9 => 29,
            13 => 33,
            14 => 34,
            15 => 35,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
        }
        return 0;
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.types {
            21 => 1,
            22 => 2,
            23 => 3,
            27 => 7,
            28 => 8,
            29 => 9,
            33 => 13,
            34 => 14,
            35 => 15,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.types {
            1 => 61,
            2 => 62,
            3 => 63,
            7 => 67,
            8 => 68,
            9 => 69,
            13 => 73,
            14 => 74,
            15 => 75,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.types {
            61 => 1,
            62 => 2,
            63 => 3,
            67 => 7,
            68 => 8,
            69 => 9,
            73 => 13,
            74 => 14,
            75 => 15,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }

    pub fn get_verb_gender(&self, str: &str) -> String {
        if self.is_man == false {
            return "W".to_string() + &str;
        }
        else {
            return str.to_string();
        }
    }
    pub fn is_women(&self) -> bool {
        return self.is_man == false;
    }
    pub fn is_man(&self) -> bool {
        return self.is_man;
    }
    pub fn is_suspended(&self) -> bool {
        return self.types < 40 && self.types > 30;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types < 50 && self.types > 40;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types < 20 && self.types > 10;
    }
    pub fn is_closed(&self) -> bool {
        return self.types < 30 && self.types > 20;
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types == 6;
    }
    pub fn is_identified(&self) -> bool {
        return self.types > 6 && self.types < 30;
    }

    pub fn is_online(&self) -> bool {
        use chrono::Duration;
        return (self.last_activity + Duration::seconds(300)) > chrono::Local::now().naive_utc();
    }
    pub fn get_online_status(&self) -> String {
        if self.is_online() {
            return "Онлайн".to_string();
        }
        else {
            if self.is_women() {
                return "Была ".to_string() + &self.last_activity.to_string();
            } else {
                return "Был ".to_string() + &self.last_activity.to_string();
            }
        }
    }
    pub fn is_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(user_id))
            .filter(schema::user_visible_perms::target_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(self.user_id))
            .filter(schema::friends::target_id.eq(user_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        // подписан ли user_id на self
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_followers_user_with_id(&self, user_id: i32) -> bool {
        // подписан ли self на user_id
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(user_id))
            .filter(schema::follows::user_id.eq(self.user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }

    pub fn is_anon_user_see_community(&self) -> bool {
        return self.see_community == 1;
    }
    pub fn follow_user(&self, user_id: i32) -> bool {
        if self.user_id == user_id || self.is_self_user_in_block(user_id) || self.is_followers_user_with_id(user_id) || self.is_following_user_with_id(user_id) {
            return false;
        }

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);
        if new_follow.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn unfollow_user(&self, user_id: i32) -> bool {
        if self.user_id == user_id || !self.is_following_user_with_id(user_id) {
            return false;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        if follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .first::<Follow>(&_connection).is_ok() {
                let del = diesel::delete (
                    follows
                        .filter(schema::follows::target_id.eq(user_id))
                        .filter(schema::follows::user_id.eq(self.user_id))
                )
                .execute(&_connection);

            if del.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }

    pub fn frend_user(&self, user_id: i32) -> bool {
        // тут друзья создаются всего в одном экземпляре, где
        // self.user_id - это id создающего, а user_id -
        // id создаваемого. Это нужно для фильтрации приватности по
        // друзьям.
        if self.user_id == user_id || !self.is_followers_user_with_id(user_id) {
            return false;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let new_friend = diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .execute(&_connection);

        let del = diesel::delete (
            follows
                .filter(schema::follows::user_id.eq(user_id))
                .or_filter(schema::follows::target_id.eq(self.user_id))
                .filter(schema::follows::target_id.eq(user_id))
                .or_filter(schema::follows::user_id.eq(self.user_id))
            )
            .execute(&_connection);

        if del.is_ok() && new_friend.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn unfrend_user(&self, user_id: i32) -> bool {
        if self.user_id == user_id || !self.is_connected_with_user_with_id(user_id) {
            return false;
        }
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();

        let del = diesel::delete (
            friends
                .filter(schema::friends::user_id.eq(self.user_id))
                .filter(schema::friends::target_id.eq(user_id))
            )
            .execute(&_connection);

        let _new_follow = NewFollow {
            user_id:   user_id,
            target_id: self.user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);

        if del.is_ok() && new_follow.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn block_user(&self, user_id: i32) -> bool {
        if self.user_id == user_id || self.is_user_in_block(user_id) {
            return false;
        }
        let _connection = establish_connection();

        if self.is_connected_with_user_with_id(user_id) {
            use crate::schema::friends::dsl::friends;
            diesel::delete (
                friends
                    .filter(schema::friends::user_id.eq(self.user_id))
                    .filter(schema::friends::target_id.eq(user_id)))
                    .execute(&_connection)
                    .expect("E");
        }
        else if self.is_followers_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete (
                follows
                    .filter(schema::follows::target_id.eq(self.user_id))
                    .filter(schema::follows::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else if self.is_following_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete (
                follows
                    .filter(schema::follows::user_id.eq(self.user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }

        let _user_block = NewUserVisiblePerm {
            user_id:   self.user_id,
            target_id: user_id,
            types:     20,
        };
        diesel::insert_into(schema::user_visible_perms::table)
            .values(&_user_block)
            .get_result::<UserVisiblePerm>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn unblock_user(&self, user_id: i32) -> bool {
        if self.user_id == user_id || !self.is_user_in_block(user_id) {
            return false;
        }
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let del = diesel::delete (
            user_visible_perms
                .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                .filter(schema::user_visible_perms::target_id.eq(user_id))
            )
            .execute(&_connection);

        if del.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn get_gender_a(&self) -> String {
        if self.is_man {
            return "".to_string();
        }
        return "a".to_string();
    }
    pub fn get_gender(&self) -> String {
        if self.is_man {
            return "Мужской".to_string();
        }
        return "Женский".to_string();
    }
}

/////// Friend //////
// id друзей пользователя, для приватности
// записываем id пользователей основного сервиса пользователей.
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Friend {
    pub id:        i32,
    pub user_id:   i32, // кто
    pub target_id: i32, // на кого
}
#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// Follow //////
// id подписчиков пользователя, для приватности
// записываем id пользователей основного сервиса пользователей.
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Follow {
    pub id:        i32,
    pub user_id:   i32, // кто
    pub target_id: i32, // на кого
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:   i32,
    pub target_id: i32,
}

// включения и исключения для пользователей касательно конкретного пользоватетеля
// Связь - с пользователями сервиса пользователей, так как
// могут сюда внести того, кого в этом сервисе нет.
// ведь запрос передается либо для анонима, либо с id основного
// пользоваетля.

// 0 может видеть профиль
// 1 может видеть сообщества
// 10 не может видеть профиль
// 11 не может видеть сообщества
// 20 пользователь заблокирован у владельца блока сообществ

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}
