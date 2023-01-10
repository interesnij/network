use serde::{Serialize, Deserialize};
use crate::models::OwnerService;


pub static TOKEN: &str = "111";

// список url сервисов, на которых присутствуют копии сообществ.
pub const COMMUNITIES_SERVICES: &'static [&'static str] = &[
    "194.58.90.123:9003", // сервис записей
    "194.58.90.123:9004", // сервис фотографий
];

#[derive(Deserialize)]
pub struct CItemVecData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub list_id:      Option<i32>,
    pub users_ids: Option<Vec<i32>>,
}
#[derive(Deserialize)]
pub struct PListData {
    pub token:        Option<String>,
    pub list_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub item_id:      Option<i32>,
}

#[derive(Deserialize)]
pub struct UItemVecData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub list_id:      Option<i32>,
    pub communities_ids: Option<Vec<i32>>,
}

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

#[derive(Serialize, Deserialize)]
pub struct ObjectData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}

#[derive(Deserialize)]
pub struct ItemParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct MinimalData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
}

#[derive(Serialize)]
pub struct EditNotifyResp {
    pub community_id:       i32,
    pub connection_request: i16,
    pub new_member:         i16,
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
pub struct SectionJson { 
    pub lists: Vec<CardList>,
    pub items: Vec<CardCommunityJson>, 
}
#[derive(Serialize, Deserialize, Queryable)]
pub struct CardList {
    pub id:       i32,
    pub name:     String,
    pub position: i16,
    pub count:    i32,
}
#[derive(Serialize, Deserialize, Queryable)]
pub struct CardEditList {
    pub id:   i32,
    pub name: String,
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
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchRegListData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub q:            Option<String>,
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct EditCommunityPrivateData {
    pub token:   Option<String>,
    pub community_id: Option<i32>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<i32>>,
}
#[derive(Serialize, Deserialize)]
pub struct EditUserPrivateData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub field:   Option<String>,
    pub value:   Option<i16>,
    pub users:   Option<Vec<i32>>,
}

#[derive(Serialize)] 
pub struct EditCommunityPrivateResp {
    pub see_member:           KeyValue,
    pub see_info:             KeyValue,
    pub see_settings:         KeyValue,
    pub see_log:              KeyValue,
    pub see_stat:             KeyValue,
    pub see_member_members:   Option<Vec<CardUserJson>>,
    pub see_info_members:     Option<Vec<CardUserJson>>,
    pub see_settings_members: Option<Vec<CardUserJson>>,
    pub see_log_members:      Option<Vec<CardUserJson>>,
    pub see_stat_members:     Option<Vec<CardUserJson>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AttachOwner {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub types:      i16,
    pub link:       String,
    pub s_avatar:   Option<String>,
    pub see_all:    i16,
}

#[derive(Serialize)] 
pub struct EditUserPrivateResp {
    pub see_community:       KeyValue,
    pub invite:              KeyValue,
    pub see_community_users: Option<Vec<CardUserJson>>,
    pub invite_users:        Option<Vec<CardUserJson>>,
}

#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}