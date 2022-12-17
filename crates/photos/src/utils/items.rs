use serde::{Serialize, Deserialize};
use crate::utils::AttachmentsJson;
use crate::models::{
    OwnerService,
};

#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}
#[derive(Deserialize)]
pub struct ItemParams {
    pub token:        Option<String>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}
#[derive(Deserialize)]
pub struct DataCopyPhoto {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
    pub lists:   Vec<i32>,
}
#[derive(Deserialize)]
pub struct DataCopyList {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
    pub owners:  Vec<String>,
}
#[derive(Deserialize)]
pub struct SmallData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
}
#[derive(Deserialize)]
pub struct ReactionData {
    pub token:       Option<String>,
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub reaction_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct EditListJson {
    pub id:                   i32,
    pub name:                 String,
    pub description:          Option<String>,
    pub image:                Option<String>,
    pub see_el:               i16,
    pub see_comment:          i16,
    pub create_el:            i16,
    pub create_comment:       i16,
    pub copy_el:              i16,
    pub reactions:            Option<String>,

    pub see_el_exclude_users:         Vec<CardUserJson>,
    pub see_comment_exclude_users:    Vec<CardUserJson>,
    pub create_el_exclude_users:      Vec<CardUserJson>,
    pub create_comment_exclude_users: Vec<CardUserJson>,
    pub copy_el_exclude_users:        Vec<CardUserJson>,
    pub see_el_include_users:         Vec<CardUserJson>,
    pub see_comment_include_users:    Vec<CardUserJson>,
    pub create_el_include_users:      Vec<CardUserJson>,
    pub create_comment_include_users: Vec<CardUserJson>,
    pub copy_el_include_users:        Vec<CardUserJson>,
}

#[derive(Serialize, Deserialize)]
pub struct DataListJson {
    pub token:                Option<String>,
    pub id:                   Option<i32>,
    pub community_id:         Option<i32>,
    pub user_id:              Option<i32>,
    pub name:                 Option<String>,
    pub description:          Option<String>,
    pub image:                Option<String>,
    pub see_el:               Option<i16>,
    pub see_comment:          Option<i16>,
    pub create_el:            Option<i16>,
    pub create_comment:       Option<i16>,
    pub copy_el:              Option<i16>,
    pub reactions:            Option<String>,

