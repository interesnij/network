use serde::{Serialize, Deserialize};
use crate::models::OwnerService;


// список url сервисов, на которых присутствуют копии сообществ.
pub const COMMUNITIES_SERVICES: &'static [&'static str] = &[
    "194.58.90.123:9003", // сервис записей
    "194.58.90.123:9004", // сервис фотографий
];

////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct UsersJson {
    pub users:     Vec<CardUserJson>,
    pub next_page: i32,
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

#[derive(Deserialize, Serialize)]
pub struct EditTokenPageResp {
    pub id:            i32,
    pub name:          String,
    pub description:   Option<String>,
    pub is_active:     bool,
    pub item_services: Vec<OwnerService>,
    pub all_services:  Vec<OwnerService>,
}

#[derive(Serialize)]
pub struct KeyValue {
    pub value: i16,
    pub info:  String,
}

#[derive(Serialize)]
pub struct EditLinkResp {
    pub link: String,
}

#[derive(Deserialize)]
pub struct ObjectData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub id:      Option<i32>,
}

#[derive(Deserialize)]
pub struct SmallData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
}

////////
#[derive(Serialize, Queryable)]
pub struct CommunityCategoryJson {
    pub id:     i32,
    pub name:   String,
    pub avatar: Option<String>,
}

////////
#[derive(Serialize, Queryable)]
pub struct CommunitySubcategoryJson {
    pub id:     i32,
    pub name:   String,
    pub avatar: Option<String>,
}
////////
#[derive(Serialize)]
pub struct CommunityDetailJson {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub types:       i16,
    pub perm:        String,
    pub link:        String, // community.get_link()
    pub image:       String,
    pub cover:       Option<String>,
    pub user_id:     i32,
}
#[derive(Serialize)]
pub struct NewCommunityJson {
    pub name:  String,
    pub types: i16,
    pub link:  String,
}

#[derive(Serialize, Queryable)]
pub struct CardCommunityJson {
    pub id:    i32,
    pub name:  String, 
    pub link:  String,
    pub image: Option<String>,
    pub count: i32,
}

////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct CommunityInvitesJson {
    pub users:     Vec<CardCommunityInviteJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
// это объект пользователя
pub struct CardCommunityInviteJson {
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      String,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityInfoJson {
    pub posts:     i16,
    pub members:   i16,
    pub photos:    i16,
    pub goods:     i16,
    pub tracks:    i16,
    pub videos:    i16,
    pub docs:      i16,
    pub articles:  i16,
    pub survey:    i16,
    pub planners:  i16,
}

////////
#[derive(Serialize)]
pub struct CommunityPrivateJson {
    pub see_member:   i16,
    pub see_info:     i16,
    pub see_settings: i16,
    pub see_log:      i16,
    pub see_stat:     i16,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityNotificationJson {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}


#[derive(Deserialize)]
pub struct SearchRegListData {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub q:            Option<String>,
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchTargetListData {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub q:            Option<String>,
    pub target_id:    Option<i32>,
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
}

#[derive(Serialize)]
pub struct EditPrivateResp {
    pub see_member:                   KeyValue,
    pub see_info:                     KeyValue,
    pub see_settings:                 KeyValue,
    pub see_log:                      KeyValue,
    pub see_stat:                     KeyValue,

    pub see_member_exclude_members:   Option<Vec<CardUserJson>>,
    pub see_member_include_members:   Option<Vec<CardUserJson>>,

    pub see_info_exclude_members:     Option<Vec<CardUserJson>>,
    pub see_info_include_members:     Option<Vec<CardUserJson>>,

    pub see_settings_exclude_members: Option<Vec<CardUserJson>>,
    pub see_settings_include_members: Option<Vec<CardUserJson>>,

    pub see_log_exclude_members:      Option<Vec<CardUserJson>>,
    pub see_log_include_members:      Option<Vec<CardUserJson>>,

    pub see_stat_exclude_members:     Option<Vec<CardUserJson>>,
    pub see_stat_include_members:     Option<Vec<CardUserJson>>,
}

#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}