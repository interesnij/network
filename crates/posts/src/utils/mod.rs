use diesel::prelude::*;
use crate::schema;
use serde::{Serialize, Deserialize};
use crate::models::{
    PostList,
    Post,
    PostComment,
};

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
    pub next_page:         i32,
    pub is_user_create_el: bool,
}

// это объект страницы записей (подгружается по нажатию на список)
pub struct PostListPageJson {
    pub selected_list_id: i32,               // id подгружаемого списка
    pub owner_name:       String,            // чья страница
    pub owner_link:       String,            // сслыка на владельца
    pub owner_image:      Option<String>,    // фото владельца
    pub image:            Option<String>,    // аватар списка
    pub lists:            Vec<CardPostListJson>, // списки записей для карточек
    pub next_page:        i32,               // а есть ли следующая порция списков?
}

#[derive(Serialize)]
// это инфо о тех, кто репостил, и цифры
pub struct RepostsPostListJson {
    pub message_reposts: String,
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
// это запись
pub struct CardPostJson {
    pub id:              i32,
    pub content:         Option<String>,
    pub owner_name:      String,
    pub owner_link:      String,
    pub owner_image:     Option<String>,
    pub comment_enabled: bool,
    pub created:         String,
    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub is_signature:    bool,
    pub reactions:       i32,
    pub types:           String,                         // например pos1
    pub parent:          Option<CardParentPostJson>,     // пост родитель
    pub reposts:         Option<RepostsPostJson>,        // кто репостил пост (6 объектов)
    pub reactions_list:  Option<Vec<ReactionBlockJson>>, // блок реакции (6 объектов)
    pub items:           Option<()>,
}


#[derive(Serialize)]
// это запись
pub struct PostDetailJson {
    // see_list не упоминаем, в случае неполных прав вернем статус 403
    pub content:              Option<String>,
    pub owner_name:           String,
    pub owner_link:           String,
    pub owner_image:          Option<String>,
    pub comment_enabled:      bool,
    pub created:              String,
    pub comment:              i32,
    pub view:                 i32,
    pub repost:               i32,
    pub is_signature:         bool,
    pub reactions:            i32,
    pub types:                String,                         // например pos1
    pub parent:               Option<CardParentPostJson>,     // пост родитель
    pub reposts:              Option<RepostsPostJson>,        // кто репостил пост (6 объектов)
    pub reactions_list:       Option<Vec<ReactionBlockJson>>, // блок реакции (6 объектов)
    pub prev:                 Option<i32>,
    pub next:                 Option<i32>,
    pub is_user_see_comments: bool,
    pub is_user_create_el:    bool,
    pub comments:             CommentsSmallJson,
    pub items:                Option<()>,
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
    pub items:       Option<()>,
}
#[derive(Serialize)]
// это инфо о тех, кто репостил, и цифры
pub struct RepostsPostJson {
    pub message_reposts: String,
    pub copy_count:      String,
    pub posts:           Vec<CardOwnerJson>,
}

#[derive(Serialize, Queryable)]
// это инфо о тех, кто реагировал и общее количество у реакции
pub struct ReactionBlockJson {
    pub count:    i32,
    pub reaction: i32,
    pub users:    Vec<CardReactionPostJson>,
}
#[derive(Serialize, Queryable)]
// // это карточка того, кто поставил реакцию
pub struct CardReactionPostJson {
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
    pub reactions_list: Vec<CardReactionPostJson>,
    pub comments:       Vec<CardCommentJson>,
    pub next_page:      i32,
}
#[derive(Serialize)]
// это объекты комментов для встраивания
pub struct CommentsSmallJson {
    pub comments:       Vec<CardCommentJson>,
    pub next_page:      i32,
}

#[derive(Serialize)]
// это объекты ответов
pub struct RepliesJson {
    pub reactions_list: Vec<CardReactionPostJson>,
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
    pub items:          Option<()>,
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
    pub items:          Option<()>,
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

////////////////////////


pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_count_for_ru(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}
pub fn get_count_for_ru_alt(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    if a == 1 && b != 11 {
        return word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return word2;
    }
    else {
        return word3;
    }
}

pub fn get_post_list(pk: i32) -> PostList {
    use crate::schema::post_lists::dsl::post_lists;
    let _connection = establish_connection();
    return post_lists
        .filter(schema::post_lists::id.eq(pk))
        .load::<PostList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_post(pk: i32) -> Post {
    use crate::schema::posts::dsl::posts;
    let _connection = establish_connection();
    return posts
        .filter(schema::posts::id.eq(pk))
        .load::<Post>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_post_comment(pk: i32) -> PostComment {
    use crate::schema::post_comments::dsl::post_comments;
    let _connection = establish_connection();
    return post_comments
        .filter(schema::post_comments::id.eq(pk))
        .load::<PostComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
