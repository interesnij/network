//use crate::schema;
use crate::schema::{
    user_privates,
    user_locations,
    ip_users,
    user_delete_anketas,
    user_love_statuss,
    user_partner_ones,
    user_mom_ones,
    user_dad_ones,
    user_brother_sisters,
    user_children_ones,
    user_grandsons_ones,
    user_colleagues_ones,
    user_blocks,
    list_user_communities_keys,
    user_populate_smiles,
    user_populate_stickers,
    user_infos,
    friends,
    follows,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
//use crate::utils::establish_connection;
//use actix_web::web::Json;
use crate::errors::Error;

/////// Friend //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Friend {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
    pub visited:   i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user_id:   i32,
    pub target_id: i32,
    pub visited:   i32,
}

/////// Follow //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Follow {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
    pub view:      bool,
    pub visited:   i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:   i32,
    pub target_id: i32,
    pub view:      bool,
    pub visited:   i32,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserInfo {
    pub id:        i32,
    pub user_id:   i32,
    pub avatar_id: Option<i32>,
    pub language:  String,
    pub email:     Option<String>,
    pub birthday:  Option<chrono::NaiveDate>,
    pub b_avatar:  Option<String>,
    pub status:    Option<String>,
    pub city:      Option<String>,
    pub level:     i16,
    pub cover:     Option<String>,
    pub created:   chrono::NaiveDateTime,
    pub friends:   i32,
    pub follows:   i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_infos"]
pub struct NewUserInfo {
    pub user_id:   i32,
    pub avatar_id: Option<i32>,
    pub language:  String,
    pub email:     Option<String>,
    pub birthday:  Option<chrono::NaiveDate>,
    pub b_avatar:  Option<String>,
    pub status:    Option<String>,
    pub city:      Option<String>,
    pub level:     i16,
    pub cover:     Option<String>,
    pub created:   chrono::NaiveDateTime,
    pub friends:   i32,
    pub follows:   i32,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserLocation {
    pub id:         i32,
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_locations"]
pub struct NewUserLocation {
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct IpUser {
    pub id:      i32,
    pub user_id: i32,
    pub ip:      String,
}
#[derive(Deserialize, Insertable)]
#[table_name="ip_users"]
pub struct NewIpUser {
    pub user_id: i32,
    pub ip:      String,
}

/////// UserDeleteAnketa //////
    // 1 "У меня есть другая страница",
    // 2 "Соцсеть отнимает много времени",
    // 3 "Мало свободы самовыражения",
    // 4 "Соцсеть плохо защищает данные",
    // 5 "Соцсеть плохо защищает детей",
    // 6 "Другая причина",

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserDeleteAnketa {
    pub id:      i32,
    pub user_id: i32,
    pub answer:  i16,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_delete_anketas"]
pub struct NewUserDeleteAnketa {
    pub user_id: i32,
    pub answer:  i16,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}

/////// UserLoveStatus //////

// 1 "Не выбрано",
// 2 "Не женат",
// 3 "Есть подруга",
// 4 "Помолвлен",
// 5 "Женат",
// 6 "В гражданском браке",
// 7 "Влюблён",
// 8 "Всё сложно",
// 9 "В активном поиске",

// 1 "Не выбрано",
// 2 "Не женат",
// 3 "Есть подруга",
// 4 "Помолвлен",
// 5 "Женат",
// 6 "В гражданском браке",
// 7 "Влюблён",
// 8 "Всё сложно",
// 9 "В активном поиске",

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserLoveStatus {
    pub id:            i32,
    pub user_id:       i32,
    pub male_status:   i16,
    pub female_status: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_love_statuss"]
pub struct NewUserLoveStatus {
    pub user_id:       i32,
    pub male_status:   i16,
    pub female_status: i16,
}

/////// UserPartnerOne //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPartnerOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_partner_ones"]
pub struct NewUserPartnerOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserMomOne //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserMomOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_mom_ones"]
pub struct NewUserMomOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserDadOne //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserDadOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_dad_ones"]
pub struct NewUserDadOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserBrothersSisters //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserBrotherSister {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_brother_sisters"]
pub struct NewUserBrotherSister {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserChildren //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserChildrenOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_children_ones"]
pub struct NewUserChildrenOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserGrandsons //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserGrandsonsOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_grandsons_ones"]
pub struct NewUserGrandsonsOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserColleagues //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserColleaguesOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_colleagues_ones"]
pub struct NewUserColleaguesOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserBlocks //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserBlock {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_blocks"]
pub struct NewUserBlock {
    pub user_id:   i32,
    pub target_id: i32,
}

//////////////////////////////////////////////////////
/////// ListUC //////
    // 1 Не активный
    // 2 Активный список

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct ListUserCommunitiesKey {
    pub id:    i16,
    pub types: i16,
    pub name:  String,
    pub owner: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="list_user_communities_keys"]
pub struct NewListUserCommunitiesKey {
    pub types: i16,
    pub name:  String,
    pub owner: i32,
}

/////====================================////

/////// UserPrivate //////
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

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPrivate {
    pub id:         i32,
    pub user_id:    i32,
    pub see_all:    i16,
    pub see_info:   i16,
    pub see_friend: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_privates"]
pub struct NewUserPrivate {
    pub user_id:    i32,
    pub see_all:    i16,
    pub see_info:   i16,
    pub see_friend: i16,
}

/////// UserPopulateSmiles //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPopulateSmile {
    pub id:       i32,
    pub user_id:  i32,
    pub smile_id: i32,
    pub count:    i32,
    pub image:    String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_smiles"]
pub struct NewUserPopulateSmile {
    pub user_id:  i32,
    pub smile_id: i32,
    pub count:    i32,
    pub image:    String,
}

/////// UserPopulateStickers //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPopulateSticker {
    pub id:         i32,
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
    pub image:      String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_stickers"]
pub struct NewUserPopulateSticker {
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
    pub image:      String,
}
