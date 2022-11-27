use serde::{Serialize, Deserialize};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    QueryDsl,
    PgTextExpressionMethods,
    ExpressionMethods,
};
use crate::schema;
use crate::models::{
    UserLocation,
    UserInfo,
    UserPrivate,
    TokenDetailJson, TokenJson,
};
use crate::utils::{
    establish_connection, get_limit_offset,
    UserPrivateJson, LocationJson, KeyWalue,
    CardUserJson, UserDetailJson, EditPrivateResp, 

};
use crate::schema::users;
use crate::errors::Error;


///// Типы пользоватетеля
    // 1 стандартный тип пользователя
    // 6 пославший запрос на идентификацию
    // 7 идентифицированный

    // 10 TRAINEE_MODERATOR
    // 11 MODERATOR
    // 12 HIGH_MODERATOR
    // 13 TEAMLEAD_MODERATOR
    // 14 TRAINEE_MANAGER
    // 15 MANAGER
    // 16 HIGH_MANAGER
    // 17 TEAMLEAD_MANAGER
    // 18 ADVERTISER
    // 19 HIGH_ADVERTISER
    // 20 TEAMLEAD_ADVERTISER
    // 21 ADMINISTRATOR
    // 22 HIGH_ADMINISTRATOR
    // 23 TEAMLEAD_ADMINISTRATOR
    // 25 SUPERMANAGER

    // 31 удаленный стандартный
    // 36 удаленный пославший запрос на идентификацию
    // 37 удаленный идентифицированный

    // 41 закрытый стандартный
    // 46 закрытый пославший запрос на идентификацию
    // 47 закрытый идентифицированный

    // 51 приостановленный стандартный
    // 56 приостановленный пославший запрос на идентификацию
    // 57 приостановленный идентифицированный

    // 61 закрытый баннером стандартный
    // 66 закрытый баннером пославший запрос на идентификацию
    // 67 закрытый баннером идентифицированный


#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub is_man:        bool,
    pub password:      String,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub is_man:        bool,
    pub password:      String,
    pub link:          String,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserSignup {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub is_man:        bool,
    pub password:      String,
    pub link:          String,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub phone:    String,
    pub password: String,
}

#[derive(Serialize)]
pub struct GetSessionFields {
    pub id:       i32,
    pub phone:    String,
    pub password: String,
}

