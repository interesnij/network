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
use crate::models::{UserLocation, UserInfo, UserPrivate, UserBlock};
use crate::schema::users;
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
    pub last_activity: DateTimeNative,
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
    pub fn get_user_detail_json(&self) -> Json<UserDetailJson> {
         let user_json = UserDetailJson {
             id:            self.id,
             first_name:    self.first_name.clone(),
             last_name:     self.last_name.clone(),
             types:         self.types,
             is_man:        self.is_man.clone(),
             link:          self.get_slug(), // community.get_link()
             last_activity: self.last_activity.format("%d-%m-%Y в %H:%M").to_string(),
         };
         return Json(user_json);
    }
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }

    pub fn count_users() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .load::<User>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_ss_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return self.s_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }
    pub fn get_s_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:30px;width:30px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_30' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }
    pub fn get_40_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:40px;width:40px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_40' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }
    pub fn get_50_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:50px;width:50px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg fill='currentColor' class='svg_default svg_default_50' viewBox='0 0 24 24'><path d='M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z'/><path d='M0 0h24v24H0z' fill='none'/></svg>".to_string();
        }
    }

    //pub fn save_playlist(&self, types: &String) -> () {
    //    let _connection = establish_connection();
    //    let profile = self.get_info();
    //    diesel::update(&profile)
    //        .set(schema::user_profiles::saved_playlist.eq(types))
    //        .get_result::<UserProfile>(&_connection)
    //        .expect("E");
    //}

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

    pub fn get_plus_or_create_populate_smile(&self, smile_id: i32, image: String) {
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;
        use crate::models::{UserPopulateSmile, NewUserPopulateSmile};

        let _connection = establish_connection();

        let populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .filter(schema::user_populate_smiles::smile_id.eq(smile_id))
            .load::<UserPopulateSmile>(&_connection)
            .expect("E");
        if populate_smiles.len() > 0 {
            let populate_smile = populate_smiles.into_iter().nth(0).unwrap();
            diesel::update(&populate_smile)
                .set(schema::user_populate_smiles::count.eq(populate_smile.count + 1))
                .get_result::<UserPopulateSmile>(&_connection)
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
                .get_result::<UserPopulateSmile>(&_connection)
                .expect("Error.");
        }
    }
    pub fn get_plus_or_create_populate_sticker(&self, sticker_id: i32, image: String) {
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;
        use crate::models::{UserPopulateSticker, NewUserPopulateSticker};

        let _connection = establish_connection();

        let populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .filter(schema::user_populate_stickers::sticker_id.eq(sticker_id))
            .load::<UserPopulateSticker>(&_connection)
            .expect("E");
        if populate_stickers.len() > 0 {
            let populate_sticker = populate_stickers.into_iter().nth(0).unwrap();
            diesel::update(&populate_sticker)
                .set(schema::user_populate_stickers::count.eq(populate_sticker.count + 1))
                .get_result::<UserPopulateSticker>(&_connection)
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
                .get_result::<UserPopulateSticker>(&_connection)
                .expect("Error.");
        }
    }

    pub fn get_last_location_json(&self) -> Json<LocationJson> {
        use crate::schema::user_locations::dsl::user_locations;

        let _connection = establish_connection();
        let location = user_locations
            .filter(schema::user_locations::user_id.eq(self.id))
            .order(schema::user_locations::id.desc())
            .limit(1)
            .load::<UserLocation>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let location_json = LocationJson {
            city_ru:    location.city_ru,
            region_ru:  location.region_ru,
            country_ru: location.country_ru,
        };
        return Json(location_json);
    }
    pub fn get_last_location(&self) -> UserLocation {
        use crate::schema::user_locations::dsl::user_locations;

        let _connection = establish_connection();
        return user_locations
            .filter(schema::user_locations::user_id.eq(self.id))
            .order(schema::user_locations::id.desc())
            .limit(1)
            .load::<UserLocation>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_verb_gender(&self, str: &str) -> String {
        if self.is_man == false {
            return "W".to_string() + &str;
        }
        else {
            return str.to_string();
        }
    }

    pub fn get_populate_smiles_json(&self) -> Json<Vec<UserPopulateSmileJson>> {
        use crate::schema::user_populate_smiles::dsl::user_populate_smiles;

        let _connection = establish_connection();
        let all_populate_smiles = user_populate_smiles
            .filter(schema::user_populate_smiles::user_id.eq(self.id))
            .order(schema::user_populate_smiles::count.desc())
            .limit(20)
            .select((schema::user_populate_smiles::smile_id, schema::user_populate_smiles::image))
            .load::<(i32, String)>(&_connection)
            .expect("E");
        let mut smiles_json = Vec::new();
        for smile in all_populate_smiles.iter() {
            smiles_json.push(UserPopulateSmileJson {
                smile_id: smile.0,
                image:    smile.1.clone(),
            });
        }
        return Json(smiles_json);
    }

    pub fn get_populate_stickers_json(&self) -> Json<Vec<UserPopulateStickerJson>> {
        use crate::schema::user_populate_stickers::dsl::user_populate_stickers;

        let _connection = establish_connection();
        let all_populate_stickers = user_populate_stickers
            .filter(schema::user_populate_stickers::user_id.eq(self.id))
            .order(schema::user_populate_stickers::count.desc())
            .limit(20)
            .select((schema::user_populate_stickers::sticker_id, schema::user_populate_stickers::image))
            .load::<(i32, String)>(&_connection)
            .expect("E");
        let mut stickers_json = Vec::new();
        for sticker in all_populate_stickers.iter() {
            stickers_json.push(UserPopulateStickerJson {
                sticker_id: sticker.0,
                image:    sticker.1.clone(),
            });
        }
        return Json(stickers_json);
    }

    pub fn calculate_age(&self) -> i32 {
        use chrono::{NaiveDate, Datelike};

        let info = self.get_info_model();
        if info.birthday.is_some() {
            let birthday = self.birthday.unwrap();
            let d = NaiveDate::from_ymd(2015, 6, 3);
            return d.year() - birthday.year();
        }
        return 0;
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
            .limit(1)
            .select(schema::user_blocks::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .filter(schema::user_blocks::target_id.eq(user_id))
            .limit(1)
            .select(schema::user_blocks::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_id.eq(self.id))
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
            .filter(schema::follows::user_id.eq(self.id))
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
            .filter(schema::follows::target_id.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .limit(1)
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_followers_user_view(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.id))
            .filter(schema::follows::user_id.eq(user_id))
            .filter(schema::follows::view.eq(true))
            .limit(1)
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn get_buttons_profile(&self, user_id: i32) -> String {
        let mut suffix: String = "".to_string();
        if self.perm > 19 {
            suffix = "staff_".to_string();
        }
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
    pub fn get_info_model(&self) -> CommunityInfo {
        use crate::schema::user_infos::dsl::user_infos;

        let _connection = establish_connection();
        let infos = user_infos
            .filter(schema::user_infos::id.eq(self.id))
            .load::<UserInfo>(&_connection)
            .expect("E.");

        if infos.len() > 0 {
            return infos.into_iter().nth(0).unwrap();
        }
        else {
            use crate::models::NewUserInfo;

            let _user_info = NewUserInfo {
                user_id:   self.id,
                avatar_id: None,
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
                .get_result::<UserInfo>(&_connection)
                .expect("E.");

            return new_info;
        }
    }

    pub fn is_have_followers(&self) -> bool {
        return self.get_info_model().follows > 0
    }
    pub fn is_have_followings(&self) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.id))
            .limit(1)
            .select(schema::follows::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_have_blacklist(&self) -> bool {
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        return user_blocks
            .filter(schema::user_blocks::user_id.eq(self.id))
            .limit(1)
            .select(schema::user_blocks::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() > 0;
    }
    pub fn is_have_friends(&self) -> bool {
        return self.get_info_model().friends > 0;
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
        return self.get_info_model().follows;
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

    pub fn get_blocked_users_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_blocked_users(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_blocked_users(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_blocked_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Черный спсок".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }

    pub fn get_blocked_users(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
            .filter(schema::users::types.lt(10))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return blocked_users;
    }

    pub fn count_friends(&self) -> i32 {
        return self.get_info_model().friends;
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

    pub fn plus_follows(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_infos::follows.eq(profile.follows + count))
            .get_result::<UserInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_friends(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::user_infos::friends.eq(profile.friends + count))
            .get_result::<UserInfo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_follows(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        if profile.follows > 0 {
            let _connection = establish_connection();
            diesel::update(&profile)
                .set(schema::user_infos::follows.eq(profile.follows - count))
                .get_result::<UserInfo>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn minus_friends(&self, count: i32) -> bool {
        let profile = self.get_info_model();
        if profile.friends > 0 {
            let _connection = establish_connection();
            diesel::update(&profile)
                .set(schema::user_infos::friends.eq(profile.friends - count))
                .get_result::<UserInfo>(&_connection)
                .expect("Error.");
        }
        return true;
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
    pub fn get_friend_and_follows_ids(&self) -> Vec<i32> {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follows = follows
            .filter(schema::follows::user_id.eq(self.id))
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _follows.append(&mut _friends);
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
    pub fn get_friend_and_friend_of_friend_ids(&self) -> Vec<i32> {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let mut stack: Vec<i32> = Vec::new();

        let user_friends = friends
            .filter(schema::friends::user_id.eq(self.id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");

        for _item in user_friends.iter() {
            stack.push(*_item);
        };
        for friend in self.get_friends(500, 0).iter() {
            let user_friend_friends = friends
                .filter(schema::friends::user_id.eq(friend.id))
                .select(schema::friends::target_id)
                .load::<i32>(&_connection)
                .expect("E.");
            for f in user_friend_friends.iter() {
                if stack.iter().any(|&i| &i!=f) {
                    stack.push(*f);
                }
            }
        }
        return stack;
    }

    pub fn get_friends_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_friends(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_friends(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_friends(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Друзья".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }

    pub fn get_friends(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
            .filter(schema::users::types.lt(10))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_6_friends(&self) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let _friends = users
            .filter(schema::users::id.eq_any(self.get_6_friends_ids()))
            .filter(schema::users::types.lt(10))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }

    pub fn get_online_users_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_online_friends(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_online_friends(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_online_friends(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Друзья в сети".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }
    pub fn get_online_friends(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
            .filter(schema::users::types.lt(10))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn get_online_friends_count(&self) -> usize {
        return self.get_online_friends(500, 0).len();
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
            .filter(schema::users::types.lt(10))
            .filter(schema::users::last_activity.gt(chrono::Local::now().naive_utc() - Duration::seconds(300)))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_followers_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_followers(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_followers(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_followers(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Подписчики".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }
    pub fn get_followers(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
                schema::users::image,
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
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn get_all_users_count(&self) -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(10))
            .select(schema::users::id)
            .load::<User>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_users_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_users(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_users(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Пользователи".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }

    pub fn get_users(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let users_list = users
            .filter(schema::users::types.lt(10))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return users_list;
    }
    pub fn get_anon_users(limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let _users = users
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");

        return _users;
    }
    pub fn get_anon_users_json(page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = User::get_anon_users(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = User::get_anon_users(limit.into(), 0);
            have_next = limit + 1;
        }
        if User::get_anon_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Пользователи".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }

    pub fn get_anon_users_count() -> usize {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::types.lt(10))
            .select(schema::users::id)
            .load::<User>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_followings_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_followings(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_followings(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_followings(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Отправленне заявки".to_string(),
            users: users,
            next_page: next_page_number,
        });
    }

    pub fn get_followings(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
                schema::users::image,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_common_friends_of_user_json(&self, user: &User, page: i32, limit: i32) -> Json<UsersListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_common_friends_of_user(&user, limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_common_friends_of_user(&user, limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_common_friends_of_user(&user, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UsersListJson {
            description: "Общие друзья".to_string(),
            users:       users,
            next_page:   next_page_number,
        });
    }
    pub fn get_common_friends_of_user(&self, user: &User, limit: i64, offset: i64) -> Vec<CardUserJson> {
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
                schema::users::image,
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
                schema::users::image,
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

    pub fn get_private_model(&self) -> UserPrivate {
        use crate::schema::user_privates::dsl::user_privates;

        let _connection = establish_connection();
        return user_privates
            .filter(schema::user_privates::user_id.eq(self.id))
            .load::<UserPrivate>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_private_model_json(&self) -> Json<UserPrivateJson> {
        let private = self.get_private_model();
        let json = UserPrivateJson {
            see_all:    private.see_all,
            see_info:   private.see_info,
            see_friend: private.see_friend,
        };
        return Json(json);
    }
    pub fn is_user_see_info(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        return match private.see_info {
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
    }
    pub fn is_user_see_all(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        return match private.see_all {
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
    }
    pub fn is_user_see_friend(&self, user_id: i32) -> bool {
        if self.id == user_id {
            return true;
        }
        let private = self.get_private_model();
        let char = private.see_friend;
        return match char.as_str() {
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
    }

    pub fn get_profile_all_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == user_id {
            return vec![true, true, true];
        }
        let private = self.get_private_model();

        let bool_see_all = match private.see_all {
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

        let bool_see_info = match private.see_info {
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

        let see_friend = private.see_friend;
        let bool_see_friend = match see_friend.as_str() {
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
    }
    pub fn is_anon_user_see_all(&self) -> bool {
        let private = self.get_private_model();
        return private.see_all == 1;
    }
    pub fn is_anon_user_see_friend(&self) -> bool {
        let private = self.get_private_model();
        return private.see_friend == 1;
    }
    pub fn is_anon_user_see_info(&self) -> bool {
        let private = self.get_private_model();
        return private.see_info == 1;
    }

    pub fn get_anon_profile_all_see(&self) -> Vec<bool> {
        let private = self.get_private_model();

        let bool_see_all = private.see_all == 1;
        if bool_see_all == false {
            return vec![false, false, false];
        }

        let mut bool_stack = Vec::new();
        bool_stack.push(bool_see_all);

        bool_stack.push(private.see_info == 1);
        bool_stack.push(private.see_friend == 1);
        return bool_stack;
    }
    pub fn set_user_visible_perms(&self, users: String, types: i16) -> bool {
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
        let previous_user_list_delete = match types {
            1 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(11))
                )
                .execute(&_connection)
                .expect("E"),
            2 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(12))
                )
                .execute(&_connection)
                .expect("E"),
            3 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(13))
                )
                .execute(&_connection)
                .expect("E"),
            11 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(1))
                )
                .execute(&_connection)
                .expect("E"),
            12 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(2))
                )
                .execute(&_connection)
                .expect("E"),
            13 => diesel::delete (
                    user_visible_perms.filter(schema::user_visible_perms::user_id.eq(self.id))
                    user_visible_perms.filter(schema::user_visible_perms::types.eq(3))
                )
                .execute(&_connection)
                .expect("E"),
            _ => (),
        };
        for user_id in users_ids.iter() {
            let _new_perm = NewUserVisiblePerm {
                user_id:   self.id,
                target_id: user_id,
                types:     types,
            };
            diesel::insert_into(schema::user_visible_perms::table)
                .values(&_new_perm)
                .get_result::<UserVisiblePerm>(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn get_image_or_null(&self) -> Option<String> {
        if self.s_avatar.is_some() {
            return self.s_avatar.clone();
        }
        else {
            return None;
        }
    }

    pub fn follow_user(&self, user: User) -> () {
        if self.id == user.id || self.is_self_user_in_block(user.id) || self.is_followers_user_with_id(user.id) || self.is_following_user_with_id(user.id) {
            return;
        }
        use crate::models::NewFollow;

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id:   self.id,
            target_id: user.id,
            view:      false,
            visited:   0,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");
        user.plus_follows(1);
        if user.is_user_see_all(self.id) {
            self.add_new_user_subscriber(&user);
            self.get_or_create_featured_objects(user);
        }
    }
    pub fn follow_view_user(&self, user: User) -> () {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();

        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user.id))
            .load::<Follow>(&_connection)
            .expect("E");
        diesel::update(&_follow[0])
            .set(schema::follows::view.eq(true))
            .get_result::<Follow>(&_connection)
            .expect("Error.");
    }

    pub fn unfollow_user(&self, user: User) -> () {
        if self.id == user.id || !self.is_following_user_with_id(user.id) {
            return;
        }
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follow = follows
            .filter(schema::follows::user_id.eq(self.id))
            .filter(schema::follows::target_id.eq(user.id))
            .load::<Follow>(&_connection)
            .expect("E");
        if _follow.len() > 0 {
            diesel::delete(
                    follows
                        .filter(schema::follows::target_id.eq(user.id))
                        .filter(schema::follows::user_id.eq(self.id))
                )
                .execute(&_connection)
                .expect("E");
            self.delete_new_subscriber(user.id);
            user.minus_follows(1);
        }
    }

    pub fn frend_user(&self, user: User) -> () {
        if self.id == user.id || !self.is_followers_user_with_id(user.id) {
            return;
        }
        use crate::models::NewFriend;
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id:   self.id,
            target_id: user.id,
            visited:   0,
        };
        diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .get_result::<Friend>(&_connection)
            .expect("Error.");

        let _new_friend_2 = NewFriend {
            user_id:   user.id,
            target_id: self.id,
            visited:   0,
        };
        diesel::insert_into(schema::friends::table)
            .values(&_new_friend_2)
            .get_result::<Friend>(&_connection)
            .expect("Error.");

        diesel::delete(
            follows
                .filter(schema::follows::user_id.eq(user.id))
                .filter(schema::follows::target_id.eq(self.id)))
                .execute(&_connection)
                .expect("E");

        user.plus_friends(1);
        self.plus_friends(1);
        self.minus_follows(1);
        //if !user.is_user_see_all(self.id) {
        //    self.add_new_user_subscriber(&user);
        //    self.get_or_create_featured_objects(user);
        //}
    }
    pub fn unfrend_user(&self, user: User) -> () {
        if self.id == user.id || !self.is_connected_with_user_with_id(user.id) {
            return;
        }
        use crate::models::NewFollow;
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();

        diesel::delete(
            friends
                .filter(schema::friends::user_id.eq(self.id))
                .filter(schema::friends::target_id.eq(user.id))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete(
            friends
                .filter(schema::friends::target_id.eq(self.id))
                .filter(schema::friends::user_id.eq(user.id))
            )
            .execute(&_connection)
            .expect("E");

        let _new_follow = NewFollow {
            user_id:   user.id,
            target_id: self.id,
            view:      true,
            visited:   0,
        };
        diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .get_result::<Follow>(&_connection)
            .expect("Error.");

        user.minus_friends(1);
        self.minus_friends(1);
        self.plus_follows(1);
        if !user.is_user_see_all(self.id) {
            self.delete_new_subscriber(user.id);
        }
    }

    pub fn block_user(&self, user: User) -> () {
        if self.id == user.id || self.is_user_in_block(user.id) {
            return;
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
            .get_result::<UserBlock>(&_connection)
            .expect("Error.");
        self.delete_new_subscriber(user.id);
        self.delete_notification_subscriber(user.id);
    }
    pub fn unblock_user(&self, user: User) -> () {
        if self.id == user.id || !self.is_user_in_block(user.id) {
            return;
        }
        use crate::schema::user_blocks::dsl::user_blocks;

        let _connection = establish_connection();
        diesel::delete(
            user_blocks
                .filter(schema::user_blocks::user_id.eq(self.id))
                .filter(schema::user_blocks::target_id.eq(user.id)))
                .execute(&_connection)
                .expect("E");
    }
    pub fn plus_friend_visited(&self, user_id: i32) -> () {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _connect = friends
            .filter(schema::friends::user_id.eq(self.id))
            .filter(schema::friends::target_id.eq(user_id))
            .load::<Friend>(&_connection)
            .expect("E");
        diesel::update(&_connect[0])
                .set(schema::friends::visited.eq(_connect[0].visited + 1))
                .execute(&_connection)
                .expect("Error.");
    }

    pub fn get_members_for_notify_ids(&self) -> Vec<i32> {
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let items = notify_user_communities
            .filter(schema::notify_user_communities::user_id.eq(self.id))
            .filter(schema::notify_user_communities::mute.eq(false))
            .filter(schema::notify_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .select(schema::notify_user_communities::owner)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
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
