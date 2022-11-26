//use crate::schema;
use crate::schema::{
    phone_codes,
    user_visible_perms,
    featured_friends,
    user_notifications,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
//use crate::errors::Error;

/////// PhoneCode //////
// types:
// 1 create account
// 2 update phone
// 3 update secure settings
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:      i32,
    pub phone:   String,
    pub code:    i32,
    pub types:   i32,
    pub accept:  bool,
    pub created: chrono::NaiveDateTime,
} 
#[derive(Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode { 
    pub phone:   String,
    pub code:    i32,
    pub types:   i32,
    pub accept:  bool,
    pub created: chrono::NaiveDateTime,
}

/////// Варианты значения полей приватности //////
// исключения/включения пользователей (связь user_id на основных пользователей)
// 1 может видеть открытый профиль
// 2 может видеть информацию
// 3 может видеть друзей
// 11 не может видеть открытый профиль
// 12 не может видеть информацию
// 13 не может видеть друзей

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:        i32,
    pub user_id:   i32, // кто добавил в список
    pub target_id: i32, // кого добавили в список
    pub types:     i16, // варианты (выше описаны)
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}


/////// FeaturedFriend //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FeaturedFriend {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
    pub hidden:    bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="featured_friends"]
pub struct NewFeaturedFriend {
    pub user_id:   i32,  // рекомандованный друг
    pub target_id: i32,  // кому рекомендован
    pub hidden:    bool, // пользователь скрыт
}


#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct UserToken {
    pub token: String
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserSignup {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub is_man:        bool,
    pub password:      String,
    pub link:          String,
    pub last_activity: chrono::NaiveDateTime,
}

/////// UserNotifications //////
#[derive(Queryable, Serialize, Identifiable, Associations)]
pub struct UserNotification {
    pub id:                   i32,
    pub user_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:     bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_notifications"]
pub struct NewUserNotification {
    pub user_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:     bool,
}
