use serde::{Serialize, Deserialize};
use crate::utils::AttachmentsJson;
use crate::models::{
    OwnerService,
};


#[derive(Serialize)]
pub struct KeyValue {
    pub value: i16,
    pub info:  String,
}
#[derive(Serialize)]
pub struct UserEditPrivateResp {
    pub see_all:              KeyValue,
    pub see_el:               KeyValue,
    pub see_comment:          KeyValue,
    pub create_el:            KeyValue,
    pub create_comment:       KeyValue,
    pub copy_el:              KeyValue,
    pub see_all_users:        Option<Vec<CardUserJson>>,
    pub see_el_users:         Option<Vec<CardUserJson>>,
    pub see_comment_users:    Option<Vec<CardUserJson>>,
    pub create_el_users:      Option<Vec<CardUserJson>>,
    pub create_comment_users: Option<Vec<CardUserJson>>,
    pub copy_el_users:        Option<Vec<CardUserJson>>,
}
#[derive(Serialize)]
pub struct CommunityEditPrivateResp {
    pub see_el:               KeyValue,
    pub see_comment:          KeyValue,
    pub create_el:            KeyValue,
    pub create_comment:       KeyValue,
    pub copy_el:              KeyValue,
    pub see_el_users:         Option<Vec<CardUserJson>>,
    pub see_comment_users:    Option<Vec<CardUserJson>>,
    pub create_el_users:      Option<Vec<CardUserJson>>,
    pub create_comment_users: Option<Vec<CardUserJson>>,
    pub copy_el_users:        Option<Vec<CardUserJson>>, 
}

#[derive(Serialize)]
pub struct EditNotifyResp { 
    pub comment:               KeyValue,
    pub comment_reply:         KeyValue,
    pub mention:               KeyValue,
    pub comment_mention:       KeyValue,
    pub repost:                KeyValue,
    pub reactions:             KeyValue,
    pub comment_users:         Option<Vec<CardUserJson>>,
    pub comment_reply_users:   Option<Vec<CardUserJson>>,
    pub mention_users:         Option<Vec<CardUserJson>>,
    pub comment_mention_users: Option<Vec<CardUserJson>>,
    pub repost_users:          Option<Vec<CardUserJson>>,
    pub reactions_users:       Option<Vec<CardUserJson>>,
}