impl User { 
    pub fn edit_private (
        &self, 
        field:     &str, 
        value:     i16, 
        users_ids: Option<Vec<i32>>
    ) -> i16 {
        let users_vec = vec![3,4,5,6,9,10,11,12];
        if field != "see_all" && field != "see_all" && field != "see_friend" {
            return 0;
        }
        else if value < 1 || value > 13 {
            return 0;
        }
        else if users_vec.iter().any(|&i| i==value) && users.is_none() {
            return 0;
        }
        let _connection = establish_connection();
        let private = self.get_private_model().expect("E.");
        if field == "see_all" {
            diesel::update(&private)
            .set(schema::users::see_all.eq(value))
            .execute(&_connection)
            .expect("E.");
        }
        else if field == "see_info" {
            diesel::update(&private)
            .set(schema::users::see_info.eq(value))
            .execute(&_connection)
            .expect("E.");
        }
        else if field == "see_friend" {
            diesel::update(&private)
            .set(schema::users::see_friend.eq(value))
            .execute(&_connection)
            .expect("E.");
        }
        // нужно удалить из списка тех, кто был туда внесен
        // с противоположными правами.
        match value {
        1 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(11))
            )
            .execute(&_connection)
            .expect("E"),
        2 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(12))
            )
            .execute(&_connection)
            .expect("E"),
        3 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(13))
            )
            .execute(&_connection)
            .expect("E"),
        11 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(1))
            )
            .execute(&_connection)
            .expect("E"),
        12 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(2))
            )
            .execute(&_connection)
            .expect("E"),
        13 => diesel::delete (
                user_visible_perms
                    .filter(schema::user_visible_perms::user_id.eq(self.id))
                    .filter(schema::user_visible_perms::types.eq(3))
            )
            .execute(&_connection)
            .expect("E"),
        _ => 0,
        };
        for user_id in users_ids.clone() {
            let _new_perm = NewUserVisiblePerm {
                user_id:   self.id,
                target_id: *user_id,
                types:     value,
            };
            diesel::insert_into(schema::user_visible_perms::table)
                .values(&_new_perm)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn edit_name(&self, first_name: &str, last_name: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(( 
                schema::users::first_name.eq(first_name),
                schema::users::last_name.eq(last_name)
            ))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_phone(&self, phone: &str) -> i16 {
        use crate::schema::phone_codes::dsl::phone_codes;
        use chrono::Duration;
        
        let _connection = establish_connection();
        if phone_codes
            .filter(schema::phone_codes::phone.eq(phone))
            .filter(schema::phone_codes::types.eq(2))
            .filter(schema::phone_codes::created.gt(chrono::Local::now().naive_utc() - Duration::hours(1)))
            .select(schema::phone_codes::id)
            .first::<i32>(&_connection)
            .is_ok() {
            
            let _o = diesel::update(self)
                .set(schema::users::phone.eq(phone))
                .execute(&_connection)
                .expect("E.");
            return 1;
        }
        else {
            return 0;
        }
    } 
    pub fn edit_link(&self, link: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::link.eq(link))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn get_private_field(value: i16) -> KeyWalue {
        let info = match value {
            1 => "Все пользователи",
            2 => "Все друзья и все подписчики",
            3 => "Все друзья и подписчики, кроме",
            4 => "Все друзья и некоторые подписчики",
            5 => "Все подписчики и друзья, кроме",
            6 => "Все подписчики и некоторые друзья",
            7 => "Все друзья",
            8 => "Все подписчики",
            9 => "Друзья, кроме",
            10 => "Некоторые друзья",
            11 => "Подписчики, кроме",
            12 => "Некоторые подписчики",
            13 => "Только я",
            _ => "Ошибка",
        };
        return KeyWalue {
            value: value,
            info:  info.to_string(),
        }
    }

    pub fn get_private_json(&self) -> EditPrivateResp {
        let see_all_exclude_friends:    Option<Vec<CardUserJson>>;
        let see_all_exclude_follows:    Option<Vec<CardUserJson>>;
        let see_all_include_friends:    Option<Vec<CardUserJson>>;
        let see_all_include_follows:    Option<Vec<CardUserJson>>;
        let see_info_exclude_friends:   Option<Vec<CardUserJson>>;
        let see_info_exclude_follows:   Option<Vec<CardUserJson>>;
        let see_info_include_friends:   Option<Vec<CardUserJson>>;
        let see_info_include_follows:   Option<Vec<CardUserJson>>;
        let see_friend_exclude_friends: Option<Vec<CardUserJson>>;
        let see_friend_exclude_follows: Option<Vec<CardUserJson>>;
        let see_friend_include_friends: Option<Vec<CardUserJson>>;
        let see_friend_include_follows: Option<Vec<CardUserJson>>;

        let private = self.get_private_model().expect("E.");
        
        if private.see_all == 5 || private.see_all == 9 {
            see_all_exclude_friends = Some(self.get_limit_see_all_exclude_friends(Some(20), Some(0)));
        }
        else {
            see_all_exclude_friends = None;
        }
        if private.see_all == 3 || private.see_all == 11 {
            see_all_exclude_follows = Some(self.get_limit_see_all_exclude_follows(Some(20), Some(0)));
        }
        else {
            see_all_exclude_follows = None;
        }
        if private.see_all == 6 || private.see_all == 10 {
            see_all_include_friends = Some(self.get_limit_see_all_include_friends(Some(20), Some(0)));
        }
        else {
            see_all_include_friends = None;
        }
        if private.see_all == 4 || private.see_all == 12 {
            see_all_include_follows = Some(self.get_limit_see_all_include_follows(Some(20), Some(0)));
        }
        else {
            see_all_include_follows = None;
        }

        if private.see_info == 5 || private.see_info == 9 {
            see_info_exclude_friends = Some(self.get_limit_see_info_exclude_friends(Some(20), Some(0)));
        }
        else {
            see_info_exclude_friends = None;
        }
        if private.see_info == 3 || private.see_info == 11 {
            see_info_exclude_follows = Some(self.get_limit_see_info_exclude_follows(Some(20), Some(0)));
        }
        else {
            see_info_exclude_follows = None;
        }
        if private.see_info == 6 || private.see_info == 10 {
            see_info_include_friends = Some(self.get_limit_see_info_include_friends(Some(20), Some(0)));
        }
        else {
            see_info_include_friends = None;
        }
        if private.see_info == 4 || private.see_info == 12 {
            see_info_include_follows = Some(self.get_limit_see_info_include_follows(Some(20), Some(0)));
        }
        else {
            see_info_include_follows = None;
        }

        if private.see_friend == 5 || private.see_friend == 9 {
            see_friend_exclude_friends = Some(self.get_limit_see_friend_exclude_friends(Some(20), Some(0)));
        }
        else {
            see_friend_exclude_friends = None;
        }
        if private.see_friend == 3 || private.see_friend == 11 {
            see_friend_exclude_follows = Some(self.get_limit_see_friend_exclude_follows(Some(20), Some(0)));
        }
        else {
            see_friend_exclude_follows = None;
        }
        if private.see_friend == 6 || private.see_friend == 10 {
            see_friend_include_friends = Some(self.get_limit_see_friend_include_friends(Some(20), Some(0)));
        }
        else {
            see_friend_include_friends = None;
        }
        if private.see_friend == 4 || private.see_friend == 12 {
            see_friend_include_follows = Some(self.get_limit_see_friend_include_follows(Some(20), Some(0)));
        }
        else {
            see_friend_include_follows = None;
        }
    
        return EditPrivateResp {
            see_all:                    User::get_private_field(private.see_all),
            see_info:                   User::get_private_field(private.see_info),
            see_friend:                 User::get_private_field(private.see_friend),
            see_all_exclude_friends:    see_all_exclude_friends,
            see_all_exclude_follows:    see_all_exclude_follows,
            see_all_include_friends:    see_all_include_friends,
            see_all_include_follows:    see_all_include_follows,
            see_info_exclude_friends:   see_info_exclude_friends,
            see_info_exclude_follows:   see_info_exclude_follows,
            see_info_include_friends:   see_info_include_friends,
            see_info_include_follows:   see_info_include_follows,
            see_friend_exclude_friends: see_friend_exclude_friends,
            see_friend_exclude_follows: see_friend_exclude_follows,
            see_friend_include_friends: see_friend_include_friends,
            see_friend_include_follows: see_friend_include_follows,
        };
    }
    pub fn edit_password (
        &self, 
        old_password: &str,
        new_password: &str,
    ) -> i16 {
        use bcrypt::hash;

        let old = hash(old_password, 8).unwrap();
        let new = hash(new_password, 8).unwrap();
        if self.password == old && old != new {
            let _connection = establish_connection();
            let _o = diesel::update(self)
                .set(schema::users::password.eq(new))
                .execute(&_connection)
                .expect("E.");
            return 1;
        }
        else {
            return 0;
        }
    }

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
    pub fn get_user_by_phone(phone: &String) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::phone.eq(phone))
            .filter(schema::users::types.lt(30))
            .first::<User>(&_connection)?);
    }
    pub fn get_user_detail_json(&self) -> UserDetailJson {
        let city: Option<String>;
        let status: Option<String>; 
        let image: Option<String>;
        let mut _b = "".to_string();

        let info = self.get_info_model();
        match info {
          Ok(_ok) => {
              city = _ok.city;
              status = _ok.status;
              image = _ok.b_avatar;
              if _ok.birthday.is_some() {
                  _b = _ok.birthday.unwrap().format("%d-%m-%Y").to_string();
              }
          },
          Err(_error) => {
              city = None;
              status = None;
              image = None;
          },
        };
        let user_json = UserDetailJson {
             id:            self.id, 
             first_name:    self.first_name.clone(),
             last_name:     self.last_name.clone(),
             is_man:        self.is_man.clone(),
             city:          city,
             status:        status,
             image:         image,
             birthday:      _b,
             last_activity: self.last_activity.format("%d-%m-%Y в %H:%M").to_string(),
         };
         return user_json;
    }
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }

    pub fn count_users() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }

    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.get_full_name() + &"</a>".to_string();
    }
    pub fn is_user(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "use".to_string() + &self.get_str_id();
    }
    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let user_types = self.types;
        let _case = match user_types {
            1 => 41,
            6 => 46,
            7 => 47,
            _ => 41,
        };
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
    pub fn unclose_item(&self) -> i16 {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            41 => 1,
            46 => 6,
            47 => 7,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .execute(&_connection);
        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn suspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let user_types = self.types;
        let _case = match user_types {
            1 => 51,
            6 => 56,
            7 => 57,
            _ => 51,
        };
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
    pub fn unsuspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            51 => 1,
            56 => 6,
            57 => 7,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(close_case))
            .execute(&_connection);
        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn get_last_location_json(&self) -> Result<LocationJson, Error> {
        use crate::schema::user_locations::dsl::user_locations;

        let _connection = establish_connection();
        let location = user_locations
            .filter(schema::user_locations::user_id.eq(self.id))
            .order(schema::user_locations::id.desc())
            .first::<UserLocation>(&_connection);

        return match location {
             Ok(_ok) => Ok(LocationJson {
                 city_ru:    _ok.city_ru,
                 region_ru:  _ok.region_ru,
                 country_ru: _ok.country_ru,
             }),
              Err(_error) => Ok(LocationJson {
                  city_ru:    None,
                  region_ru:  None,
                  country_ru: None,
              }),
        };
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

    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_id.eq(user_id))
            .filter(schema::user_blocks::target_id.eq(self.id))
            .select(schema::user_blocks::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .filter(schema::user_blocks::target_id.eq(user_id))
            .select(schema::user_blocks::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(self.id))
            .filter(schema::friends::target_id.eq(user_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection).is_ok();
    }

    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_followers_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(user_id))
            .filter(schema::follows::user_id.eq(self.id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_followers_user_view(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .filter(schema::follows::view.eq(true))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn get_info_model(&self) -> Result<UserInfo, Error> {
        let profile = self.find_info_model();
        if profile.is_ok() {
            return profile;
        }
        else {
            return self.create_info_model();
        }
    }
    pub fn find_info_model(&self) -> Result<UserInfo, Error> {
        use crate::schema::user_infos::dsl::user_infos;

        let _connection = establish_connection();
        let info = user_infos
            .filter(schema::user_infos::user_id.eq(self.id))
            .first(&_connection)?;
        return Ok(info);
    }
    pub fn create_info_model(&self) -> Result<UserInfo, Error> {
        let _connection = establish_connection();
        use crate::models::NewUserInfo;

        let _user_info = NewUserInfo {
            user_id:   self.id,
            avatar_id: None,
            language:  "Ru".to_string(),
            email:     None,
            birthday:  None,
            b_avatar:  None,
            status:    None,
            city:      None,
            level:     100,
            cover:     None,
            created:   chrono::Local::now().naive_utc(),
            friends:   0,
            follows:   0,
        };
        let new_info = diesel::insert_into(schema::user_infos::table)
            .values(&_user_info)
            .get_result::<UserInfo>(&_connection)?;
        return Ok(new_info);
    }

    pub fn is_have_followers(&self) -> bool {
        let profile = self.get_info_model();
        return match profile {
          Ok(_ok) => _ok.follows > 0,
          Err(_error) => false,
        };
    }
    pub fn is_have_followings(&self) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let ok = follows
            .filter(schema::follows::user_id.eq(self.id))
            .select(schema::follows::id)
            .first::<i32>(&_connection);
        if ok.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn is_have_blacklist(&self) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        let ok = user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .select(schema::user_blocks::id)
            .first::<i32>(&_connection);
        if ok.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn is_have_friends(&self) -> bool {
        let profile = self.get_info_model();
        return match profile {
          Ok(_ok) => _ok.friends > 0,
          Err(_error) => false,
        };
    }

    pub fn count_no_view_followers(&self) -> usize {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.id))
            .filter(schema::follows::view.eq(false))
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn count_following(&self) -> usize {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn count_followers(&self) -> i32 {
        let profile = self.get_info_model();
        return match profile {
          Ok(_ok) => _ok.follows,
          Err(_error) => 0,
        };
    }
    pub fn count_followers_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_followers(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }
    pub fn count_followers_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_followers(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }

    pub fn count_blacklist(&self) -> usize {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .select(schema::user_blocks::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_blocked_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_blocks::dsl::user_blocks,
            users::dsl::users,
        };
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .order(schema::user_blocks::id.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_blocks::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        let blocked_users = users
            .filter(schema::users::id.eq_any(all_user_blocks))
            .filter(schema::users::types.lt(30))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return blocked_users;
    }
    pub fn search_blocked_users (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            user_blocks::dsl::user_blocks,
            users::dsl::users,
        };
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .order(schema::user_blocks::id.desc())
            .select(schema::user_blocks::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        let blocked_users = users
            .filter(schema::users::id.eq_any(all_user_blocks))
            .filter(schema::users::types.lt(30))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .limit(_limit)
            .offset(_offset)
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return blocked_users;
    }

    pub fn count_friends(&self) -> i32 {
        let profile = self.get_info_model();
        return match profile {
          Ok(_ok) => _ok.friends,
          Err(_error) => 0,
        };
    }
    pub fn count_friends_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_friends(),
            " друг".to_string(),
            " друга".to_string(),
            " друзей".to_string(),
        );
    }

    pub fn plus_follows(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();

        match profile {
          Ok(_ok) => diesel::update(&_ok)
              .set(schema::user_infos::follows.eq(_ok.follows + count))
              .execute(&_connection)
              .expect("Error."),
          Err(_error) => 0,
        };
    }
    pub fn plus_friends(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        match profile {
          Ok(_ok) => diesel::update(&_ok)
              .set(schema::user_infos::friends.eq(_ok.friends + count))
              .execute(&_connection)
              .expect("Error."),
          Err(_error) => 0,
        };
    }
    pub fn minus_follows(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        match profile {
            Ok(_ok) => {
                if _ok.follows > 0 {
                    diesel::update(&_ok)
                        .set(schema::user_infos::follows.eq(_ok.follows - count))
                        .execute(&_connection)
                        .expect("Error.");
                }
            },
            Err(_error) => (),
        };
    }
    pub fn minus_friends(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        match profile {
            Ok(_ok) => {
                if _ok.friends > 0 {
                    diesel::update(&_ok)
                        .set(schema::user_infos::friends.eq(_ok.friends - count))
                        .execute(&_connection)
                        .expect("Error.");
                }
            },
            Err(_error) => (),
        };
    }

    pub fn get_friends_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_follows_ids(&self) -> Vec<i32> {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follows = follows
            .filter(schema::follows::user_id.eq(self.id))
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _follows;
    }

    pub fn get_6_friends_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .order(schema::friends::visited.desc())
            .limit(6)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");

        return _friends;
    }

    pub fn get_featured_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            featured_friends::dsl::featured_friends,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);

        let _connection = establish_connection();
        let friend_ids = featured_friends
            .filter(schema::featured_friends::target_id.eq(self.id))
            .filter(schema::featured_friends::hidden.eq(false))
            .limit(_limit)
            .offset(_offset)
            .select(schema::featured_friends::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _friends = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }

    pub fn get_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _friends = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn search_friends (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _friends = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .limit(_limit)
            .offset(_offset)
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_6_friends(&self) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let _friends = users
            .filter(schema::users::id.eq_any(self.get_6_friends_ids()))
            .filter(schema::users::types.lt(30))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }

    pub fn get_online_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };
        use chrono::Duration;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");

        let _users = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_online_friends (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };
        use chrono::Duration;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");

        let _users = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn get_online_friends_count(&self) -> usize {
        return self.get_online_friends(Some(500), Some(0)).len();
    }
    pub fn get_6_online_friends(&self) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };
        use chrono::Duration;

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(6)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");

        let _users = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(30))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_followers(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::target_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(11))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_followers (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::target_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(11))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_6_followers(&self) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };

        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::target_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(6)
            .select(schema::follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(11))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn get_all_users_count(&self) -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(30))
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_users(limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let _users = users
            .filter(schema::users::types.lt(30))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_users (
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let _users = users
            .filter(schema::users::types.lt(30))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_followings(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            follows::dsl::follows,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::user_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(11))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_followings (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            follows::dsl::follows,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::user_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(11))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_common_friends_of_user(&self, user: &User, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        let _users = users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_common_friends_of_user (
        &self,
        q:      &String,
        user:   &User,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        let _users = users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_6_common_friends_of_user(&self, user: &User) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        let _users = users
            .filter(schema::users::id.eq_any(stack))
            .filter(schema::users::types.lt(11))
            .limit(6)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn count_common_friends_of_user(&self, user: &User) -> usize {
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        let mut stack = Vec::new();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                stack.push(int);
            }
        }
        return stack.len();
    }

    pub fn is_have_common_friends_of_user(&self, user: &User) -> bool {
        let _connection = establish_connection();
        let self_friends = self.get_friends_ids();
        let user_friends = user.get_friends_ids();
        for int in self_friends.iter() {
            if user_friends.iter().any(|i| i==int) {
                return true;
            }
        }
        return false;
    }

    pub fn get_see_all_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_all_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_see_info_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_info_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_info_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_info_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_friend_exclude_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_friend_exclude_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_friend_include_friends_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_see_friend_include_follows_ids(&self) -> Vec<i32> {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let items = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }


    ///////////////////////////////
    pub fn get_limit_see_all_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_all_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_all_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(11))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_all_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(1))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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

    pub fn get_limit_see_info_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_info_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_info_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(12))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_info_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(2))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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

    pub fn get_limit_see_friend_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_friend_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_friend_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(13))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    pub fn get_limit_see_friend_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(3))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E"); 
        
        return users
            .filter(schema::users::id.eq_any(items_ids))
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
    ///////////////////

    pub fn get_private_model(&self) -> Result<UserPrivate, Error> {
        let private = self.find_private_model();
        if private.is_ok() {
            return private;
        }
        else {
            return self.create_private_model();
        }
    }
    pub fn create_private_model(&self) -> Result<UserPrivate, Error> {
        use crate::models::NewUserPrivate;

        let _connection = establish_connection();
        let _new_private = NewUserPrivate {
            user_id:    self.id,
            see_all:    1,
            see_info:   1,
            see_friend: 1,
        };
        let _private = diesel::insert_into(schema::user_privates::table)
            .values(&_new_private)
            .get_result::<UserPrivate>(&_connection)?;

        return Ok(_private);
    }
    pub fn find_private_model(&self) -> Result<UserPrivate, Error> {
        use crate::schema::user_privates::dsl::user_privates;

        let _connection = establish_connection();
        let private = user_privates
            .filter(schema::user_privates::user_id.eq(self.id))
            .first(&_connection)?;
        return Ok(private);
    }

    pub fn get_private_model_json(&self) -> Result<UserPrivateJson, Error> {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => Ok(UserPrivateJson {
              see_all:    _ok.see_all,
              see_info:   _ok.see_info,
              see_friend: _ok.see_friend,
          }),
          Err(_error) => Err(_error),
        };
    }

    pub fn is_user_see_info(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_info {
              1 => true,
              2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
              3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
              4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
              5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
              6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
              7 => self.is_connected_with_user_with_id(user_id),
              8 => self.is_self_followers_user_with_id(user_id),
              9 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
              10 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
              11 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
              12 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
              _ => false,
          },
          Err(_) => false,
        };
    }
    pub fn is_user_see_all(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_all {
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
          },
          Err(_) => false,
        };
    }
    pub fn is_user_see_friend(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_friend {
              1 => true,
              2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
              3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_friend_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
              4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_friend_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
              5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_friend_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
              6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_friend_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
              7 => self.is_connected_with_user_with_id(user_id),
              8 => self.is_self_followers_user_with_id(user_id),
              9 => !self.get_see_friend_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
              10 => self.get_see_friend_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
              11 => !self.get_see_friend_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
              12 => self.get_see_friend_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
              _ => false,
          },
          Err(_) => false,
        };
    }

    pub fn get_profile_all_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == user_id {
            return vec![true, true, true];
        }
        let private = self.get_private_model();
        match private {
          Ok(_ok) => {
              let bool_see_all = match _ok.see_all {
                  1 => true,
                  2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
                  3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  7 => self.is_connected_with_user_with_id(user_id),
                  8 => self.is_self_followers_user_with_id(user_id),
                  9 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  10 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  11 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
                  12 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
                  _ => false,
              };
              if bool_see_all == false {
                  return vec![false, false, false];
              }
              let mut bool_stack = Vec::new();
              bool_stack.push(true);
              let bool_see_info = match _ok.see_info {
                  1 => true,
                  2 => self.is_connected_with_user_with_id(user_id) || self.is_followers_user_with_id(user_id),
                  3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  7 => self.is_connected_with_user_with_id(user_id),
                  8 => self.is_self_followers_user_with_id(user_id),
                  9 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  10 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  11 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
                  12 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id),
                  _ => false,
              };
              bool_stack.push(bool_see_info);

              let bool_see_friend = match _ok.see_friend {
                  1 => true,
                  2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
                  3 => self.is_connected_with_user_with_id(user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  4 => self.is_connected_with_user_with_id(user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_self_followers_user_with_id(user_id)),
                  5 => self.is_self_followers_user_with_id(user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  6 => self.is_self_followers_user_with_id(user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id)),
                  7 => self.is_connected_with_user_with_id(user_id),
                  8 => self.is_self_followers_user_with_id(user_id),
                  9 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  10 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.is_connected_with_user_with_id(user_id),
                  11 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.is_followers_user_with_id(user_id),
                  12 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.is_followers_user_with_id(user_id),
                  _ => false,
              };
              bool_stack.push(bool_see_friend);
              return bool_stack;

          },
          Err(_) => return vec![false, false, false],
        };
    }
    pub fn is_anon_user_see_all(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_all == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_friend(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_friend == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_info(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_info == 1,
            Err(_) => false,
        }
    }

    pub fn get_anon_profile_all_see(&self) -> Vec<bool> {
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();
        return match private {
            Ok(_ok) => {
                bool_stack.push(_ok.see_all == 1);
                bool_stack.push(_ok.see_info == 1);
                bool_stack.push(_ok.see_friend == 1);
                bool_stack
            },
            Err(_) => vec![false, false, false],
        }
    }
    pub fn set_user_visible_perms(&self, users: String, types: i16) -> i16 {
        use crate::models::NewUserVisiblePerm;
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
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(11))
                )
                .execute(&_connection)
                .expect("E"),
            2 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(12))
                )
                .execute(&_connection)
                .expect("E"),
            3 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(13))
                )
                .execute(&_connection)
                .expect("E"),
            11 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(1))
                )
                .execute(&_connection)
                .expect("E"),
            12 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(2))
                )
                .execute(&_connection)
                .expect("E"),
            13 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.id))
                        .filter(schema::user_visible_perms::types.eq(3))
                )
                .execute(&_connection)
                .expect("E"),
            _ => 0,
        };
        for user_id in users_ids.iter() {
            let _new_perm = NewUserVisiblePerm {
                user_id:   self.id,
                target_id: *user_id,
                types:     types,
            };
            diesel::insert_into(schema::user_visible_perms::table)
                .values(&_new_perm)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn get_image_or_null(&self) -> Option<String> {
        if self.s_avatar.is_some() {
            return self.s_avatar.clone();
        }
        else {
            return None;
        }
    }

    pub fn get_or_create_featured_friends(&self, user: User) -> () {
        use crate::schema::featured_friends::dsl::featured_friends;
        use crate::models::NewFeaturedFriend;

        let _connection = establish_connection();
        for _user in user.get_6_friends().iter() {
            if _user.id != self.id && !featured_friends
                .filter(schema::featured_friends::target_id.eq(self.id))
                .filter(schema::featured_friends::user_id.eq(_user.id))
                .select(schema::featured_friends::id)
                .first::<i32>(&_connection)
                .is_ok() {

                let _new_friend = NewFeaturedFriend {
                    user_id:   _user.id,
                    target_id: self.id,
                    hidden:    false,
                };
                let _new_item = diesel::insert_into(schema::featured_friends::table)
                    .values(&_new_friend)
                    .execute(&_connection);
            }
        }
    }
    pub fn get_or_create_featured_friend(&self, user_id: i32) -> () {
        use crate::schema::featured_friends::dsl::featured_friends;
        use crate::models::NewFeaturedFriend;

        let _connection = establish_connection();
        if !featured_friends
            .filter(schema::featured_friends::target_id.eq(self.id))
            .filter(schema::featured_friends::user_id.eq(user_id))
            .select(schema::featured_friends::id)
            .first::<i32>(&_connection)
            .is_ok() {

            let _new_friend = NewFeaturedFriend {
                user_id:   user_id,
                target_id: self.id,
                hidden:    false,
            };
            let _new_item = diesel::insert_into(schema::featured_friends::table)
                .values(&_new_friend)
                .execute(&_connection);
        }
    }

    pub fn delete_featured_friend(&self, user_id: i32) -> () {
        use crate::schema::featured_friends::dsl::featured_friends;

        let _connection = establish_connection();
        diesel::delete (
            featured_friends
            .filter(schema::featured_friends::target_id.eq(self.id))
            .filter(schema::featured_friends::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E");
    }

    pub fn follow_user(&self, user: User) -> i16 {
        if self.id == user.id || self.is_self_user_in_block(user.id) || self.is_followers_user_with_id(user.id) || self.is_following_user_with_id(user.id) {
            return 0;
        }
        use crate::models::{Follow, NewFollow};

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id:   self.id,
            target_id: user.id,
            view:      false,
            visited:   0,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection);
        if new_follow.is_ok() {
            user.plus_follows(1);
            if user.is_user_see_all(self.id) {
                //self.add_new_user_subscriber(&user);
                self.get_or_create_featured_friends(user);
            }
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn follow_view_user(&self, user_id: i32) -> i16 {
        if self.id == user_id || !self.is_followers_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow; 

        let _connection = establish_connection();

        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user_id))
            .first::<Follow>(&_connection)
            .expect("E");
        let u = diesel::update(&_follow)
            .set(schema::follows::view.eq(true))
            .execute(&_connection);

        if u.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn unfollow_user(&self, user: User) -> i16 {
        if self.id == user.id || !self.is_following_user_with_id(user.id) {
            return 0;
        }
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();
        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user.id))
            .first::<Follow>(&_connection);
        if _follow.is_ok() {
            let del = diesel::delete (
                    follows
                        .filter(schema::follows::target_id.eq(user.id))
                        .filter(schema::follows::user_id.eq(self.id))
                )
                .execute(&_connection);
            //self.delete_new_subscriber(user.id);
            user.minus_follows(1);
            if del.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }

    pub fn frend_user(&self, user: User) -> i16 {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return 0;
        }
        use crate::models::NewFriend;
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id:   self.id,
            target_id: user.id,
            visited:   0,
        };
        let f_1 = diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .execute(&_connection);

        let _new_friend_2 = NewFriend {
            user_id:   user.id,
            target_id: self.id,
            visited:   0,
        };
        let f_2 = diesel::insert_into(schema::friends::table)
            .values(&_new_friend_2)
            .execute(&_connection);

        let del = diesel::delete (
            follows
                .filter(schema::follows::user_id.eq(user.id))
                .filter(schema::follows::target_id.eq(self.id)))
                .execute(&_connection);
        if del.is_ok() && f_1.is_ok() && f_2.is_ok() {
            user.plus_friends(1);
            self.plus_friends(1);
            self.minus_follows(1);
            if !user.is_user_see_all(self.id) {
                self.get_or_create_featured_friends(user);
            }
            return 1;
        }
        return 0;
    }
    pub fn unfrend_user(&self, user: User) -> i16 {
        if self.id == user.id || !self.is_connected_with_user_with_id(user.id) {
            return 0;
        }
        use crate::models::NewFollow;
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();

        let del_1 = diesel::delete (
            friends
                .filter(schema::friends::user_id.eq(self.id))
                .filter(schema::friends::target_id.eq(user.id))
            )
            .execute(&_connection);
        let del_2 = diesel::delete (
            friends
                .filter(schema::friends::target_id.eq(self.id))
                .filter(schema::friends::user_id.eq(user.id))
            )
            .execute(&_connection);

        let _new_follow = NewFollow {
            user_id:   user.id,
            target_id: self.id,
            view:      true,
            visited:   0,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);

        if del_1.is_ok() && del_2.is_ok() && new_follow.is_ok() {
            user.minus_friends(1);
            self.minus_friends(1);
            self.plus_follows(1);
            //if !user.is_user_see_all(self.id) {
            //    self.delete_new_subscriber(user.id);
            //}
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn block_user(&self, user: User) -> i16 {
        if self.id == user.id || self.is_user_in_block(user.id) {
            return 0;
        }
        //use crate::schema::user_blocks::dsl::user_blocks;
        use crate::models::NewUserBlock;

        let _connection = establish_connection();

        if self.is_connected_with_user_with_id(user.id) {
            use crate::schema::friends::dsl::friends;
            diesel::delete(
                friends
                    .filter(schema::friends::user_id.eq(self.id))
                    .filter(schema::friends::target_id.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            diesel::delete(
                friends
                    .filter(schema::friends::target_id.eq(self.id))
                    .filter(schema::friends::user_id.eq(user.id)))
                    .execute(&_connection)
                    .expect("E");
            user.minus_friends(1);
            self.minus_friends(1);
        }
        else if self.is_followers_user_with_id(user.id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete(
                follows
                    .filter(schema::follows::target_id.eq(self.id))
                    .filter(schema::follows::user_id.eq(user.id))
                )
                .execute(&_connection)
                .expect("E");
            user.minus_follows(1);
        }
        else if self.is_following_user_with_id(user.id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete(
                follows
                    .filter(schema::follows::user_id.eq(self.id))
                    .filter(schema::follows::target_id.eq(user.id))
                )
                .execute(&_connection)
                .expect("E");
            self.minus_follows(1);
        }

        let _user_block = NewUserBlock {
            user_id:   self.id,
            target_id: user.id,
        };
        diesel::insert_into(schema::user_blocks::table)
            .values(&_user_block)
            .execute(&_connection)
            .expect("Error.");
        self.delete_featured_friend(user.id);
        //self.delete_notification_subscriber(user.id);
        return 1;
    }
    pub fn unblock_user(&self, user: User) -> i16 {
        if self.id == user.id || !self.is_user_in_block(user.id) {
            return 0;
        }
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        let del = diesel::delete(
            user_blocks
                .filter(schema::user_blocks::user_id.eq(self.id))
                .filter(schema::user_blocks::target_id.eq(user.id))
            )
            .execute(&_connection);
        if del.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn plus_friend_visited(&self, user_id: i32) -> () {
        use crate::schema::friends::dsl::friends;
        use crate::models::Friend;

        let _connection = establish_connection();
        let _connect = friends
            .filter(schema::friends::user_id.eq(self.id))
            .filter(schema::friends::target_id.eq(user_id))
            .first::<Friend>(&_connection)
            .expect("E");
        diesel::update(&_connect)
                .set(schema::friends::visited.eq(_connect.visited + 1))
                .execute(&_connection)
                .expect("Error.");
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
    pub fn get_token_detail(&self, token_id: i32) -> TokenDetailJson {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let _token = owners
            .filter(schema::owners::id.eq(token_id))
            .filter(schema::owners::types.eq(2))
            .first::<Owner>(&_connection)
            .expect("E.");

        let mut services = Vec::new();
        for s in _token.get_services().iter() {
            services.push (TokenServiceJson {
                id:   s.id,
                name: s.name.clone(),
            });
        }

        return TokenDetailJson {
            id:          _token.id,
            name:        _token.name.clone(),
            description: _token.description.clone(),
            is_active:   _token.is_active,
            services:    services,
        }
    }
    pub fn get_app_token_detail(&self, token_id: i32) -> TokenDetailJson {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let _token = owners
            .filter(schema::owners::id.eq(token_id))
            .filter(schema::owners::types.eq(1))
            .first::<Owner>(&_connection)
            .expect("E.");

        let mut services = Vec::new();
        for s in _token.get_services().iter() {
            services.push (TokenServiceJson {
                id:   s.id,
                name: s.name.clone(),
            });
        }

        return TokenDetailJson {
            id:          _token.id,
            name:        _token.name.clone(),
            description: _token.description.clone(),
            is_active:   _token.is_active,
            services:    services,
        }
    }

    pub fn get_tokens(&self) -> Vec<TokenJson> {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let mut list = Vec::new();

        let _tokens = owners
            .filter(schema::owners::user_id.eq(self.id))
            .filter(schema::owners::types.eq(2))
            .load::<Owner>(&_connection)
            .expect("E.");

        for i in _tokens.iter() {
            let mut services = Vec::new();
            for s in i.get_services().iter() {
                services.push (TokenServiceJson {
                    id:   s.id,
                    name: s.name.clone(),
                });
            }
            list.push (
                TokenJson {
                    id:        i.id,
                    name:      i.name.clone(),
                    is_active: i.is_active,
                    services:  services,
                }
            );
        }

        return list;
    }
    pub fn get_app_tokens(&self) -> Vec<TokenJson> {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let mut list = Vec::new();

        let _tokens = owners
            .filter(schema::owners::user_id.eq(self.id))
            .filter(schema::owners::types.eq(1))
            .load::<Owner>(&_connection)
            .expect("E.");

        for i in _tokens.iter() {
            let mut services = Vec::new();
            for s in i.get_services().iter() {
                services.push (TokenServiceJson {
                    id:   s.id,
                    name: s.name.clone(),
                });
            }
            list.push (
                TokenJson {
                    id:        i.id,
                    name:      i.name.clone(),
                    is_active: i.is_active,
                    services:  services,
                }
            );
        }

        return list;
    }
    pub fn get_all_tokens(&self) -> Vec<TokenJson> {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let mut list = Vec::new();

        let _tokens = owners
            .filter(schema::owners::user_id.eq(self.id))
            .load::<Owner>(&_connection)
            .expect("E.");

        for i in _tokens.iter() {
            let mut services = Vec::new();
            for s in i.get_services().iter() {
                services.push (TokenServiceJson {
                    id:   s.id,
                    name: s.name.clone(),
                });
            }
            list.push (
                TokenJson {
                    id:        i.id,
                    name:      i.name.clone(),
                    is_active: i.is_active,
                    services:  services,
                }
            );
        }

        return list;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
