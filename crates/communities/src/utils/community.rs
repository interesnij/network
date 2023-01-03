use serde::{Serialize, Deserialize};
use crate::models::OwnerService;


pub static TOKEN: &str = "111";

// список url сервисов, на которых присутствуют копии сообществ.
pub const COMMUNITIES_SERVICES: &'static [&'static str] = &[
    "194.58.90.123:9003", // сервис записей
    "194.58.90.123:9004", // сервис фотографий
];

#[derive(Serialize, Queryable)]
pub struct AttachCommunityResp {
    pub community_id: i32, 
    pub name:         String,
    pub types:        i16,
    pub link:         String,
    pub s_avatar:     Option<String>,
} 

#[derive(Serialize, Deserialize)]
pub struct EditListJson {
    pub id:                   i32,
    pub name:                 String,
    pub position:             i16,
    pub see_el_exclude_users: Vec<CardUserJson>,
    pub see_el_include_users: Vec<CardUserJson>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct CardCommunitiesList {
    pub id:       i32,
    pub name:     String,
    pub position: i16,
    pub count:    i32,
}

#[derive(Serialize, Deserialize, Queryable)]
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

#[derive(Deserialize)]
pub struct ObjectData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}

#[derive(Deserialize)]
pub struct MinimalData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
}

#[derive(Serialize)]
pub struct EditNotifyResp {
    pub community_id:       i32,
    pub connection_request: bool,
    pub new_member:         bool,
} 

#[derive(Serialize)]
pub struct EditLinkResp {
    pub community_id: i32,
    pub link:         String,
}

#[derive(Serialize)]
pub struct EditNameResp {
    pub community_id: i32,
    pub name:         String,
}

#[derive(Serialize, Deserialize)]
pub struct SmallData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
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
    pub status:      Option<String>,
    pub slug:        String,
    pub description: Option<String>,
    pub cover:       Option<String>,
    pub image:       Option<String>,
    pub avatar_id:   Option<i32>,
    pub identified:  i16,
}

#[derive(Deserialize, Serialize)]
pub struct RespListJson {
    pub id:   i32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DataListJson {
    pub id:    i32,
    pub name:  String,
    pub types: i16,
}
#[derive(Serialize)]
pub struct NewCommunityJson {
    pub name:  String,
    pub types: i16,
    pub link:  String,
}

#[derive(Serialize, Queryable)]
pub struct CardCommunityJson { 
    pub id:      i32,
    pub name:    String, 
    pub link:    String,
    pub image:   Option<String>,
    pub members: i32,
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
pub struct RegListData {
    pub token:   Option<String>,
    pub id:      Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchRegListData {
    pub token:  Option<String>,
    pub id:     Option<i32>,
    pub q:      Option<String>,
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
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