#[derive(Deserialize)]
pub struct TokenParams {
    pub token: Option<String>,
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
pub struct ItemParams {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
    pub id:           Option<i32>,
}
#[derive(Deserialize)]
pub struct DataCopyPost {
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
pub struct SmallCommunityData {
    pub token:        Option<String>,
    pub community_id: Option<i32>,
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
// принимаем параметры для нового поста
pub struct DataNewPost {
    pub token:        Option<String>,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub content:      Option<String>,
    pub attachments:  Option<String>,
    pub comments_on:  bool,
    pub is_signature: bool,
    pub parent_id:    Option<i32>,
}
#[derive(Deserialize)]
// принимаем параметры для редактируемого поста
pub struct DataEditPost {
    pub token:        Option<String>,
    pub id:           Option<i32>,
    pub user_id:      Option<i32>,
    pub content:      Option<String>,
    pub attachments:  Option<String>,
    pub comments_on:  bool,
    pub is_signature: bool,
}
#[derive(Serialize)]
// отдаем пост
pub struct RespPost {
    pub id:           i32,
    pub list_id:      i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub content:      Option<String>,
    pub attach:       Option<String>,
    pub comments_on:  bool,
    pub is_signature: bool,
    pub parent_id:    Option<i32>,
    pub attachments:  Option<AttachmentsJson>,
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
    pub post_id:      i32,
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
pub struct PostListsJson {
    pub lists: Vec<CardPostListJson>,
}
#[derive(Serialize)]
// это объект списка записей
pub struct CardPostListJson {
    pub name:        String,
    pub owner_name:  String, 
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub image:       Option<String>,
    pub types:       String,           // например cpo1
    pub count:       i32,
}

// это объект списка записей (подгружается по нажатию на список)
#[derive(Serialize)]
pub struct PostListDetailJson {
    // see_list не упоминаем, в случае неполных прав вернем статус 403
    pub id:                i32,
    pub name:              String,
    pub owner_name:        String,
    pub owner_link:        String,
    pub owner_image:       Option<String>,
    pub image:             Option<String>,
    pub types:             i16,             // здесь просто тип, остальное на месте пририсуем, а такой тип нужен так и так
    pub count:             i32,
    pub reactions_list:    Vec<i32>,
    pub posts:             Vec<CardPostJson>,
    pub lists:             Vec<CardPostListJson>,
    pub is_user_create_el: bool,
}

// это объект страницы записей (подгружается по нажатию на список)
#[derive(Serialize)]
pub struct PostListPageJson {
    pub selected_list_id: i32,               // id подгружаемого списка
    pub owner_name:       String,            // чья страница
    pub owner_link:       String,            // сслыка на владельца
    pub owner_image:      Option<String>,    // фото владельца
    pub image:            Option<String>,    // аватар списка
    pub lists:            Vec<CardPostListJson>, // списки записей для карточек
}

#[derive(Serialize)]
// это инфо о тех, кто репостил, и цифры
pub struct RepostsPostListJson {
    pub message_reposts: usize,
    pub copy_count:      i32,
    pub posts:           Vec<CardUserJson>,
}
////////////////////////

//////////// Сериализаторы записей
#[derive(Serialize)]
// это объект записи
pub struct PostsJson {
    pub posts: Vec<CardPostJson>,
}

#[derive(Serialize)]
// это запись для API, выдавать для создания прикрепов в других сервисах
pub struct AttachPost {
    pub id:             i32,
    pub content:        Option<String>,
    pub comments_on:    bool,
    pub created:        String,
    pub comment:        i32,
    pub view:           i32,
    pub repost:         i32,
    pub is_signature:   bool,
    pub reactions:      i32,
    pub types:          String,                          // например pos1
    pub parent:         Option<CardParentPostJson>,     // пост родитель
    pub reactions_list: Option<Vec<SmallReactionBlockJson>>,
    pub attachments:    Option<AttachmentsJson>,
}

#[derive(Serialize)]
// это запись
pub struct CardPostJson {
    pub id:             i32,
    pub content:        Option<String>,
    pub owner_name:     String,
    pub owner_link:     String,
    pub owner_image:    Option<String>,
    pub comments_on:    bool,
    pub created:        String,
    pub comment:        i32,
    pub view:           i32,
    pub repost:         i32,
    pub is_signature:   bool,
    pub reactions:      i32,
    pub types:          String,                          // например pos1
    pub parent:         Option<CardParentPostJson>,     // пост родитель
    pub reactions_list: Option<Vec<SmallReactionBlockJson>>,
    pub attachments:    Option<AttachmentsJson>,
}

#[derive(Serialize)]
pub struct EditPostJson {
    pub content:      Option<String>,
    pub comments_on:  bool,
    pub is_signature: bool,
    pub attachments:  Option<AttachmentsJson>,
}
#[derive(Serialize)]
// это запись
pub struct PostDetailJson {
    // see_list не упоминаем, в случае неполных прав вернем статус 403
    pub content:              Option<String>,
    pub owner_name:           String,
    pub owner_link:           String,
    pub owner_image:          Option<String>,
    pub comments_on:          bool,
    pub created:              String,
    pub comment:              i32,
    pub view:                 i32,
    pub repost:               i32,
    pub is_signature:         bool,
    pub reactions:            i32,
    pub types:                String,                         // например pos1
    pub parent:               Option<CardParentPostJson>,     // пост родитель
    pub reposts:              RepostsPostJson,                // кто репостил пост (6 объектов)
    pub reactions_list:       Option<Vec<SmallReactionBlockJson>>, // блок реакции (6 объектов)
    pub prev:                 Option<i32>,
    pub next:                 Option<i32>,
    pub is_user_see_comments: bool,
    pub is_user_create_comments:bool,
    pub comments:             Vec<CardCommentJson>,
    pub attachments:          Option<AttachmentsJson>,
}

#[derive(Serialize)]
// это объект запись репост
pub struct CardParentPostJson {
    pub id:          i32,
    pub content:     Option<String>,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub created:     String,
    pub attachments: Option<AttachmentsJson>,
}
#[derive(Serialize)]
// это инфо о тех, кто репостил, и цифры
pub struct RepostsPostJson {
    pub message_reposts: usize,
    pub creators:        Vec<CardOwnerJson>,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionBlockJson {
    pub count:         i32,                       // кол-во отреагировавших
    pub reaction:      i32,                       // id реакции
    pub users:         Vec<CardReactionPostJson>, // отреагировавшие
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
pub struct CardReactionPostJson {
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
}
////////////////////////

//////////// Сериализаторы комментов
#[derive(Serialize)]
// это объекты комментов
pub struct CommentsJson {
    pub reactions_list: Vec<CardReactionPostJson>,
    pub comments:       Vec<CardCommentJson>,
}

#[derive(Serialize)]
// это объекты ответов
pub struct RepliesJson {
    pub reactions_list: Vec<CardReactionPostJson>,
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
pub struct CardReactionPostCommentJson {
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionsCommentJson {
    pub count:  String,
    pub users:  Vec<CardReactionPostJson>,
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