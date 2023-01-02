use serde::{Serialize, Deserialize};
use crate::models::OwnerService;


// список url сервисов, на которых присутствуют копии пользователей.
// нужно, к примеру, для синхронизации данных пользователей.
pub const USERS_SERVICES: &'static [&'static str] = &[
    "194.58.90.123:9002", // сервис сообществ
    "194.58.90.123:9003", // сервис записей
    "194.58.90.123:9004", // сервис фотографий
];

#[derive(Serialize, Queryable)]
pub struct AttachUserResp {
    pub user_id:    i32,
    pub first_name: String,
    pub last_name:  String,
    pub types:      i16,
    pub link:       String,
    pub s_avatar:   Option<String>,
    pub see_all:    i16, 
} 

#[derive(Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}

#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}

#[derive(Deserialize)]
pub struct RegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
} 

#[derive(Deserialize)]
pub struct TargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize)]
pub struct ObjectData {
    pub token: Option<String>,
    pub id:    Option<i32>,
}

#[derive(Deserialize)]
pub struct SmallData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct SearchRegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub q:       Option<String>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchTargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub q:         Option<String>,
    pub target_id: Option<i32>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct EditTokenPageResp {
    pub id:            i32,
    pub name:          String,
    pub description:   Option<String>,
    pub is_active:     bool,
    pub item_services: Vec<OwnerService>,
    pub all_services:  Vec<OwnerService>,
} 

#[derive(Deserialize)]
pub struct UsersData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
}

#[derive(Serialize)]
pub struct EditPhoneResp {
    pub phone: String,
}
#[derive(Serialize)]
pub struct EditPasswordResp {
    pub password: String,
}
#[derive(Serialize)]
pub struct EditNameResp {
    pub first_name: String,
    pub last_name:  String,
}
#[derive(Serialize)]
pub struct EditLinkResp {
    pub link: String,
}

#[derive(Serialize)]
pub struct KeyValue {
    pub value: i16,
    pub info:  String,
}

#[derive(Serialize)]
pub struct EditPrivateResp {
    pub see_all:                    KeyValue,
    pub see_info:                   KeyValue,
    pub see_friend:                 KeyValue,
    pub see_all_exclude_friends:    Option<Vec<CardUserJson>>,
    pub see_all_exclude_follows:    Option<Vec<CardUserJson>>,
    pub see_all_include_friends:    Option<Vec<CardUserJson>>,
    pub see_all_include_follows:    Option<Vec<CardUserJson>>,
    pub see_info_exclude_friends:   Option<Vec<CardUserJson>>,
    pub see_info_exclude_follows:   Option<Vec<CardUserJson>>,
    pub see_info_include_friends:   Option<Vec<CardUserJson>>,
    pub see_info_include_follows:   Option<Vec<CardUserJson>>,
    pub see_friend_exclude_friends: Option<Vec<CardUserJson>>,
    pub see_friend_exclude_follows: Option<Vec<CardUserJson>>,
    pub see_friend_include_friends: Option<Vec<CardUserJson>>,
    pub see_friend_include_follows: Option<Vec<CardUserJson>>,
}

#[derive(Serialize)]
pub struct EditNotifyResp {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
} 