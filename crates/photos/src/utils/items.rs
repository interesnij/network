use serde::{Serialize, Deserialize};
use crate::utils::AttachmentsJson;


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
//////////// Сериализаторы списков записей

#[derive(Serialize)]
// это для пагинации
pub struct PhotoListsJson {
    pub lists: Vec<CardPhotoListJson>,
}
#[derive(Serialize)]
// это объект списка записей
pub struct CardPhotoListJson {
    pub name:        String,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub image:       Option<String>,
    pub types:       String,           // например cpo1
    pub count:       i32,
}

// это объект списка записей (подгружается по нажатию на список)
pub struct PhotoListDetailJson {
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
    pub photos:             Vec<CardPhotoJson>,
    pub lists:             Vec<CardPhotoListJson>,
    pub next_page:         i32,
    pub is_user_create_el: bool,
}

// это объект страницы записей (подгружается по нажатию на список)
pub struct PhotoListPageJson {
    pub selected_list_id: i32,               // id подгружаемого списка
    pub owner_name:       String,            // чья страница
    pub owner_link:       String,            // сслыка на владельца
    pub owner_image:      Option<String>,    // фото владельца
    pub image:            Option<String>,    // аватар списка
    pub lists:            Vec<CardPhotoListJson>, // списки записей для карточек
    pub next_page:        i32,               // а есть ли следующая порция списков?
}

//////////// Сериализаторы записей
#[derive(Serialize)]
// это объект записи
pub struct PhotosJson {
    pub photos: Vec<CardPhotoJson>,
}

#[derive(Serialize)]
// это карточка фото
pub struct CardPhotoJson {
    pub id:      i32,
    pub preview: String,
    pub file:    String,
}

#[derive(Serialize)]
// это запись
pub struct PhotoDetailJson {
    // see_list не упоминаем, в случае неполных прав вернем статус 403
    pub owner_name:           String,
    pub owner_link:           String,
    pub owner_image:          Option<String>,
    pub preview:              String,
    pub file:                 String,
    pub description:          Option<String>,
    pub comments_on:          bool,
    pub created:              String,
    pub comment:              i32,
    pub view:                 i32,
    pub repost:               i32,
    pub reactions:            i32,
    pub reactions_list:       Option<Vec<ReactionBlockJson>>, // блок реакции (6 объектов)
    pub prev:                 Option<i32>,
    pub next:                 Option<i32>,
    pub is_user_see_comments: bool,
    pub is_user_create_el:    bool,
    pub comments:             CommentsSmallJson,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionBlockJson {
    pub count:    i32,
    pub reaction: i32,
    pub users:    Vec<CardReactionPhotoJson>,
}
#[derive(Serialize, Queryable)]
// // это карточка того, кто поставил реакцию
pub struct CardReactionPhotoJson {
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
    pub is_user_reaction: bool,
}
////////////////////////

//////////// Сериализаторы комментов
#[derive(Serialize)]
// это объекты комментов
pub struct CommentsJson {
    pub reactions_list: Vec<CardReactionPhotoJson>,
    pub comments:       Vec<CardCommentJson>,
    pub next_page:      i32,
}

#[derive(Serialize)]
// это объекты ответов
pub struct RepliesJson {
    pub reactions_list: Vec<CardReactionPhotoJson>,
    pub replies:        Vec<CardReplyJson>,
    pub next_page:      i32,
}
#[derive(Serialize)]
// это объекты ответов для встраивания
pub struct RepliesSmallJson {
    pub replies:        Vec<CardReplyJson>,
    pub next_page:      i32,
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
    pub reactions_list: Option<Vec<ReactionBlockJson>>, // блок реакции (6 объектов)
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
    pub reactions_list: Option<Vec<ReactionBlockJson>>, // блок реакции (6 объектов)
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
