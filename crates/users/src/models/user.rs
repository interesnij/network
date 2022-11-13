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
use crate::models::{
    UserLocation,
    UserInfo,
    UserPrivate,
    UserBlock,
};
use crate::utils::{
    UserPrivateJson,
    CardUserJson,
    UserPopulateStickerJson,
    UserPopulateSmileJson,
    LocationJson,
    UserDetailJson,

};
use crate::schema::users;
use actix_web::web::Json;
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
    pub fn get_user_by_phone(phone: &String) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::phone.eq(phone))
            .filter(schema::users::types.lt(30))
            .first::<User>(&_connection)?);
    }
    pub fn get_user_detail_json(&self) -> UserDetailJson {
        let language: String;
        let city: Option<String>;
        let status: Option<String>;
        let image: Option<String>;
        let mut _b = "".to_string();

        let info = self.get_info_model();
        match info {
          Ok(_ok) => {
              language = _ok.language;
              city = _ok.city;
              status = _ok.status;
              image = _ok.b_avatar;
              if _ok.birthday.is_some() {
                  _b = _ok.birthday.unwrap().format("%d-%m-%Y").to_string();
              }
          },
          Err(_error) => {
              language = "".to_string();
              city = None;
              status = None;
              image = None;
          },
        };
        let user_json = UserDetailJson {
             id:            self.id,
             first_name:    self.first_name.clone(),
             last_name:     self.last_name.clone(),
             types:         self.types,
             is_man:        self.is_man.clone(),
             language:      language,
             link:          self.get_slug(), // community.get_link()
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

    pub fn get_plus_or_create_populate_smile(&self, smile_id: i32, image: String) {
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;
        use crate::models::{UserPopulateSmile, NewUserPopulateSmile};

        let _connection = establish_connection();

        let populate_smile = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .filter(schema::user_populate_smiles::smile_id.eq(smile_id))
            .first::<UserPopulateSmile>(&_connection);
        if populate_smile.is_ok() {
            let _smile = populate_smile.expect("E.");
            diesel::update(&_smile)
                .set(schema::user_populate_smiles::count.eq(_smile.count + 1))
                .execute(&_connection)
                .expect("Error.");
        } else {
            let new_smile = NewUserPopulateSmile {
                user_id:  self.id,
                smile_id: smile_id,
                count:    1,
                image:    image,
            };
            diesel::insert_into(schema::user_populate_smiles::table)
                .values(&new_smile)
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn get_plus_or_create_populate_sticker(&self, sticker_id: i32, image: String) {
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;
        use crate::models::{UserPopulateSticker, NewUserPopulateSticker};

        let _connection = establish_connection();

        let populate_sticker = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .filter(schema::user_populate_stickers::sticker_id.eq(sticker_id))
            .first::<UserPopulateSticker>(&_connection);
        if populate_sticker.is_ok() {
            let _sticker = populate_sticker.expect("E.");
            diesel::update(&_sticker)
                .set(schema::user_populate_stickers::count.eq(_sticker.count + 1))
                .execute(&_connection)
                .expect("Error.");
        } else {
            let new_sticker = NewUserPopulateSticker {
                user_id:    self.id,
                sticker_id: sticker_id,
                count:      1,
                image:      image,
            };
            diesel::insert_into(schema::user_populate_stickers::table)
                .values(&new_sticker)
                .execute(&_connection)
                .expect("Error.");
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

    pub fn get_populate_smiles_json(&self) -> Vec<UserPopulateSmileJson> {
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;

        let _connection = establish_connection();
        let all_populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .order(schema::user_populate_smiles::count.desc())
            .limit(20)
            .select((
                schema::user_populate_smiles::smile_id,
                schema::user_populate_smiles::image
            ))
            .load::<(i32, String)>(&_connection)
            .expect("E");
        let mut smiles_json = Vec::new();
        for smile in all_populate_smiles.iter() {
            smiles_json.push(UserPopulateSmileJson {
                smile_id: smile.0,
                image:    smile.1.clone(),
            });
        }
        return smiles_json;
    }

    pub fn get_populate_stickers_json(&self) -> Vec<UserPopulateStickerJson> {
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;

        let _connection = establish_connection();
        let all_populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .order(schema::user_populate_stickers::count.desc())
            .limit(20)
            .select((
                schema::user_populate_stickers::sticker_id,
                schema::user_populate_stickers::image
            ))
            .load::<(i32, String)>(&_connection)
            .expect("E");
        let mut stickers_json = Vec::new();
        for sticker in all_populate_stickers.iter() {
            stickers_json.push(UserPopulateStickerJson {
                sticker_id: sticker.0,
                image:    sticker.1.clone(),
            });
        }
        return stickers_json;
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
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_id.eq(self.id))
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
    pub fn get_buttons_profile(&self, user_id: i32) -> String {
        let mut suffix: String = "".to_string();
        if self.is_user_in_block(user_id) {
            return "desctop/users/button/".to_owned() + &suffix + &"blocked_user.stpl".to_string();
        }
        else if self.is_self_user_in_block(user_id) {
            return "desctop/users/button/".to_owned() + &suffix + &"blocker_user.stpl".to_string();
        }
        else if self.is_connected_with_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"frend_user.stpl".to_string();
        }
        else if self.is_followers_user_view(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_user.stpl".to_string();
        }
        else if self.is_following_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"following_user.stpl".to_string();
        }
        else if self.is_followers_user_with_id(user_id){
            return "desctop/users/button/".to_owned() + &suffix + &"follow_view_user.stpl".to_string();
        }
        else {
            return "desctop/users/button/".to_owned() + &suffix + &"default_user.stpl".to_string();
        }
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
        use crate::schema::user_infos::dsl::user_infos;

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

    pub fn get_blocked_users(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            user_blocks::dsl::user_blocks,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let all_user_blocks = user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .order(schema::user_blocks::id.desc())
            .limit(limit)
            .offset(offset)
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(blocked_users);
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

    pub fn get_friends(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(limit)
            .offset(offset)
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_friends);
    }
    pub fn get_6_friends(&self) -> Result<Vec<CardUserJson>, Error> {
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_friends);
    }

    pub fn get_online_friends(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };
        use chrono::Duration;

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(limit)
            .offset(offset)
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }
    pub fn get_online_friends_count(&self) -> usize {
        let count = self.get_online_friends(500, 0);
        return match count {
          Ok(_ok) => _ok.len(),
          Err(_) => 0,
        };
    }
    pub fn get_6_online_friends(&self) -> Result<Vec<CardUserJson>, Error> {
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_followers(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };

        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::target_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(limit)
            .offset(offset)
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
            .load::<CardUserJson>(&_connection)?;

        return Ok(_users);
    }
    pub fn get_6_followers(&self) -> Result<Vec<CardUserJson>, Error> {
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
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

    pub fn get_users(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let users_list = users
            .filter(schema::users::types.lt(30))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(users_list);
    }
    pub fn get_anon_users(limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let _users = users
            .filter(schema::users::types.lt(30))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)?;

        return Ok(_users);
    }

    pub fn get_anon_users_count() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(30))
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_followings(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::follows::dsl::follows;
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::user_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(limit)
            .offset(offset)
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_common_friends_of_user(&self, user: &User, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
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
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)?;

        return Ok(_users);
    }
    pub fn get_6_common_friends_of_user(&self, user: &User) -> Result<Vec<CardUserJson>, Error> {
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
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
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

    pub fn get_see_all_exclude_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_all_exclude_friends_ids());
    }
    pub fn get_see_all_include_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_all_include_friends_ids());
    }
    pub fn get_see_all_exclude_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_all_exclude_follows_ids());
    }
    pub fn get_see_all_include_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_all_include_follows_ids());
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

    pub fn get_see_info_exclude_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_info_exclude_friends_ids());
    }
    pub fn get_see_info_include_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_info_include_friends_ids());
    }
    pub fn get_see_info_exclude_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_info_exclude_follows_ids());
    }
    pub fn get_see_info_include_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_info_include_follows_ids());
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

    pub fn get_see_friend_exclude_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_friend_exclude_friends_ids());
    }
    pub fn get_see_friend_include_friends(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_friend_include_friends_ids());
    }
    pub fn get_see_friend_exclude_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_friend_exclude_follows_ids());
    }
    pub fn get_see_friend_include_follows(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;
        return get_users_from_ids(self.get_see_friend_include_follows_ids());
    }

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
              2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
              3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              7 => self.get_friends_ids().iter().any(|&i| i==user_id),
              8 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              9 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              10 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
              11 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
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
              2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
              3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_all_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_all_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_all_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_all_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              7 => self.get_friends_ids().iter().any(|&i| i==user_id),
              8 => !self.get_see_all_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              9 => self.get_see_all_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              10 => !self.get_see_all_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
              11 => self.get_see_all_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
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
              2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
              3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_friend_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_friend_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
              5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_friend_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_friend_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
              7 => self.get_friends_ids().iter().any(|&i| i==user_id),
              8 => !self.get_see_friend_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              9 => self.get_see_friend_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
              10 => !self.get_see_friend_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
              11 => self.get_see_friend_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
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
        return match private {
          Ok(_ok) => {
              let bool_see_all = match _ok.see_all {
                  1 => true,
                  2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
                  3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  7 => self.get_friends_ids().iter().any(|&i| i==user_id),
                  8 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  9 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  10 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
                  11 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
                  _ => false,
              };
              if bool_see_all == false {
                  return vec![false, false, false];
              }
              let mut bool_stack = Vec::new();
              bool_stack.push(true);
              let bool_see_info = match _ok.see_info {
                  1 => true,
                  2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
                  3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  7 => self.get_friends_ids().iter().any(|&i| i==user_id),
                  8 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  9 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  10 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
                  11 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
                  _ => false,
              };
              bool_stack.push(bool_see_info);

              let bool_see_friend = match _ok.see_friend {
                  1 => true,
                  2 => self.get_friends_ids().iter().any(|&i| i==user_id) || self.get_friends_ids().iter().any(|&i| i==user_id),
                  3 => self.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  4 => self.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id)),
                  5 => self.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  6 => self.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id)),
                  7 => self.get_friends_ids().iter().any(|&i| i==user_id),
                  8 => !self.get_see_info_exclude_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  9 => self.get_see_info_include_friends_ids().iter().any(|&i| i==user_id) && self.get_friends_ids().iter().any(|&i| i==user_id),
                  10 => !self.get_see_info_exclude_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
                  11 => self.get_see_info_include_follows_ids().iter().any(|&i| i==user_id) && self.get_follows_ids().iter().any(|&i| i==user_id),
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
        use crate::models::{UserVisiblePerm, NewUserVisiblePerm};
        use crate::schema::user_visible_perms::dsl::user_visible_perms;
        use crate::schema::friends::dsl::friends;

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
            //if user.is_user_see_all(self.id) {
            //    self.add_new_user_subscriber(&user);
            //    self.get_or_create_featured_objects(user);
            //}
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn follow_view_user(&self, user: User) -> i16 {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return 0;
        }
        use crate::schema::follows::dsl::follows;
        use crate::models::Follow;

        let _connection = establish_connection();

        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user.id))
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
            //if !user.is_user_see_all(self.id) {
            //    self.add_new_user_subscriber(&user);
            //    self.get_or_create_featured_objects(user);
            //}
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
        //self.delete_new_subscriber(user.id);
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub phone: String,
}
