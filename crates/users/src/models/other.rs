//use crate::schema;
use crate::schema::{
    phone_codes,
    user_visible_perms,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;

/////// PhoneCode //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:    i32,
    pub phone: String,
    pub code:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode {
    pub phone: String,
    pub code:  i32,
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