    pub see_el_users:         Option<Vec<i32>>,
    pub see_comment_users:    Option<Vec<i32>>,
    pub create_el_users:      Option<Vec<i32>>,
    pub create_comment_users: Option<Vec<i32>>,
    pub copy_el_users:        Option<Vec<i32>>,
}

#[derive(Deserialize)]
// принимаем параметры для новых фото
pub struct DataNewPhoto {
    pub token:        Option<String>,
    pub list_id:      Option<i32>,
    pub server_id:    Option<i16>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub files:        Option<Vec<String>>,
}
#[derive(Deserialize)]
// принимаем параметры для редактируемого фото
pub struct DataEditPhoto {
    pub token:       Option<String>,
    pub id:          Option<i32>,
    pub user_id:     Option<i32>,
    pub description: Option<String>,
}
#[derive(Serialize)]
// отдаем пост
pub struct RespPhoto {
    pub id:           i32,
    pub list_id:      i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub server_id:    i16, 
    pub file:         String,
    pub position:     i16, 
}

#[derive(Deserialize)]
// принимаем параметры для нового коммента
pub struct DataNewComment {
    pub token:        Option<String>,
    pub item_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub content:      Option<String>,
    pub parent_id:    Option<i32>,
    pub attachments:  Option<String>,
}
#[derive(Deserialize)]
// принимаем параметры для редактируемого коммента
pub struct DataEditComment {
    pub token:       Option<String>,
    pub id:          Option<i32>,
    pub user_id:     Option<i32>,
    pub content:     Option<String>,
    pub attachments: Option<String>,
}
#[derive(Serialize)]
// отдаем коммент
pub struct RespComment {
    pub id:           i32,
    pub photo_id:     i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub content:      Option<String>,
    pub parent_id:    Option<i32>,
    pub attachments:  Option<AttachmentsJson>,
}

#[derive(Serialize, Deserialize)]
pub struct RespListJson {
    pub id:             i32,
    pub community_id:   Option<i32>,
    pub user_id:        i32,
    pub name:           String,
    pub description:    Option<String>,
    pub image:          Option<String>,
    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,
    pub reactions:      Option<String>,
}

#[derive(Deserialize)]
pub struct JsonPosition {
    pub key:   i32,
    pub value: i16,
}
#[derive(Serialize, Deserialize)]
pub struct NewListValues {
    pub pk:    i32,
    pub name:  String,
    pub image: Option<String>,
}
#[derive(Deserialize,Serialize)]
pub struct JsonItemReactions {
    pub count:     i32,  // кол-во отреагировавших реакцией
    pub reactions: i32,  // кол-во реакций объекта
}
#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}


#[derive(Serialize)]
// это объекты пользователей
pub struct UserListJson {
    pub users: Vec<CardUserJson>,
}
#[derive(Deserialize, Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}
#[derive(Serialize, Queryable)]
// это объект пользователя
pub struct CardCommunityJson {
    pub id:    i32,
    pub name:  String,
    pub link:  String,
    pub image: Option<String>,
}
// это объект данных владельца объекта
#[derive(Serialize, Queryable)]
pub struct CardOwnerJson {
    pub name:  String,
    pub link:  String,
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct ObjectData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub id:      Option<i32>,
}

#[derive(Serialize)]
// это для пагинации
pub struct PhotoListsJson {
    pub lists: Vec<CardPhotoListJson>,
}
#[derive(Serialize)]
// это объект списка фото
pub struct CardPhotoListJson {
    pub name:        String,
    pub owner_name:  String, 
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub image:       Option<String>,
    pub types:       String,           // например cpo1
    pub count:       i32,
}

// это объект списка фото (подгружается по нажатию на список)
#[derive(Serialize)]
pub struct PhotoListDetailJson {
    pub id:                i32,
    pub name:              String,
    pub owner_name:        String,
    pub owner_link:        String,
    pub owner_image:       Option<String>,
    pub image:             Option<String>,
    pub types:             i16,
    pub count:             i32,
    pub reactions_list:    Vec<i32>,
    pub photos:            Vec<CardPhotoJson>,
    pub lists:             Vec<CardPhotoListJson>,
    pub is_user_create_el: bool,
}

// это объект страницы фото (подгружается по нажатию на список)
#[derive(Serialize)]
pub struct PhotoListPageJson {
    pub selected_list_id: i32,               // id подгружаемого списка
    pub owner_name:       String,            // чья страница
    pub owner_link:       String,            // сслыка на владельца
    pub owner_image:      Option<String>,    // фото владельца
    pub image:            Option<String>,    // аватар списка
    pub lists:            Vec<CardPhotoListJson>, // списки фото для карточек
}

//////////// Сериализаторы фото
#[derive(Serialize)]
// это объект фото
pub struct PhotosJson {
    pub photos: Vec<CardPhotoJson>,
}

#[derive(Serialize)]
// это запись для API, выдавать для создания прикрепов в других сервисах
pub struct AttachPhoto {
    pub id:        i32, 
    pub server_id: i16,
    pub file:      String,
}

#[derive(Serialize)]
// это карточка фото
pub struct CardPhotoJson { 
    pub id:        i32, 
    pub server_id: i16,
    pub file:      String,
    pub position:  i16,
}

#[derive(Serialize)]
pub struct EditPhotoJson {
    pub description: Option<String>,
    pub comments_on: bool,
}
#[derive(Serialize)]
// это фото
pub struct PhotoDetailJson { 
    pub file:                 String,
    pub description:          Option<String>,
    pub owner_name:           String, 
    pub owner_link:           String,
    pub owner_image:          Option<String>,
    pub comments_on:          bool,
    pub created:              String,
    pub comment:              i32,
    pub view:                 i32,
    pub repost:               i32,
    pub reactions:            i32,
    pub reactions_list:       Option<Vec<SmallReactionBlockJson>>, // блок реакции (6 объектов)
    pub prev:                 Option<i32>,
    pub next:                 Option<i32>,
    pub is_user_see_comments: bool,
    pub is_user_create_comments:bool,
    pub comments:             Vec<CardCommentJson>,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionBlockJson {
    pub count:         i32,                       // кол-во отреагировавших
    pub reaction:      i32,                       // id реакции
    pub users:         Vec<CardReactionPhotoJson>, // отреагировавшие
    pub user_react_id: Option<i32>,               // id реакции текущего пользователя, на которую он реагировал
}
#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct SmallReactionBlockJson {
    pub count:         i32, // кол-во отреагировавших
    pub reaction:      i32, // id реакции
    pub user_react_id: i32, // id реакции текущего пользователя, на которую он реагировал
}

#[derive(Serialize, Queryable)]
// // это карточка того, кто поставил реакцию
pub struct CardReactionPhotoJson {
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
}
////////////////////////

//////////// Сериализаторы комментов
#[derive(Serialize)]
// это объекты комментов
pub struct CommentsJson {
    pub reactions_list: Vec<CardReactionPhotoJson>,
    pub comments:       Vec<CardCommentJson>,
}

#[derive(Serialize)]
// это объекты ответов
pub struct RepliesJson {
    pub reactions_list: Vec<CardReactionPhotoJson>,
    pub replies:        Vec<CardReplyJson>,
}
#[derive(Serialize)]
// это объекты ответов для встраивания
pub struct RepliesSmallJson {
    pub replies: Vec<CardReplyJson>,
}

#[derive(Serialize, Queryable)]
// это коммент
pub struct CardCommentJson {
    pub content:        Option<String>,
    pub owner_name:     String,
    pub owner_link:     String,
    pub owner_image:    Option<String>,
    pub created:        String,
    pub reactions:      i32,
    pub types:          String, // например cpo1
    pub replies:        i32,    // кол-во ответов
    pub reactions_list: Option<Vec<SmallReactionBlockJson>>, // блок реакции (6 объектов)
    pub attachments:    Option<AttachmentsJson>,
}
#[derive(Serialize, Queryable)]
// это ответ на коммент
pub struct CardReplyJson {
    pub content:        Option<String>,
    pub owner_name:     String,
    pub owner_link:     String,
    pub owner_image:    Option<String>,
    pub created:        String,
    pub reactions:      i32,
    pub types:          String, // например cpo1 - ответ
    pub reactions_list: Option<Vec<SmallReactionBlockJson>>, // блок реакции (6 объектов)
    pub attachments:    Option<AttachmentsJson>,
}

#[derive(Serialize, Queryable)]
// // это карточка того, кто поставил реакцию
pub struct CardReactionPhotoCommentJson {
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionsCommentJson {
    pub count:  String,
    pub users:  Vec<CardReactionPhotoJson>,
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
pub struct SearchObjectRegListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub object_id: Option<i32>,
    pub q:         Option<String>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchTargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
    pub q:         Option<String>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchObjectTargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub object_id: Option<i32>,
    pub target_id: Option<i32>,
    pub q:         Option<String>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct EditTokenPageResp {
    pub id:            i32,
    pub name:          String,
    pub is_active:     bool,
    pub item_services: Vec<OwnerService>,
    pub all_services:  Vec<OwnerService>,
} 