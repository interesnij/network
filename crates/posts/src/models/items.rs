use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use crate::schema::{
    item_lists,
    item_comments,
    //item_posts,
    item_photos,
    item_docs,
    item_goods,
    item_articles,
    item_wikis,
    item_forums,
    item_audios,
    item_surveys,
    item_videos,
    item_sites,
    attach_items,
};

use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
};
//use actix_web::web::Json;

/////// ItemList //////
// list_types ↓
// 20 Список записей (не создаем, он и так есть на этом сервисе)
// 21 Плейлист
// 22 Список документов
// 23 Список опросов
// 24 Список фотографий
// 25 Список роликов
// 26 Список товаров
// 27 Список обсуждений
// 28 Список википедии
// 29 Список статей
// 30 Папка
// 31 Список стикеров

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemList {
    pub id:             i32,
    pub name:           String,
    pub user_id:        i32,
    pub community_id:   Option<i32>,
    pub list_id:        i32,
    pub list_types:     i16,
    pub types:          i16,
    pub image:          Option<String>,
    pub count:          i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_lists"]
pub struct NewItemList {
    pub name:           String,
    pub user_id:        i32,
    pub community_id:   Option<i32>,
    pub list_id:        i32,
    pub list_types:     i16,
    pub types:          i16,
    pub image:          Option<String>,
    pub count:          i32,
}

/////// ItemComment //////
// comment_types ↓
// 81 Коммент к записи     (не создаем, он и так есть на этом сервисе)
// 82 Коммент к фотографии
// 83 Коммент к ролику
// 84 Коммент к товару
// 85 Коммент к обсуждению
// 86 Коммент к статье википедии
// 87 Коммент форума

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemComment {
    pub id:            i32,
    pub user_id:       i32,
    pub community_id:  Option<i32>,
    pub sticker_id:    Option<i32>,
    pub content:       Option<String>,
    pub attach:        Option<String>,
    pub comment_id:    i32,
    pub comment_types: i16,
    pub item_id:       i32,
    pub item_types:    i16,
    pub types:         i16,
    pub created:       chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_comments"]
pub struct NewItemComment {
    pub user_id:       i32,
    pub community_id:  Option<i32>,
    pub sticker_id:    Option<i32>,
    pub content:       Option<String>,
    pub attach:        Option<String>,
    pub comment_id:    i32,
    pub comment_types: i16,
    pub item_id:       i32,
    pub item_types:    i16,
    pub types:         i16,
    pub created:       chrono::NaiveDateTime,
}

/////// ItemPhotos //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemPhotos {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub preview:      String,
    pub file:         String,
    pub types:        i16,

}
#[derive(Deserialize, Insertable)]
#[table_name="item_photos"]
pub struct NewItemComment {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub preview:      String,
    pub file:         String,
    pub types:        i16,
}

/////// ItemDoc //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemDoc {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub file:         String,

}
#[derive(Deserialize, Insertable)]
#[table_name="item_docs"]
pub struct NewItemDoc {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub file:         String,
}

/////// ItemGood //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemGood {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub price:        Option<i32>,
    pub types:        i16,
    pub image:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_goods"]
pub struct NewItemGood {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub price:        Option<i32>,
    pub types:        i16,
    pub image:        Option<String>,
}

/////// ItemArticle //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemArticle {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_articles"]
pub struct NewItemArticle {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
}

/////// ItemWiki //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemWiki {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_wikis"]
pub struct NewItemWiki {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
}

/////// ItemForum //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemForum {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_forums"]
pub struct NewItemForum {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
}

/////// ItemAudio //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemAudio {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub artist_id:    Option<i32>,
    pub types:        i16,
    pub file:         String,
    pub image:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_audios"]
pub struct NewItemAudio {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub artist_id:    Option<i32>,
    pub types:        i16,
    pub file:         String,
    pub image:        Option<String>,
}

/////// ItemSurvey //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemSurvey {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
    pub is_anonymous: bool,
    pub is_multiple:  bool,
    pub is_no_edited: bool,
    pub time_end:     Option<chrono::NaiveDateTime>,
    pub vote:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_surveys"]
pub struct NewItemSurvey {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
    pub is_anonymous: bool,
    pub is_multiple:  bool,
    pub is_no_edited: bool,
    pub time_end:     Option<chrono::NaiveDateTime>,
    pub vote:         i32,
}


/////// ItemVideo //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemVideo {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
    pub file:         String,
    pub view:         i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_videos"]
pub struct NewItemVideo {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub types:        i16,
    pub image:        Option<String>,
    pub file:         String,
    pub view:         i32,
}

/////// ItemSite //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemSite {
    pub id:           i32,
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub item_id:      i32,
    pub types:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_sites"]
pub struct NewItemSite {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub item_id:      i32,
    pub types:        i16,
}

/////// AttachItem //////
// типы объектов (item_types)
// 1  Пользователь item_users
// 2  Сообщество   item_communitys
// 3  Сайт         item_sites
// 20 Список записей    item_lists
// 21 Плейлист          item_lists
// 22 Список документов item_lists
// 23 Список опросов    item_lists
// 24 Список фотографий item_lists
// 25 Список роликов    item_lists
// 26 Список товаров    item_lists
// 27 Список обсуждений item_lists
// 28 Список википедии  item_lists
// 29 Список статей     item_lists
// 30 Папка             item_lists
// 31 Список стикеров   item_lists
// 51 Запись       item_posts
// 52 Трек         item_audios
// 53 Документ     item_docs
// 54 Опрос        item_surveys
// 55 Фотография   item_photos
// 56 Ролик        item_videos
// 57 Товар        item_goods
// 58 Обсуждение   item_forums
// 59 Статья вики  item_wikis
// 60 Статья       item_articles
// 81 Коммент к записи           item_comments
// 82 Коммент к фотографии       item_comments
// 83 Коммент к ролику           item_comments
// 84 Коммент к товару           item_comments
// 85 Коммент к обсуждению       item_comments
// 86 Коммент к статье википедии item_comments
// 87 Коммент форума             item_comments

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct AttachItem {
    pub id:           i32,
    pub item_id:      i32, // id объекта (которые выше)
    pub item_types:   i16, // тип связанного объекта (которые выше)
    pub attach_types: i16, // к чему прикреплен объект (коммент, запись)
}
#[derive(Deserialize, Insertable)]
#[table_name="attach_items"]
pub struct NewAttachItem {
    pub item_id:      i32, // id объекта (которые выше)
    pub item_types:   i16, // тип связанного объекта (которые выше)
    pub attach_types: i16, // к чему прикреплен объект (коммент, запись)
}
