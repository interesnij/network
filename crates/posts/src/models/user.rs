use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use crate::schema;
use crate::schema::{
    users,
    friends,
    follows,
    user_visible_perms,
};
use actix_web::web::Json;

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


#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:             i32,
    pub user_id:        i32,
    pub first_name:     String,
    pub last_name:      String,
    pub types:          i16,
    pub is_man:         bool,
    pub link:           String,
    pub s_avatar:       Option<String>,
    pub last_activity:  chrono::NaiveDateTime,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_id:        i32,
    pub first_name:     String,
    pub last_name:      String,
    pub types:          i16,
    pub is_man:         bool,
    pub link:           String,
    pub s_avatar:       Option<String>,
    pub last_activity:  chrono::NaiveDateTime,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
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
    pub friends:    Option<Vec<i32>>,  // список id друзей пользователя
    pub follows:    Option<Vec<i32>>,  // список id подписчтков пользователя
}

impl User {
    pub fn create_user(user: NewUserJson) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if users
            .filter(schema::users::user_id.eq(user.user_id))
            .limit(1)
            .select(schema::users::id)
            .load::<User>(&_connection)
            .expect("E")
            .len() == 0 {
                return users
                    .filter(schema::users::user_id.eq(user.user_id))
                    .limit(1)
                    .select(schema::users::id)
                    .load::<i32>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();
        }
        let new_form = NewUser {
            user_id:        user.user_id,
            first_name:     user.first_name,
            last_name:      user.last_name,
            types:          user.types,
            is_man:         user.is_man,
            link:           user.link,
            s_avatar:       user.s_avatar,
            last_activity:  chrono::Local::now().naive_utc(),
            see_el:         1,
            see_comment:    1,
            create_el:      12,
            create_comment: 12,
            copy_el:        1,
            lists:          0,
            posts:          0,
            comments:       0,
        };
        let new_user = diesel::insert_into(schema::users::table)
            .values(&new_form)
            .get_result::<User>(&_connection)
            .expect("Error.");

        let new_user_id = user.user_id;

        if user.friends.is_some() {
            use crate::schema::friends::dsl::friends;

            for user_id in user.friends.unwrap() {
                if friends
                    .filter(schema::friends::user_id.eq(new_user_id))
                    .filter(schema::friends::target_id.eq(user_id))
                    .limit(1)
                    .select(schema::friends::id)
                    .load::<i32>(&_connection)
                    .expect("E")
                    .len() == 0 {
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
        if user.follows.is_some() {
            use crate::schema::follows::dsl::follows;

            for user_id in user.follows.unwrap() {
                if follows
                    .filter(schema::follows::user_id.eq(new_user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                    .limit(1)
                    .select(schema::follows::id)
                    .load::<i32>(&_connection)
                    .expect("E")
                    .len() == 0 {
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

    pub fn get_see_el_exclude_friends_ids(&self) -> Vec<i32> {
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
    pub fn get_see_el_include_friends_ids(&self) -> Vec<i32> {
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
    pub fn get_see_el_exclude_follows_ids(&self) -> Vec<i32> {
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
    pub fn get_see_el_include_follows_ids(&self) -> Vec<i32> {
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

    pub fn get_see_comment_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_comment_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_comment_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_comment_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_el_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_el_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_el_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_el_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_comment_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(14))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_comment_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(4))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_comment_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(14))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_create_comment_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(4))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_copy_el_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(15))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_copy_el_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(5))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_copy_el_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(15))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_copy_el_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(5))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }

    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
            3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            7 => self.get_friends_ids().iter().any(|&i| i==user_id),
            8 => !self.get_see_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            9 => self.get_see_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            10 => !self.get_see_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            11 => self.get_see_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_see_comment(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
            3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_comment_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_comment_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_comment_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_comment_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            7 => self.get_friends_ids().iter().any(|&i| i==user_id),
            8 => !self.get_see_comment_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            9 => self.get_see_comment_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            10 => !self.get_see_comment_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            11 => self.get_see_comment_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_create_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
            3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_create_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_create_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_create_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_create_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            7 => self.get_friends_ids().iter().any(|&i| i==user_id),
            8 => !self.get_create_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            9 => self.get_create_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            10 => !self.get_create_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            11 => self.get_create_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_create_comment(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
            3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_create_comment_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_create_comment_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_create_comment_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_create_comment_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            7 => self.get_friends_ids().iter().any(|&i| i==user_id),
            8 => !self.get_create_comment_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            9 => self.get_create_comment_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            10 => !self.get_create_comment_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            11 => self.get_create_comment_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_copy_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
            3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_copy_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_copy_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
            5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_copy_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_copy_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
            7 => self.get_friends_ids().iter().any(|&i| i==user_id),
            8 => !self.get_copy_el_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            9 => self.get_copy_el_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
            10 => !self.get_copy_el_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            11 => self.get_copy_el_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1;
    }
    pub fn is_anon_user_see_comment(&self) -> bool {
        return self.see_comment == 1;
    }
    pub fn is_anon_user_create_el(&self) -> bool {
        return self.create_el == 1;
    }
    pub fn is_anon_user_create_comment(&self) -> bool {
        return self.create_comment == 1;
    }
    pub fn is_anon_user_copy_el(&self) -> bool {
        return self.copy_el == 1;
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
        use crate::models::{UserVisiblePerm, NewUserVisiblePerm};
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
        let previous_user_list_delete = match types {
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
            2 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(12))
                )
                .execute(&_connection)
                .expect("E"),
            12 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(2))
                )
                .execute(&_connection)
                .expect("E"),
            3 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(13))
                )
                .execute(&_connection)
                .expect("E"),
            13 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(3))
                )
                .execute(&_connection)
                .expect("E"),
            4 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(14))
                )
                .execute(&_connection)
                .expect("E"),
            14 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(4))
                )
                .execute(&_connection)
                .expect("E"),
            5 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(15))
                )
                .execute(&_connection)
                .expect("E"),
            15 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(5))
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

    pub fn delete_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let _case = match user_types {
            1 => 11,
            6 => 16,
            7 => 17,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(_case))
            .get_result::<User>(&_connection)
            .expect("E");

        //hide_wall_notify_items(1, self.id);
    }
    pub fn restore_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            11 => 1,
            16 => 6,
            17 => 7,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .get_result::<User>(&_connection)
            .expect("E");
        //show_wall_notify_items(1, self.id);
    }
    pub fn add_new_community_subscriber (&self, community_id: i32) -> () {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::community_id.eq(community_id))
            .select(schema::news_user_communities::id)
            .load::<i32>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNewsUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: None,
                    community_id: Some(community_id),
                    mute: false,
                    sleep: None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
        }
    }

    pub fn add_notification_community_subscriber (&self, community_id: i32) -> () {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::community_id.eq(community_id))
            .select(schema::notify_user_communities::id)
            .load::<i32>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNotifyUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: None,
                    community_id: Some(community_id),
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .get_result::<NotifyUserCommunitie>(&_connection)
                    .expect("Error.");
        }
    }
    pub fn add_new_user_subscriber(&self, user: &User) -> () {
        use crate::models::{NewsUserCommunitie, NewNewsUserCommunitie};
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::user_id.eq(user.id))
            .select(schema::news_user_communities::id)
            .load::<i32>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNewsUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: Some(user.id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
        }
    }
    pub fn add_new_subscriber_in_list(&self, new_id: i32, list_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .load::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _new.len() > 0 && _new[0].owner == self.id && _list.len() > 0 && _list[0].owner == self.id {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(list_id))
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber(&self, new_id: i32) -> bool {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E");
        if _new.len() > 0 && _new[0].owner == self.id {
            diesel::delete(
                news_user_communities
                    .filter(schema::news_user_communities::id.eq(new_id))
                )
                .execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber_from_list(&self, new_id: i32) -> () {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities.filter(schema::news_user_communities::id.eq(new_id)).load::<NewsUserCommunitie>(&_connection).expect("E");
        let null_value: Option<i32> = None;

        if _new.len() > 0 && _new[0].owner == self.id {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(null_value))
                .get_result::<NewsUserCommunitie>(&_connection)
                .expect("Error.");
            }
    }

    pub fn add_notification_user_subscriber(&self, user: &User) -> () {
        use crate::models::{NotifyUserCommunitie, NewNotifyUserCommunitie};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.id))
            .filter(schema::notify_user_communities::user_id.eq(user.id))
            .select(schema::notify_user_communities::id)
            .load::<i32>(&_connection)
            .expect("E").len() == 0 {
                let _new = NewNotifyUserCommunitie {
                    owner: self.id,
                    list_id: None,
                    user_id: Some(user.id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .get_result::<NotifyUserCommunitie>(&_connection)
                    .expect("Error.");
        }
    }
    pub fn add_notification_subscriber_in_list(&self, notify_id: i32, list_id: i32) -> () {
        use crate::models::{NotifyUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        let _list = list_user_communities_keys.filter(schema::list_user_communities_keys::id.eq(list_id)).load::<ListUserCommunitiesKey>(&_connection).expect("E");

        if _notify.len() > 0 && _notify[0].owner == self.id && _list.len() > 0 && _list[0].owner == self.id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(_list[0].id))
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
        }
    }
    pub fn delete_notification_subscriber(&self, notify_id: i32) -> () {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::id.eq(notify_id))
            .load::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        if _notify.len() > 0 && _notify[0].owner == self.id {
            diesel::delete(
                notify_user_communities
                    .filter(schema::notify_user_communities::id.eq(notify_id))
                )
                .execute(&_connection)
                .expect("E");
        }
    }
    pub fn delete_notification_subscriber_from_list(&self, notify_id: i32) -> () {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)).load::<NotifyUserCommunitie>(&_connection).expect("E");
        let null_value: Option<i32> = None;
        if _notify.len() > 0 && _notify[0].owner == self.id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(null_value))
                .get_result::<NotifyUserCommunitie>(&_connection)
                .expect("Error.");
            }
    }

    pub fn plus_lists(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::lists.eq(self.lists + count))
            .get_result::<User>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_lists(&self, count: i32) -> bool {
        if self.lists > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::lists.eq(self.lists - count))
                .get_result::<User>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_posts(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::posts.eq(self.posts + count))
            .get_result::<User>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_posts(&self, count: i32) -> bool {
        if self.posts > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::posts.eq(self.posts - count))
                .get_result::<User>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::comments.eq(self.comments + count))
            .get_result::<User>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        if self.comments > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::comments.eq(self.comments - count))
                .get_result::<User>(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }

    pub fn count_lists(&self) -> i32 {
        return self.lists;
    }
    pub fn count_lists_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_lists(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
        );
    }
    pub fn count_lists_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_lists(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
        );
    }

    pub fn count_posts(&self) -> i32 {
        return self.posts;
    }
    pub fn count_posts_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }
    pub fn count_posts_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn count_comments(&self) -> i32 {
        return self.comments;
    }
    pub fn count_comments_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_comments(),
            " комментарий".to_string(),
            " комментария".to_string(),
            " комментариев".to_string(),
        );
    }
    pub fn count_comments_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_comments(),
            " комментарий".to_string(),
            " комментария".to_string(),
            " комментариев".to_string(),
        );
    }

    pub fn get_ids_for_featured_news(&self) -> (Vec<i32>, Vec<i32>) {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let news = featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.id))
            .filter(schema::featured_user_communities::mute.eq(false))
            .filter(schema::featured_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
        let mut users_stack = Vec::new();
        let mut communities_stack = Vec::new();
        for i in news.iter() {
            if i.community_id.is_some() {
                communities_stack.push(i.community_id.unwrap());
            }
            else {
                users_stack.push(i.user_id.unwrap());
            }
        }
        return (users_stack, communities_stack);
    }
    pub fn get_ids_for_main_news(&self) -> (Vec<i32>, Vec<i32>) {
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::models::NewsUserCommunitie;

        let _connection = establish_connection();
        let news = news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.id))
            .filter(schema::news_user_communities::mute.eq(false))
            .filter(schema::news_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E.");
        let mut users_stack = Vec::new();
        let mut communities_stack = Vec::new();
        for i in news.iter() {
            if i.community_id.is_some() {
                communities_stack.push(i.community_id.unwrap());
            }
            else {
                users_stack.push(i.user_id.unwrap());
            }
        }
        return (users_stack, communities_stack);
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
        return self.types == 7;
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
            .limit(1)
            .select(schema::user_visible_perms::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(user_id))
            .filter(schema::user_visible_perms::target_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .limit(1)
            .select(schema::user_visible_perms::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_id.eq(self.user_id))
            .limit(1)
            .select(schema::friends::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .limit(1)
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.").len() > 0;
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
            .limit(1)
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }

    pub fn follow_user(&self, user_id: i32) -> () {
        if self.user_id == user_id || self.is_self_user_in_block(user_id) || self.is_followers_user_with_id(user_id) || self.is_following_user_with_id(user_id) {
            return;
        }
        use crate::models::NewFollow;

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id:   self.user_id,
            target_id: user_id,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");
    }
    pub fn unfollow_user(&self, user_id: i32) -> () {
        if self.user_id == user_id || !self.is_following_user_with_id(user_id) {
            return;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follows = follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .load::<Follow>(&_connection)
            .expect("E");
        if _follows.len() > 0 {
            diesel::delete (
                    follows
                        .filter(schema::follows::target_id.eq(user_id))
                        .filter(schema::follows::user_id.eq(self.user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
    }

    pub fn frend_user(&self, user_id: i32) -> () {
        // тут друзья создаются всего в одном экземпляре, где
        // self.user_id - это id создающего, а user_id -
        // id создаваемого. Это нужно для фильтрации приватности по
        // друзьям.
        if self.user_id == user_id || !self.is_followers_user_with_id(user_id) {
            return;
        }
        use crate::models::NewFriend;
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id:   self.user_id,
            target_id: user_id,
        };
        diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .get_result::<Friend>(&_connection)
            .expect("Error.");

        diesel::delete (
            follows
                .filter(schema::follows::user_id.eq(user_id))
                .or_filter(schema::follows::target_id.eq(self.user_id))
                .filter(schema::follows::target_id.eq(user_id))
                .or_filter(schema::follows::user_id.eq(self.user_id))
            )
            .execute(&_connection)
            .expect("E");
    }
    pub fn unfrend_user(&self, user_id: i32) -> () {
        if self.user_id == user_id || !self.is_connected_with_user_with_id(user_id) {
            return;
        }
        use crate::models::NewFollow;
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();

        diesel::delete (
            friends
                .filter(schema::friends::user_id.eq(self.user_id))
                .filter(schema::friends::target_id.eq(user_id))
            )
            .execute(&_connection)
            .expect("E");

        let _new_follow = NewFollow {
            user_id:   user_id,
            target_id: self.user_id,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");
    }
    pub fn block_user(&self, user_id: i32) -> () {
        if self.user_id == user_id || self.is_user_in_block(user_id) {
            return;
        }
        //use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::NewUserBlock;

        let _connection = establish_connection();

        if self.is_connected_with_user_with_id(user_id) {
            use crate::schema::friends::dsl::friends;
            diesel::delete (
                friends
                    .filter(schema::friends::user_id.eq(self.user_id))
                    .filter(schema::friends::target_id.eq(user_id))
                )
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
            diesel::delete(
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
    }
    pub fn unblock_user(&self, user_id: i32) -> () {
        if self.user_id == user_id || !self.is_user_in_block(user_id) {
            return;
        }
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        diesel::delete (
            user_visible_perms
                .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                .filter(schema::user_visible_perms::target_id.eq(user_id)))
                .execute(&_connection)
                .expect("E");
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
    pub user_id:   i32,
    pub target_id: i32,
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
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:   i32,
    pub target_id: i32,
}

// UserVisiblePerm
// types
// 1 может видеть записи
// 2 может видеть комменты к записям
// 3 может создавать записи
// 4 может создавать комменты к записям
// 5 может копировать списки / записи
// 11 не может видеть записи
// 12 не может видеть комменты к записям
// 13 не может создавать записи
// 14 не может создавать комменты к записям
// 15 не может копировать списки / записи
// 20 пользователь заблокирован у владельца записей
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:        i32,
    pub user_id:   i32, // какой пользователь добавляет
    pub target_id: i32, // кокого пользователя добавляет
    pub types:     i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}
