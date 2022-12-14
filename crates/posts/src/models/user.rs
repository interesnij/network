use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    CardPostJson,
    CardUserJson, EditNotifyResp,
    CardPostListJson, KeyValue,
    AttachOwner, UserEditPrivateResp, 
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgTextExpressionMethods,
    PgConnection,
};

use crate::schema;
use crate::schema::{
    users,
    friends,
    follows,
    user_visible_perms,
};
use crate::errors::Error;
use crate::models::{Post, PostList, SearchAllComments, UserPostNotification};

/*
Типы пользоватетеля
Типы пользоватетеля
1 стандартный тип пользователя
2 стандартный пославший запрос на идентификацию
3 стандартный идентифицированный
4 детский тип пользователя
5 детский пославший запрос на идентификацию
6 детский идентифицированный

10 TRAINEE_MODERATOR
11 MODERATOR
12 HIGH_MODERATOR
13 TEAMLEAD_MODERATOR
14 TRAINEE_MANAGER
15 MANAGER
16 HIGH_MANAGER
17 TEAMLEAD_MANAGER
18 ADVERTISER
19 HIGH_ADVERTISER
20 TEAMLEAD_ADVERTISER
21 ADMINISTRATOR
22 HIGH_ADMINISTRATOR
23 TEAMLEAD_ADMINISTRATOR
25 SUPERMANAGER

31 удаленный стандартный
32 удаленный пославший запрос на идентификацию
33 удаленный идентифицированный
34 удаленный ребенок
35 удаленный ребенок пославший запрос на идентификацию
36 удаленный ребенок идентифицированный

41 закрытый стандартный
42 закрытый пославший запрос на идентификацию
43 закрытый идентифицированный
44 закрытый ребенок
45 закрытый ребенок пославший запрос на идентификацию
46 закрытый ребенок идентифицированный

51 приостановленный стандартный
52 приостановленный пославший запрос на идентификацию
53 приостановленный идентифицированный
54 приостановленный ребенок
55 приостановленный ребенок пославший запрос на идентификацию
56 приостановленный ребенок идентифицированный

61 закрытый баннером стандартный
62 закрытый баннером пославший запрос на идентификацию
63 закрытый баннером идентифицированный
64 приостановленный ребенок
65 приостановленный ребенок пославший запрос на идентификацию
66 приостановленный ребенок идентифицированный

приватность
1 Все пользователи
2 Все друзья и все подписчики
3 Все друзья и подписчики, кроме
4 Все друзья и некоторые подписчики
5 Все подписчики и друзья, кроме
6 Все подписчики и некоторые друзья
7 Все друзья
8 Все подписчики
9 Друзья, кроме
10 Некоторые друзья
11 Подписчики, кроме
12 Некоторые подписчики
13 Только я

31 Все друзья и списки подписчиков, кроме
32 Все друзья и некоторые списки подписчиков
33 Все подписчики и списки друзей, кроме
34 Все подписчики и некоторые списки друзей
35 Списки друзей, кроме
36 Некоторые списки друзей
37 Списки подписчиков, кроме
38 Некоторые списки подписчиков
*/
#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:             i32,
    pub user_id:        i32,
    pub first_name:     String,
    pub last_name:      String,
    pub types:          i16,
    pub is_man:         bool,
    pub password:       String,
    pub link:           String,
    pub s_avatar:       Option<String>,
    pub last_activity:  chrono::NaiveDateTime,

    pub see_all:        i16,
    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_id:        i32,
    pub first_name:     String,
    pub last_name:      String,
    pub types:          i16,
    pub is_man:         bool,
    pub password:       String,
    pub link:           String,
    pub s_avatar:       Option<String>,
    pub last_activity:  chrono::NaiveDateTime,

    pub see_all:        i16,
    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserJson {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub types:      Option<i16>,
    pub is_man:     Option<i16>,
    pub password:   Option<String>,
    pub link:       Option<String>,
}

impl User {
    pub fn get_notify_field(value: i16) -> KeyValue {
        let info = match value {
            1 => "Все пользователи",
            2 => "Все друзья и все подписчики",
            3 => "Все друзья и подписчики, кроме",
            4 => "Все друзья и некоторые подписчики",
            5 => "Все подписчики и друзья, кроме",
            6 => "Все подписчики и некоторые друзья",
            7 => "Все друзья",
            8 => "Все подписчики",
            9 => "Друзья, кроме",
            10 => "Некоторые друзья",
            11 => "Подписчики, кроме",
            12 => "Некоторые подписчики",
            13 => "Никто",
            _ => "Ошибка",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    } 
    pub fn get_private_field(value: i16) -> KeyValue {
        let info = match value {
            1 => "Все пользователи",
            2 => "Все друзья и все подписчики",
            3 => "Все друзья и подписчики, кроме",
            4 => "Все друзья и некоторые подписчики",
            5 => "Все подписчики и друзья, кроме",
            6 => "Все подписчики и некоторые друзья",
            7 => "Все друзья",
            8 => "Все подписчики",
            9 => "Друзья, кроме",
            10 => "Некоторые друзья",
            11 => "Подписчики, кроме",
            12 => "Некоторые подписчики",
            13 => "Только я",
            _ => "Ошибка",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    }
    
    pub fn get_private_json(&self) -> UserEditPrivateResp {
        let see_all_users:        Option<Vec<CardUserJson>>;
        let see_el_users:         Option<Vec<CardUserJson>>;
        let see_comment_users:    Option<Vec<CardUserJson>>;
        let create_el_users:      Option<Vec<CardUserJson>>;
        let create_comment_users: Option<Vec<CardUserJson>>;
        let copy_el_users:        Option<Vec<CardUserJson>>; 
        
        if self.see_all == 5 || self.see_all == 9 {
            see_all_users = Some(self.get_limit_see_all_exclude_friends(Some(20), Some(0)));
        }
        else if self.see_all == 3 || self.see_all == 11 {
            see_all_users = Some(self.get_limit_see_all_exclude_follows(Some(20), Some(0)));
        }
        else if self.see_all == 6 || self.see_all == 10 {
            see_all_users = Some(self.get_limit_see_all_include_friends(Some(20), Some(0)));
        }
        else if self.see_all == 4 || self.see_all == 12 {
            see_all_users = Some(self.get_limit_see_all_include_follows(Some(20), Some(0)));
        }
        else {
            see_all_users = None;
        }

        if self.see_el == 5 || self.see_el == 9 {
            see_el_users = Some(self.get_limit_see_el_exclude_friends(Some(20), Some(0)));
        }
        else if self.see_el == 3 || self.see_el == 11 {
            see_el_users = Some(self.get_limit_see_el_exclude_follows(Some(20), Some(0)));
        }
        else if self.see_el == 6 || self.see_el == 10 {
            see_el_users = Some(self.get_limit_see_el_include_friends(Some(20), Some(0)));
        }
        else if self.see_el == 4 || self.see_el == 12 {
            see_el_users = Some(self.get_limit_see_el_include_follows(Some(20), Some(0)));
        }
        else {
            see_el_users = None;
        }

        if self.see_comment == 5 || self.see_comment == 9 {
            see_comment_users = Some(self.get_limit_see_comment_exclude_friends(Some(20), Some(0)));
        }
        else if self.see_comment == 3 || self.see_comment == 11 {
            see_comment_users = Some(self.get_limit_see_comment_exclude_follows(Some(20), Some(0)));
        }
        else if self.see_comment == 6 || self.see_comment == 10 {
            see_comment_users = Some(self.get_limit_see_comment_include_friends(Some(20), Some(0)));
        }
        else if self.see_comment == 4 || self.see_comment == 12 {
            see_comment_users = Some(self.get_limit_see_comment_include_follows(Some(20), Some(0)));
        }
        else {
            see_comment_users = None;
        }

        if self.create_el == 5 || self.create_el == 9 {
            create_el_users = Some(self.get_limit_create_el_exclude_friends(Some(20), Some(0)));
        }
        else if self.create_el == 3 || self.create_el == 11 {
            create_el_users = Some(self.get_limit_create_el_exclude_follows(Some(20), Some(0)));
        }
        else if self.create_el == 6 || self.create_el == 10 {
            create_el_users = Some(self.get_limit_create_el_include_friends(Some(20), Some(0)));
        }
        else if self.create_el == 4 || self.create_el == 12 {
            create_el_users = Some(self.get_limit_create_el_include_follows(Some(20), Some(0)));
        }
        else {
            create_el_users = None;
        }

        if self.create_comment == 5 || self.create_comment == 9 {
            create_comment_users = Some(self.get_limit_create_comment_exclude_friends(Some(20), Some(0)));
        }
        else if self.create_comment == 3 || self.create_comment == 11 {
            create_comment_users = Some(self.get_limit_create_comment_exclude_follows(Some(20), Some(0)));
        }
        else if self.create_comment == 6 || self.create_comment == 10 {
            create_comment_users = Some(self.get_limit_create_comment_include_friends(Some(20), Some(0)));
        }
        else if self.create_comment == 4 || self.create_comment == 12 {
            create_comment_users = Some(self.get_limit_create_comment_include_follows(Some(20), Some(0)));
        }
        else {
            create_comment_users = None;
        }

        if self.copy_el == 5 || self.copy_el == 9 {
            copy_el_users = Some(self.get_limit_copy_el_exclude_friends(Some(20), Some(0)));
        }
        else if self.copy_el == 3 || self.copy_el == 11 {
            copy_el_users = Some(self.get_limit_copy_el_exclude_follows(Some(20), Some(0)));
        }
        else if self.copy_el == 6 || self.copy_el == 10 {
            copy_el_users = Some(self.get_limit_copy_el_include_friends(Some(20), Some(0)));
        }
        else if self.copy_el == 4 || self.copy_el == 12 {
            copy_el_users = Some(self.get_limit_copy_el_include_follows(Some(20), Some(0)));
        }
        else {
            copy_el_users = None;
        }
    
        return UserEditPrivateResp {
            see_all:              User::get_private_field(self.see_all),
            see_el:               User::get_private_field(self.see_el),
            see_comment:          User::get_private_field(self.see_comment),
            create_el:            User::get_private_field(self.create_el),
            create_comment:       User::get_private_field(self.create_comment),
            copy_el:              User::get_private_field(self.copy_el),
            see_all_users:        see_all_users,
            see_el_users:         see_el_users,
            see_comment_users:    see_comment_users,
            create_el_users:      create_el_users,
            create_comment_users: create_comment_users,
            copy_el_users:        copy_el_users,
        };
    }
    pub fn get_notify_json(&self) -> EditNotifyResp {
        let comment_users:         Option<Vec<CardUserJson>>;
        let comment_reply_users:   Option<Vec<CardUserJson>>;
        let mention_users:         Option<Vec<CardUserJson>>;
        let comment_mention_users: Option<Vec<CardUserJson>>;
        let repost_users:          Option<Vec<CardUserJson>>;
        let reactions_users:       Option<Vec<CardUserJson>>;
        
        let notify = self.get_notify_model().expect("E.");
        
        if notify.comment == 5 || notify.comment == 9 {
            comment_users = Some(self.get_limit_comment_exclude_friends(Some(20), Some(0)));
        }
        else if notify.comment == 3 || notify.comment == 11 {
            comment_users = Some(self.get_limit_comment_exclude_follows(Some(20), Some(0)));
        }
        else if notify.comment == 6 || notify.comment == 10 {
            comment_users = Some(self.get_limit_comment_include_friends(Some(20), Some(0)));
        }
        else if notify.comment == 4 || notify.comment == 12 {
            comment_users = Some(self.get_limit_comment_include_follows(Some(20), Some(0)));
        }
        else {
            comment_users = None;
        }

        if notify.comment_reply == 5 || notify.comment_reply == 9 {
            comment_reply_users = Some(self.get_limit_comment_reply_exclude_friends(Some(20), Some(0)));
        }
        else if notify.comment_reply == 3 || notify.comment_reply == 11 {
            comment_reply_users = Some(self.get_limit_comment_reply_exclude_follows(Some(20), Some(0)));
        }
        else if notify.comment_reply == 6 || notify.comment_reply == 10 {
            comment_reply_users = Some(self.get_limit_comment_reply_include_friends(Some(20), Some(0)));
        }
        else if notify.comment_reply == 4 || notify.comment_reply == 12 {
            comment_reply_users = Some(self.get_limit_comment_reply_include_follows(Some(20), Some(0)));
        }
        else {
            comment_reply_users = None;
        }

        if notify.mention == 5 || notify.mention == 9 {
            mention_users = Some(self.get_limit_mention_exclude_friends(Some(20), Some(0)));
        }
        else if notify.mention == 3 || notify.mention == 11 {
            mention_users = Some(self.get_limit_mention_exclude_follows(Some(20), Some(0)));
        }
        else if notify.mention == 6 || notify.mention == 10 {
            mention_users = Some(self.get_limit_mention_include_friends(Some(20), Some(0)));
        }
        else if notify.mention == 4 || notify.mention == 12 {
            mention_users = Some(self.get_limit_mention_include_follows(Some(20), Some(0)));
        }
        else {
            mention_users = None;
        }

        if notify.comment_mention == 5 || notify.comment_mention == 9 {
            comment_mention_users = Some(self.get_limit_comment_mention_exclude_friends(Some(20), Some(0)));
        }
        else if notify.comment_mention == 3 || notify.comment_mention == 11 {
            comment_mention_users = Some(self.get_limit_comment_mention_exclude_follows(Some(20), Some(0)));
        }
        else if notify.comment_mention == 6 || notify.comment_mention == 10 {
            comment_mention_users = Some(self.get_limit_comment_mention_include_friends(Some(20), Some(0)));
        }
        else if notify.comment_mention == 4 || notify.comment_mention == 12 {
            comment_mention_users = Some(self.get_limit_comment_mention_include_follows(Some(20), Some(0)));
        }
        else {
            comment_mention_users = None;
        }

        if notify.repost == 5 || notify.repost == 9 {
            repost_users = Some(self.get_limit_repost_exclude_friends(Some(20), Some(0)));
        }
        else if notify.repost == 3 || notify.repost == 11 {
            repost_users = Some(self.get_limit_repost_exclude_follows(Some(20), Some(0)));
        }
        else if notify.repost == 6 || notify.repost == 10 {
            repost_users = Some(self.get_limit_repost_include_friends(Some(20), Some(0)));
        }
        else if notify.repost == 4 || notify.repost == 12 {
            repost_users = Some(self.get_limit_repost_include_follows(Some(20), Some(0)));
        }
        else {
            repost_users = None;
        }

        if notify.reactions == 5 || notify.reactions == 9 {
            reactions_users = Some(self.get_limit_reactions_exclude_friends(Some(20), Some(0)));
        }
        else if notify.reactions == 3 || notify.reactions == 11 {
            reactions_users = Some(self.get_limit_reactions_exclude_follows(Some(20), Some(0)));
        }
        else if notify.reactions == 6 || notify.reactions == 10 {
            reactions_users = Some(self.get_limit_reactions_include_friends(Some(20), Some(0)));
        }
        else if notify.reactions == 4 || notify.reactions == 12 {
            reactions_users = Some(self.get_limit_reactions_include_follows(Some(20), Some(0)));
        }
        else {
            reactions_users = None;
        }
    
        return EditNotifyResp {
            comment:               User::get_notify_field(notify.comment),
            comment_reply:         User::get_notify_field(notify.comment_reply),
            mention:               User::get_notify_field(notify.mention),
            comment_mention:       User::get_notify_field(notify.comment_mention),
            repost:                User::get_notify_field(notify.repost),
            reactions:             User::get_notify_field(notify.reactions),
            comment_users:         comment_users,
            comment_reply_users:   comment_reply_users,
            mention_users:         mention_users,
            comment_mention_users: comment_mention_users,
            repost_users:          repost_users,
            reactions_users:       reactions_users,
        };
    }
    pub fn get_notify_model(&self) -> Result<UserPostNotification, Error> {
        let notify = self.find_notify_model();
        if notify.is_ok() {
            return notify;
        }
        else {
            return self.create_notify_model();
        }
    }
    pub fn create_notify_model(&self) -> Result<UserPostNotification, Error> {
        use crate::models::NewUserPostNotification;

        let _connection = establish_connection();
        let _new_notify = NewUserPostNotification {
            user_id:         self.id,
            comment:         1,
            comment_reply:   1,
            mention:         1,
            comment_mention: 1,
            repost:          1,
            reactions:       1,
        };
        let _notify = diesel::insert_into(schema::user_post_notifications::table)
            .values(&_new_notify)
            .get_result::<UserPostNotification>(&_connection)?;

        return Ok(_notify);
    }
    pub fn find_notify_model(&self) -> Result<UserPostNotification, Error> {
        use crate::schema::user_post_notifications::dsl::user_post_notifications;

        let _connection = establish_connection();
        let notify = user_post_notifications
            .filter(schema::user_post_notifications::user_id.eq(self.id))
            .first(&_connection)?;
        return Ok(notify);
    }
    pub fn get_main_post_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let _post_list = post_lists
            .filter(schema::post_lists::user_id.eq(self.id))
            .filter(schema::post_lists::types.eq(0))
            .filter(schema::post_lists::community_id.is_null())
            .first::<PostList>(&_connection);
        if _post_list.is_ok() {
            return _post_list.expect("E.");
        } 
        else {
            use crate::models::{NewPostList, NewUserPostListPosition};
            let new_list = NewPostList {
                name:            "Основной список".to_string(),
                community_id:    None,
                user_id:         self.user_id,
                types:           0,
                description:     None,
                image:           None,
                created:         chrono::Local::now().naive_utc(),
                count:           0,
                repost:          0,
                copy:            0,
                see_el:          1,
                see_comment:     1,
                create_el:       13,
                create_comment:  1,
                copy_el:         1,
                reactions:       Some("1, 2".to_string()),
            };
            let _posts_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_list)
                .get_result::<PostList>(&_connection)
                .expect("Error saving post_list.");

            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  self.id,
                list_id:  _posts_list.id,
                position: 1,
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .execute(&_connection)
                .expect("Error saving post_list_position.");
            return _posts_list;
        }
    }
    pub fn create_main_post_list(&self) -> () {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let _post_list = post_lists
            .filter(schema::post_lists::user_id.eq(self.id))
            .filter(schema::post_lists::types.eq(0))
            .filter(schema::post_lists::community_id.is_null())
            .first::<PostList>(&_connection);
 
        if _post_list.is_err() {
            use crate::models::{NewPostList, NewUserPostListPosition};
            let new_list = NewPostList {
                    name:            "Основной список".to_string(),
                    community_id:    None,
                    user_id:         self.user_id,
                    types:           0,
                    description:     None,
                    image:           None,
                    created:         chrono::Local::now().naive_utc(),
                    count:           0,
                    repost:          0,
                    copy:            0,
                    see_el:          1,
                    see_comment:     1,
                    create_el:       13,
                    create_comment:  1,
                    copy_el:         1,
                    reactions:       Some("1, 2".to_string()),
            };
            let _posts_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_list)
                .get_result::<PostList>(&_connection)
                .expect("Error saving post_list.");

            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  self.id,
                list_id:  _posts_list.id,
                position: 1,
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .execute(&_connection)
                .expect("Error saving post_list_position.");
        }
    }
    pub fn update_last_activity(&self) -> i16 {
        let _now = chrono::Local::now().naive_utc();
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::last_activity.eq(&_now))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_name(&self, first_name: &str, last_name: &str) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::users::first_name.eq(first_name),
                schema::users::last_name.eq(last_name)
            ))
            .execute(&_connection)
            .expect("E.");

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set((  
                    schema::item_users::first_name.eq(first_name),
                    schema::item_users::last_name.eq(last_name)
                ))
                .execute(&_connection);
        }
        return 1;
    }
    pub fn edit_link(&self, link: &str) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::link.eq(link))
            .execute(&_connection)
            .expect("E.");

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::link.eq(link))
                .execute(&_connection);
        }
        return 1;
    }
    pub fn edit_avatar(&self, s_avatar: &str) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::s_avatar.eq(s_avatar))
            .execute(&_connection)
            .expect("E.");

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::s_avatar.eq(s_avatar))
                .execute(&_connection);
        }
        return 1;
    }

    pub fn edit_password(&self, password: &str) -> i16 {

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::password.eq(password))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn update_staff_member(&self, community_id: i32, value: i16) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        use crate::models::CommunitiesMembership;

        let _connection = establish_connection();
        let member_res = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.user_id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .first::<CommunitiesMembership>(&_connection);
        
        if member_res.is_ok() {
            let member = member_res.expect("E.");
            diesel::update(&member)
                .set(schema::communities_memberships::level.eq(value))
                .execute(&_connection)
                .expect("E.");
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn edit_notify ( 
        &self, 
        field: &str, 
        value: i16,
        _users: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![6,7].iter().any(|&i| i==value);
        if value < 1 || value > 7 || (is_ie_mode && _users.is_none()) {
            return 0; 
        }
        
        let _connection = establish_connection();
        let notify = self.get_notify_model().expect("E.");
        let _update_field = match field {
            "comment" => diesel::update(&notify)
                .set(schema::user_post_notifications::comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "comment_reply" => diesel::update(&notify)
                .set(schema::user_post_notifications::comment_reply.eq(value))
                .execute(&_connection)
                .expect("E."),
            "mention" => diesel::update(&notify)
                .set(schema::user_post_notifications::mention.eq(value))
                .execute(&_connection)
                .expect("E."),
            "comment_mention" => diesel::update(&notify)
                .set(schema::user_post_notifications::comment_mention.eq(value))
                .execute(&_connection)
                .expect("E."),
            "repost" => diesel::update(&notify)
                .set(schema::user_post_notifications::repost.eq(value))
                .execute(&_connection)
                .expect("E."),
            "reactions" => diesel::update(&notify)
                .set(schema::user_post_notifications::reactions.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
        };
        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::user_visible_perms::dsl::user_visible_perms;
            match value { 
                51 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(61))
                    )
                    .execute(&_connection)
                    .expect("E"),
                52 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(62))
                    )
                    .execute(&_connection)
                    .expect("E"),
                53 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(63))
                    )
                    .execute(&_connection)
                    .expect("E"),
                54 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(64))
                    )
                    .execute(&_connection)
                    .expect("E"),
                55 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(65))
                    )
                    .execute(&_connection)
                    .expect("E"),
                56 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(66))
                    )
                    .execute(&_connection)
                    .expect("E"),
                61 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(51))
                    )
                    .execute(&_connection)
                    .expect("E"),
                62 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(52))
                    )
                    .execute(&_connection)
                    .expect("E"),
                63 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(53))
                    )
                    .execute(&_connection)
                    .expect("E"),
                64 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(54))
                    )
                    .execute(&_connection)
                    .expect("E"),
                65 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(55))
                    )
                    .execute(&_connection)
                    .expect("E"),
                66 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(56))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if _users.is_some() && is_ie_mode {
            for user_id in _users.unwrap().iter() {
                let _new_perm = NewUserVisiblePerm {
                    user_id:   self.user_id,
                    target_id: *user_id,
                    types:     value,
                };
                diesel::insert_into(schema::user_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        return 1;
    }
    pub fn edit_private (
        &self, 
        field:     &str, 
        value:     i16, 
        items_ids: Option<Vec<i32>>
    ) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 130 || (is_ie_mode && items_ids.is_none()) {
            return 0;
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_all" => {
                diesel::update(self)
                    .set(schema::users::see_all.eq(value))
                    .execute(&_connection)
                    .expect("E.");
                let some_item_user = item_users
                    .filter(schema::item_users::user_id.eq(self.user_id))
                    .first::<ItemUser>(&_connection);
                if some_item_user.is_ok() {
                    let i_e = some_item_user.expect("E.");
                    diesel::update(&i_e)
                        .set(schema::item_users::see_all.eq(value))
                        .execute(&_connection)
                        .expect("E.")
                }
                else {
                    0
                }
            },
            "see_el" => diesel::update(self)
                .set(schema::users::see_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_comment" => diesel::update(self)
                .set(schema::users::see_comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "create_el" => diesel::update(self)
                .set(schema::users::create_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            "create_comment" => diesel::update(self)
                .set(schema::users::create_comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "copy_el" => diesel::update(self)
                .set(schema::users::copy_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
            };

        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::user_visible_perms::dsl::user_visible_perms;
            match value { 
                0 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(10))
                    )
                    .execute(&_connection)
                    .expect("E"),
                1 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(11))
                    )
                    .execute(&_connection)
                    .expect("E"),
                2 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(12))
                    )
                    .execute(&_connection)
                    .expect("E"),
                3 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(13))
                    )
                    .execute(&_connection)
                    .expect("E"),
                4 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(14))
                    )
                    .execute(&_connection)
                    .expect("E"),
                5 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(15))
                    )
                    .execute(&_connection)
                    .expect("E"),
                10 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(0))
                    )
                    .execute(&_connection)
                    .expect("E"),
                11 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(1))
                    )
                    .execute(&_connection)
                    .expect("E"),
                12 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(2))
                    )
                    .execute(&_connection)
                    .expect("E"),
                13 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(3))
                    )
                    .execute(&_connection)
                    .expect("E"),
                14 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(4))
                    )
                    .execute(&_connection)
                    .expect("E"),
                15 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(5))
                    )
                    .execute(&_connection)
                    .expect("E"),
                100 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(110))
                    )
                    .execute(&_connection)
                    .expect("E"),
                101 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(111))
                    )
                    .execute(&_connection)
                    .expect("E"),
                102 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(112))
                    )
                    .execute(&_connection)
                    .expect("E"),
                103 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(113))
                    )
                    .execute(&_connection)
                    .expect("E"),
                104 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(114))
                    )
                    .execute(&_connection)
                    .expect("E"),
                105 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(115))
                    )
                    .execute(&_connection)
                    .expect("E"),
                110 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(100))
                    )
                    .execute(&_connection)
                    .expect("E"),
                111 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(101))
                    )
                    .execute(&_connection)
                    .expect("E"),
                112 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(102))
                    )
                    .execute(&_connection)
                    .expect("E"),
                113 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(103))
                    )
                    .execute(&_connection)
                    .expect("E"),
                114 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(104))
                    )
                    .execute(&_connection)
                    .expect("E"),
                115 => diesel::delete (
                    user_visible_perms
                        .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                        .filter(schema::user_visible_perms::types.eq(105))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if items_ids.is_some() && is_ie_mode {
            for item_id in items_ids.unwrap().iter() {
                let _new_perm = NewUserVisiblePerm {
                    user_id: self.user_id,
                    item_id: *item_id,
                    types:   value,
                };
                diesel::insert_into(schema::user_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }

    pub fn get_post_lists (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::user_id.eq(self.user_id))
            .filter(schema::post_lists::community_id.is_null())
            .filter(schema::post_lists::types.lt(31))
            .order(schema::post_lists::created.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn search_post_lists (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardPostListJson> {
        use crate::schema::post_lists::dsl::post_lists;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let mut lists_json = Vec::new();
        let lists =  post_lists
            .filter(schema::post_lists::user_id.eq(self.user_id))
            .filter(schema::post_lists::community_id.is_null())
            .filter(schema::post_lists::types.lt(31))
            .filter(schema::post_lists::name.ilike(&q))
            .or_filter(schema::post_lists::description.ilike(&q))
            .order(schema::post_lists::created.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<PostList>(&_connection)
            .expect("E.");

        for i in lists.iter() {
            let owner = i.get_owner_meta().expect("E");
            lists_json.push (
                CardPostListJson {
                    name:        i.name.clone(),
                    owner_name:  owner.name.clone(),
                    owner_link:  owner.link.clone(),
                    owner_image: owner.image.clone(),
                    image:       i.image.clone(),
                    types:       i.get_code(),
                    count:       i.count,
                }
            );
        } 
        return lists_json;
    }
    pub fn search_posts (
        &self,
        q:       &String,
        user_id: i32,
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> Vec<CardPostJson> {
        let mut posts_json = Vec::new();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        if (user_id > 0 && self.is_user_see_el(user_id))
            ||
            (user_id == 0 && self.is_anon_user_see_el())
            {
            use crate::schema::posts::dsl::posts;

            let _connection = establish_connection();
            let mut _count = 0;

            for list in self.get_post_lists(Some(20), Some(0)).iter() {
                if (user_id > 0 && list.is_user_see_el(user_id))
                    ||
                    (user_id == 0 && list.is_anon_user_see_el())
                    {
                    let __limit = _limit - _count;
                    let reactions_list = list.get_reactions_list();
                    let items = posts
                        .filter(schema::posts::post_list_id.eq(list.id))
                        .filter(schema::posts::content.ilike(&q))
                        .filter(schema::posts::types.lt(11))
                        .limit(__limit)
                        .offset(_offset)
                        .order(schema::posts::created.desc())
                        .load::<Post>(&_connection)
                        .expect("E.");

                    for i in items.iter() {
                        _count += 1;
                        if _count < _limit {
                            posts_json.push ( i.get_post_json(user_id, reactions_list.clone()) )
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            return posts_json;
        }
        else {
            return Vec::new();
        }
    }

    pub fn search_comments (
        &self,
        q:       &String,
        user_id: i32,
        limit:   Option<i64>,
        offset:  Option<i64>,
    ) -> SearchAllComments {
        use crate::schema::post_comments::dsl::post_comments;
        use crate::models::PostComment;

        let _connection = establish_connection();
        let mut _count = 0;
        let mut _step = 0;
        let (_limit, mut _offset) = get_limit_offset(limit, offset, 20);

        let mut creator_include: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых можно смотреть посты
        let mut community_include: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых можно смотреть посты
        let mut list_include: Vec<i32> = Vec::new();
        let mut creator_exclude: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых нельзя смотреть посты
        let mut community_exclude: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых можно нельзя посты
        let mut list_exclude: Vec<i32> = Vec::new();      // запишем ids списков, у которых можно нельзя посты
        let mut list_json = Vec::new();

        while _count < _limit {
            _step += _limit;

            let items = post_comments
                .filter(schema::post_comments::user_id.eq(self.id))
                .filter(schema::post_comments::content.ilike(&q))
                .filter(schema::post_comments::types.lt(10))
                .limit(_step)
                .offset(_offset)
                .order(schema::post_comments::created.desc())
                .load::<PostComment>(&_connection)
                .expect("E.");

            for i in items.iter() {
                if _count == _limit {
                    break;
                }
                let list = i.get_list();
                // проверяем, запрещено ли запрашивающему смотреть
                // посты пользователя или сообщества или списка
                if creator_exclude.iter().any(|&a| a==list.user_id)
                    ||
                    (list.community_id.is_some() && community_exclude.iter().any(|&a| a==list.community_id.unwrap()))
                    ||
                    list_exclude.iter().any(|&a| a==list.id)
                {
                    continue;
                }
                else if list_include.iter().any(|&a| a==list.id) {
                    _count += 1;
                    list_json.push ( i.get_comment_json(user_id, list.get_reactions_list()) );
                    continue;
                }

                if list.community_id.is_some() {
                    // если пост сообщества
                    if community_include.iter().any(|&a| a==list.community_id.unwrap()) {
                        // если id сообщества в разрешенных community_include,
                        if (user_id > 0 && list.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && list.is_anon_user_see_el())
                        {
                            list_json.push ( i.get_comment_json(user_id, list.get_reactions_list()) );
                            _count += 1;
                            list_include.push(list.id);
                            continue;
                        }
                        else {
                            list_exclude.push(list.id);
                            continue;
                        }
                    }
                    else {
                        // если id сообщества нет в разрешенных community_include,
                        let community = list.get_community().expect("E.");
                        if (user_id > 0 && community.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && community.is_anon_user_see_el())
                        {
                            community_include.push(community.id);
                            if (user_id > 0 && list.is_user_see_el(user_id))
                                ||
                                (user_id == 0 && list.is_anon_user_see_el())
                            {
                                list_json.push ( i.get_comment_json(user_id, list.get_reactions_list()) );
                                _count += 1;
                                list_include.push(list.id);
                                continue;
                            }
                            else {
                                list_exclude.push(list.id);
                                continue;
                            }
                        }
                        else {
                            community_exclude.push(list.community_id.unwrap());
                            continue;
                        }
                    }
                }
                // если пост пользователя
                if creator_include.iter().any(|&a| a==list.user_id) {
                    // если id пользователя в разрешенных creator_include,
                    if (user_id > 0 && list.is_user_see_el(user_id))
                        ||
                        (user_id == 0 && list.is_anon_user_see_el())
                    {
                        list_json.push ( i.get_comment_json(user_id, list.get_reactions_list()) );
                        _count += 1;
                        list_include.push(list.id);
                        continue;
                    }
                    else {
                        list_exclude.push(list.id);
                        continue;
                    }
                }
                else {
                    // если id пользователя нет в разрешенных creator_include,
                    let creator = list.get_creator().expect("E.");
                    if (user_id > 0 && creator.is_user_see_el(user_id))
                        ||
                        (user_id == 0 && creator.is_anon_user_see_el())
                    {
                        creator_include.push(creator.id);
                        if (user_id > 0 && list.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && list.is_anon_user_see_el())
                        {
                            list_json.push ( i.get_comment_json(user_id, list.get_reactions_list()) );
                            _count += 1;
                            list_include.push(list.id);
                            continue;
                        }
                        else {
                            list_exclude.push(list.id);
                            continue;
                        }
                    }
                    else {
                        creator_exclude.push(list.user_id);
                        continue;
                    }
                }
            }
            _offset += _step;
        }
        return SearchAllComments {
            comments: list_json,
            offset:   _offset,
        };
    }


    pub fn get_fixed_posts_ids(&self) -> Vec<i32> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::user_id.eq(self.user_id))
            .filter(schema::posts::community_id.is_null())
            .filter(schema::posts::types.eq(10))
            .order(schema::posts::created.desc())
            .select(schema::posts::id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_fixed_posts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::id.eq_any(self.get_fixed_posts_ids()))
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn count_fix_items(&self) -> usize {
        return self.get_fixed_posts_ids().len();
    }
    pub fn is_can_fixed_post(&self) -> bool {
        return self.count_fix_items() < 10;
    }

    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }

    pub fn create_user (
        user_id:    i32,
        first_name: String,
        last_name:  String,
        types:      i16,
        is_man:     bool,
        password:   String,
        link:       String,
    ) -> i16 {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if users
            .filter(schema::users::user_id.eq(user_id))
            .select(schema::users::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }
        let new_form = NewUser {
            user_id:        user_id,
            first_name:     first_name.clone(),
            last_name:      last_name.clone(),
            types:          types,
            is_man:         is_man,
            password:       password.clone(),
            link:           link.clone(),
            s_avatar:       None,
            last_activity:  chrono::Local::now().naive_utc(),
            see_all:        1,
            see_el:         1,
            see_comment:    1,
            create_el:      13,
            create_comment: 12,
            copy_el:        1,
            lists:          0,
            posts:          0,
            comments:       0,
        };
        let _user = diesel::insert_into(schema::users::table)
            .values(&new_form)
            .get_result::<User>(&_connection)
            .expect("Error.");
        _user.create_main_post_list();
        return 1;
    }
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_str_id(&self) -> String {
        return self.user_id.to_string();
    }
    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.get_full_name() + &"</a>".to_string();
    }
    pub fn get_code(&self) -> String {
        return "use".to_string() + &self.get_str_id();
    }

    pub fn is_friend_perm_exists (
        &self,
        user_id: i32,
        types:   i16, 
    ) -> bool {
        // проверяем, если ли пользователь в вкл/искл списках пользователя 
        // и дружит ли он с self
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            friends::dsl::friends,
        };

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(types))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok() &&
        friends 
            .filter(schema::friends::target_id.eq(self.user_id))
            .filter(schema::friends::user_id.eq(user_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_follow_perm_exists (
        &self,
        user_id: i32,
        types:   i16, 
    ) -> bool {
        // проверяем, если ли пользователь в вкл/искл списках пользователя 
        // и подписан ли он на self
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            follows::dsl::follows,
        };

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::types.eq(types))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok() &&
        follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_ie_friends_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            item_users::dsl::item_users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

        return item_users
            .filter(schema::item_users::id.eq_any(items_ids))
            .filter(schema::item_users::types.lt(31))
            .select((
                schema::item_users::user_id,
                schema::item_users::first_name,
                schema::item_users::last_name,
                schema::item_users::link,
                schema::item_users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
    }

    pub fn get_ie_follows_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            user_visible_perms::dsl::user_visible_perms,
            item_users::dsl::item_users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::target_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

        return item_users
            .filter(schema::item_users::id.eq_any(items_ids))
            .filter(schema::item_users::types.lt(31))
            .select((
                schema::item_users::user_id,
                schema::item_users::first_name,
                schema::item_users::last_name,
                schema::item_users::link,
                schema::item_users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
    }

    pub fn get_limit_see_el_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_el_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(1, limit, offset); 
    } 
    pub fn get_limit_see_comment_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_comment_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(2, limit, offset); 
    }
    pub fn get_limit_create_el_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(13, limit, offset); 
    }
    pub fn get_limit_create_el_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(3, limit, offset); 
    }
    pub fn get_limit_create_comment_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(14, limit, offset); 
    }
    pub fn get_limit_create_comment_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(4, limit, offset); 
    }
    pub fn get_limit_copy_el_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(15, limit, offset); 
    }
    pub fn get_limit_copy_el_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(5, limit, offset); 
    }
    pub fn get_limit_see_all_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(10, limit, offset); 
    }
    pub fn get_limit_see_all_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(0, limit, offset); 
    }

    pub fn get_limit_see_el_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_el_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(1, limit, offset); 
    } 
    pub fn get_limit_see_comment_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_comment_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(2, limit, offset); 
    }
    pub fn get_limit_create_el_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(13, limit, offset); 
    }
    pub fn get_limit_create_el_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(3, limit, offset); 
    }
    pub fn get_limit_create_comment_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(14, limit, offset); 
    }
    pub fn get_limit_create_comment_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(4, limit, offset); 
    }
    pub fn get_limit_copy_el_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(15, limit, offset); 
    }
    pub fn get_limit_copy_el_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(5, limit, offset); 
    }
    pub fn get_limit_see_all_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(10, limit, offset); 
    }
    pub fn get_limit_see_all_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(0, limit, offset); 
    }

    pub fn get_limit_comment_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(61, limit, offset); 
    }
    pub fn get_limit_comment_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(51, limit, offset); 
    } 
    pub fn get_limit_comment_reply_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(62, limit, offset); 
    }
    pub fn get_limit_comment_reply_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(52, limit, offset); 
    }
    pub fn get_limit_mention_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(63, limit, offset); 
    }
    pub fn get_limit_mention_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(53, limit, offset); 
    }
    pub fn get_limit_comment_mention_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(64, limit, offset); 
    }
    pub fn get_limit_comment_mention_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(54, limit, offset); 
    }
    pub fn get_limit_repost_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(65, limit, offset); 
    }
    pub fn get_limit_repost_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(55, limit, offset); 
    }
    pub fn get_limit_reactions_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(66, limit, offset); 
    }
    pub fn get_limit_reactions_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(56, limit, offset); 
    }

    pub fn get_limit_comment_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(61, limit, offset); 
    }
    pub fn get_limit_comment_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(51, limit, offset); 
    } 
    pub fn get_limit_comment_reply_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(62, limit, offset); 
    }
    pub fn get_limit_comment_reply_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(52, limit, offset); 
    }
    pub fn get_limit_mention_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(63, limit, offset); 
    }
    pub fn get_limit_mention_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(53, limit, offset); 
    }
    pub fn get_limit_comment_mention_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(64, limit, offset); 
    }
    pub fn get_limit_comment_mention_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(54, limit, offset); 
    }
    pub fn get_limit_repost_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(65, limit, offset); 
    }
    pub fn get_limit_repost_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(55, limit, offset); 
    }
    pub fn get_limit_reactions_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(66, limit, offset); 
    }
    pub fn get_limit_reactions_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(56, limit, offset); 
    }


    pub fn is_user_see_all(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_all {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 10),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 0),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 10),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 0),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 10),
            10 => self.is_friend_perm_exists(user_id, 0),
            11 => !self.is_follow_perm_exists(user_id, 10),
            12 => self.is_follow_perm_exists(user_id, 0),
            _ => false,
        };
    }
    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_el {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 11),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 1),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 11),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 1),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 11),
            10 => self.is_friend_perm_exists(user_id, 1),
            11 => !self.is_follow_perm_exists(user_id, 11),
            12 => self.is_follow_perm_exists(user_id, 1),
            _ => false,
        };
    }
    pub fn is_user_see_comment(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_comment {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 12),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 2),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 12),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 2),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 12),
            10 => self.is_friend_perm_exists(user_id, 2),
            11 => !self.is_follow_perm_exists(user_id, 12),
            12 => self.is_follow_perm_exists(user_id, 2),
            _ => false,
        };
    }

    pub fn is_user_create_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.create_el {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 13),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 3),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 13),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 3),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 13),
            10 => self.is_friend_perm_exists(user_id, 3),
            11 => !self.is_follow_perm_exists(user_id, 13),
            12 => self.is_follow_perm_exists(user_id, 3),
            _ => false,
        };
    }
    pub fn is_user_create_comment(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.create_comment {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 14),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 4),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 14),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 4),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 14),
            10 => self.is_friend_perm_exists(user_id, 4),
            11 => !self.is_follow_perm_exists(user_id, 14),
            12 => self.is_follow_perm_exists(user_id, 4),
            _ => false,
        };
    }
    pub fn is_user_copy_el(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.copy_el {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 15),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 5),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 15),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 5),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 15),
            10 => self.is_friend_perm_exists(user_id, 5),
            11 => !self.is_follow_perm_exists(user_id, 15),
            12 => self.is_follow_perm_exists(user_id, 5),
            13 => false,
            _ => false,
        };
    }

    pub fn is_anon_user_see_all(&self) -> bool {
        return self.see_all == 1;
    }
    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1;
    }
    pub fn is_anon_user_see_comment(&self) -> bool {
        return self.see_comment == 1;
    }

    pub fn get_friends_ids(&self) -> Vec<i32> {
        // в местные таблицы друзей и подписчиков мы записываем
        // id пользователей с сервиса пользователей, чтобы было
        // корректнее их сравнивать.
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        let _friends = friends
            .filter(schema::friends::user_id.eq(self.user_id))
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_follows_ids(&self) -> Vec<i32> {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        let _follows = follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .select(schema::follows::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return _follows;
    }

    pub fn get_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            friends::dsl::friends,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);

        let _connection = establish_connection();
        let friend_ids = friends
            .filter(schema::friends::user_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::friends::target_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _friends = users
            .filter(schema::users::id.eq_any(friend_ids))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _friends;
    }
    pub fn get_followers(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let followers = follows
            .filter(schema::follows::target_id.eq(self.id))
            .order(schema::follows::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .select(schema::follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let _users = users
            .filter(schema::users::id.eq_any(followers))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn delete_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;
        /*
        любые изменения пользователей и сообществ копий должны проверять, 
        есть ли этот пользователь/сообщество в таблицах item_users/item_communitys,
        ведь пользователь может быть и там тоже, в качестве владельца, например, 
        прикрепляемых элементов.
        */
        let _connection = establish_connection();
        let _case = match self.types { 
            1 => 31,
            2 => 32,
            3 => 33,
            4 => 34,
            5 => 35,
            6 => 36,
            _ => 31,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }
        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _case = match self.types {
            31 => 1,
            32 => 2,
            33 => 2,
            34 => 4,
            35 => 5,
            36 => 6,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn close_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _case = match self.types {
            1 => 41,
            2 => 42,
            3 => 43,
            4 => 44,
            5 => 45,
            6 => 46,
            _ => 41,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _case = match self.types {
            41 => 1,
            42 => 2,
            43 => 2,
            44 => 4,
            45 => 5,
            46 => 6,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn suspend_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _case = match self.types {
            1 => 51,
            2 => 52,
            3 => 53,
            4 => 54,
            5 => 55,
            6 => 56,
            _ => 51,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);

        let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unsuspend_item(&self) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let _connection = establish_connection();
        let _case = match self.types {
            51 => 1,
            52 => 2,
            53 => 2,
            54 => 4,
            55 => 5,
            56 => 6,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::users::types.eq(_case))
            .execute(&_connection);
        
            let some_item_user = item_users
            .filter(schema::item_users::user_id.eq(self.user_id))
            .first::<ItemUser>(&_connection);
        if some_item_user.is_ok() {
            let i_e = some_item_user.expect("E.");
            let _i = diesel::update(&i_e)
                .set(schema::item_users::types.eq(_case))
                .execute(&_connection);
        }

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn change_staff(&self, types: i16) -> i16 {
        let _connection = establish_connection();
        let o = diesel::update(self)
            .set(schema::users::types.eq(types))
            .execute(&_connection);

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn add_new_community_subscriber (&self, community_id: i32) -> () {
        use crate::models::NewNewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.user_id))
            .filter(schema::news_user_communities::community_id.eq(community_id))
            .select(schema::news_user_communities::id)
            .first::<i32>(&_connection)
            .is_ok() {
                let _new = NewNewsUserCommunitie {
                    owner:        self.user_id,
                    list_id:      None,
                    user_id:      None,
                    community_id: Some(community_id),
                    mute:         false,
                    sleep:        None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .execute(&_connection)
                .expect("Error.");
        }
    }

    pub fn add_notification_community_subscriber (&self, community_id: i32) -> () {
        use crate::models::NewNotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.user_id))
            .filter(schema::notify_user_communities::community_id.eq(community_id))
            .select(schema::notify_user_communities::id)
            .first::<i32>(&_connection)
            .is_ok() {
                let _new = NewNotifyUserCommunitie {
                    owner: self.user_id,
                    list_id: None,
                    user_id: None,
                    community_id: Some(community_id),
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .execute(&_connection)
                    .expect("Error.");
        }
    }
    pub fn add_new_user_subscriber(&self, user_id: i32) -> () {
        use crate::models::NewNewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        if news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.user_id))
            .filter(schema::news_user_communities::user_id.eq(user_id))
            .select(schema::news_user_communities::id)
            .first::<i32>(&_connection)
            .is_ok() {
                let _new = NewNewsUserCommunitie {
                    owner: self.user_id,
                    list_id: None,
                    user_id: Some(user_id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
            diesel::insert_into(schema::news_user_communities::table)
                .values(&_new)
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn add_new_subscriber_in_list(&self, new_id: i32, list_id: i32) -> bool {
        use crate::models::{NewsUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .first::<NewsUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .first::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _new.owner == self.user_id && _list.owner == self.user_id {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(list_id))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber(&self, new_id: i32) -> bool {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .first::<NewsUserCommunitie>(&_connection)
            .expect("E");
        if _new.owner == self.user_id {
            diesel::delete(
                news_user_communities
                    .filter(schema::news_user_communities::id.eq(new_id))
                )
                .execute(&_connection)
                .expect("E");
            return true;
        }
        return false;
    }
    pub fn delete_new_subscriber_from_list(&self, new_id: i32) -> bool {
        use crate::models::NewsUserCommunitie;
        use crate::schema::news_user_communities::dsl::news_user_communities;

        let _connection = establish_connection();
        let _new = news_user_communities
            .filter(schema::news_user_communities::id.eq(new_id))
            .first::<NewsUserCommunitie>(&_connection)
            .expect("E");
        let null_value: Option<i32> = None;

        if _new.owner == self.user_id {
            diesel::update(news_user_communities.filter(schema::news_user_communities::id.eq(new_id)))
                .set(schema::news_user_communities::list_id.eq(null_value))
                .execute(&_connection)
                .expect("Error.");
        }
        return true;
    }

    pub fn add_notification_user_subscriber(&self, user: &User) -> () {
        use crate::models::NewNotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        if notify_user_communities
            .filter(schema::notify_user_communities::owner.eq(self.user_id))
            .filter(schema::notify_user_communities::user_id.eq(user.id))
            .select(schema::notify_user_communities::id)
            .first::<i32>(&_connection)
            .is_ok() {
                let _new = NewNotifyUserCommunitie {
                    owner: self.user_id,
                    list_id: None,
                    user_id: Some(user.id),
                    community_id: None,
                    mute: false,
                    sleep: None,
                };
                diesel::insert_into(schema::notify_user_communities::table)
                    .values(&_new)
                    .execute(&_connection)
                    .expect("Error.");
        }
    }
    pub fn add_notification_subscriber_in_list(&self, notify_id: i32, list_id: i32) -> () {
        use crate::models::{NotifyUserCommunitie, ListUserCommunitiesKey};
        use crate::schema::notify_user_communities::dsl::notify_user_communities;
        use crate::schema::list_user_communities_keys::dsl::list_user_communities_keys;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::id.eq(notify_id))
            .first::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        let _list = list_user_communities_keys
            .filter(schema::list_user_communities_keys::id.eq(list_id))
            .first::<ListUserCommunitiesKey>(&_connection)
            .expect("E");

        if _notify.owner == self.user_id && _list.owner == self.user_id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(_list.id))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn delete_notification_subscriber(&self, notify_id: i32) -> bool {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::id.eq(notify_id))
            .first::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        if _notify.owner == self.user_id {
            let del = diesel::delete (
                notify_user_communities
                    .filter(schema::notify_user_communities::id.eq(notify_id))
                )
                .execute(&_connection);
            if del.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }
        return false;
    }
    pub fn delete_notification_subscriber_from_list(&self, notify_id: i32) -> bool {
        use crate::models::NotifyUserCommunitie;
        use crate::schema::notify_user_communities::dsl::notify_user_communities;

        let _connection = establish_connection();
        let _notify = notify_user_communities
            .filter(schema::notify_user_communities::id.eq(notify_id))
            .first::<NotifyUserCommunitie>(&_connection)
            .expect("E");
        let null_value: Option<i32> = None;
        if _notify.owner == self.user_id {
            diesel::update(notify_user_communities.filter(schema::notify_user_communities::id.eq(notify_id)))
                .set(schema::notify_user_communities::list_id.eq(null_value))
                .execute(&_connection)
                .expect("Error.");
        }
        return true;
    }

    pub fn plus_lists(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::lists.eq(self.lists + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_lists(&self, count: i32) -> bool {
        if self.lists > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::lists.eq(self.lists - count))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_posts(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::posts.eq(self.posts + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_posts(&self, count: i32) -> bool {
        if self.posts > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::posts.eq(self.posts - count))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::comments.eq(self.comments + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        if self.comments > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::users::comments.eq(self.comments - count))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }

    pub fn count_lists(&self) -> i32 {
        return self.lists;
    }
    pub fn count_lists_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_lists(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
        );
    }
    pub fn count_lists_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_lists(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
        );
    }

    pub fn count_posts(&self) -> i32 {
        return self.posts;
    }
    pub fn count_posts_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }
    pub fn count_posts_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn count_comments(&self) -> i32 {
        return self.comments;
    }
    pub fn count_comments_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_comments(),
            " комментарий".to_string(),
            " комментария".to_string(),
            " комментариев".to_string(),
        );
    }
    pub fn count_comments_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_comments(),
            " комментарий".to_string(),
            " комментария".to_string(),
            " комментариев".to_string(),
        );
    }

    pub fn get_ids_for_featured_news(&self) -> (Vec<i32>, Vec<i32>) {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;
        use crate::models::FeaturedUserCommunitie;

        let _connection = establish_connection();
        let news = featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.user_id))
            .filter(schema::featured_user_communities::mute.eq(false))
            .filter(schema::featured_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<FeaturedUserCommunitie>(&_connection)
            .expect("E.");
        let mut users_stack = Vec::new();
        let mut communities_stack = Vec::new();
        for i in news.iter() {
            if i.community_id.is_some() {
                communities_stack.push(i.community_id.unwrap());
            }
            else {
                users_stack.push(i.user_id.unwrap());
            }
        }
        return (users_stack, communities_stack);
    }
    pub fn get_ids_for_main_news(&self) -> (Vec<i32>, Vec<i32>) {
        use crate::schema::news_user_communities::dsl::news_user_communities;
        use crate::models::NewsUserCommunitie;

        let _connection = establish_connection();
        let news = news_user_communities
            .filter(schema::news_user_communities::owner.eq(self.user_id))
            .filter(schema::news_user_communities::mute.eq(false))
            .filter(schema::news_user_communities::sleep.lt(chrono::Local::now().naive_utc()))
            .load::<NewsUserCommunitie>(&_connection)
            .expect("E.");
        let mut users_stack = Vec::new();
        let mut communities_stack = Vec::new();
        for i in news.iter() {
            if i.community_id.is_some() {
                communities_stack.push(i.community_id.unwrap());
            }
            else {
                users_stack.push(i.user_id.unwrap());
            }
        }
        return (users_stack, communities_stack);
    }

    pub fn get_verb_gender(&self, str: &str) -> String {
        if self.is_man == false {
            return "W".to_string() + &str;
        }
        else {
            return str.to_string();
        }
    }
    pub fn is_women(&self) -> bool {
        return self.is_man == false;
    }
    pub fn is_man(&self) -> bool {
        return self.is_man;
    }
    
    pub fn is_child(&self) -> bool {
        return self.types > 3 && self.types < 7;
    }
    pub fn is_suspended(&self) -> bool {
        return self.types < 60 && self.types > 50;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types < 70 && self.types > 50; 
    }
    pub fn is_deleted(&self) -> bool {
        return self.types < 40 && self.types > 30;
    }
    pub fn is_closed(&self) -> bool {
        return self.types < 50 && self.types > 40;
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types == 2 && self.types == 5;
    }
    pub fn is_identified(&self) -> bool {
        return self.types == 3 && self.types == 6;
    }

    pub fn is_online(&self) -> bool {
        use chrono::Duration;
        return (self.last_activity + Duration::seconds(300)) > chrono::Local::now().naive_utc();
    }
    pub fn get_online_status(&self) -> String {
        if self.is_online() {
            return "Онлайн".to_string();
        }
        else {
            if self.is_women() {
                return "Была ".to_string() + &self.last_activity.to_string();
            } else {
                return "Был ".to_string() + &self.last_activity.to_string();
            }
        }
    }
    pub fn is_user_in_block(&self, user_id: i32) -> bool {
        // пользователь с user_id заблокирован у self
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
         // self заблокирован у user_id
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(user_id))
            .filter(schema::user_visible_perms::target_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        // self дружит с user_id
        use crate::schema::friends::dsl::friends; 

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(self.user_id))
            .filter(schema::friends::target_id.eq(user_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_self_connected_with_user_with_id(&self, user_id: i32) -> bool {
        // user_id дружит с self
        use crate::schema::friends::dsl::friends; 

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::target_id.eq(self.user_id))
            .filter(schema::friends::user_id.eq(user_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        // self подписан на user_id
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        // user_id подписан на self
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_self_followers_user_with_id(&self, user_id: i32) -> bool {
        // self подписан на user_id
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(user_id))
            .filter(schema::follows::user_id.eq(self.user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_or_create_featured_objects (
        &self,
        user_id: i32,
        conn:    &PgConnection
    ) -> () {
        use crate::models::NewFeaturedUserCommunitie;
        use crate::schema::{
            featured_user_communities::dsl::featured_user_communities,
            communities_memberships::dsl::communities_memberships,
            friends::dsl::friends,
        };

        let friends_ids = friends
            .filter(schema::friends::user_id.eq(user_id))
            .limit(6)
            .select(schema::friends::target_id)
            .load::<i32>(conn)
            .expect("E.");
        let communities_ids = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .limit(6)
            .select(schema::communities_memberships::community_id)
            .load::<i32>(conn)
            .expect("E.");

        if !friends_ids.is_empty() {
            for friend_id in friends_ids.iter() {
                if self.is_connected_with_user_with_id(*friend_id) && !featured_user_communities
                    .filter(schema::featured_user_communities::owner.eq(self.user_id))
                    .filter(schema::featured_user_communities::user_id.eq(friend_id))
                    .select(schema::featured_user_communities::id)
                    .first::<i32>(conn).is_err() {

                    let new_featured = NewFeaturedUserCommunitie {
                        owner: self.user_id,
                        list_id: None,
                        user_id: Some(*friend_id),
                        community_id: None,
                        mute: false,
                        sleep: None,
                    };
                    diesel::insert_into(schema::featured_user_communities::table)
                        .values(&new_featured)
                        .execute(conn)
                        .expect("Error.");
                    }
            }
        }
        if !communities_ids.is_empty() {
            for community_id in communities_ids.iter() {
                if self.is_member_of_community(*community_id) && !featured_user_communities
                    .filter(schema::featured_user_communities::owner.eq(self.user_id))
                    .filter(schema::featured_user_communities::community_id.eq(community_id))
                    .select(schema::featured_user_communities::id)
                    .first::<i32>(conn).is_err() {

                    let new_featured = NewFeaturedUserCommunitie {
                        owner: self.user_id,
                        list_id: None,
                        user_id: None,
                        community_id: Some(*community_id),
                        mute: false,
                        sleep: None,
                    };
                    diesel::insert_into(schema::featured_user_communities::table)
                        .values(&new_featured)
                        .execute(conn)
                        .expect("Error.");
                }
            }
        }
    }

    pub fn delete_user_featured_object (
        &self,
        user_id: i32,
    )  -> bool {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;

        let _connection = establish_connection();
        let del = diesel::delete (
            featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.user_id))
            .filter(schema::featured_user_communities::user_id.eq(user_id))
        )
        .execute(&_connection);
        if del.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn delete_community_featured_object (
        &self,
        community_id: i32,
    )  -> bool {
        use crate::schema::featured_user_communities::dsl::featured_user_communities;

        let _connection = establish_connection();
        let del = diesel::delete (
            featured_user_communities
            .filter(schema::featured_user_communities::owner.eq(self.user_id))
            .filter(schema::featured_user_communities::community_id.eq(community_id))
        )
        .execute(&_connection);
        if del.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn is_member_of_community(&self, community_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.user_id))
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn follow_user(&self, user_id: i32) -> i16 {
        use crate::schema::{
            users::dsl::users,
            follows::dsl::follows,
        };
        
        let _connection = establish_connection();
        if self.user_id == user_id || self.is_self_user_in_block(user_id) || self.is_followers_user_with_id(user_id) || self.is_following_user_with_id(user_id) {
            return 0;
        }
        else if follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }

        let _new_follow = NewFollow {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);
        if new_follow.is_ok() {
            let mut is_user_see_all = false;
            let target_user = users
                .filter(schema::users::user_id.eq(user_id))
                .first::<User>(&_connection);
            if target_user.is_ok() {
                let _target_user = target_user.expect("E.");
                is_user_see_all = _target_user.is_user_see_all(user_id);
            }
            if is_user_see_all {
                self.add_new_user_subscriber(user_id);
                self.get_or_create_featured_objects(user_id, &_connection);
            }
            return 1;
        }
        else {
            return 0;
        }

    }
    pub fn unfollow_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_following_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::{
            follows::dsl::follows,
            users::dsl::users,
        };

        let _connection = establish_connection();
        if follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .first::<Follow>(&_connection).is_ok() {
                let del = diesel::delete (
                    follows
                        .filter(schema::follows::target_id.eq(user_id))
                        .filter(schema::follows::user_id.eq(self.user_id))
                )
                .execute(&_connection);
            if del.is_ok() {
                let target_user = users
                    .filter(schema::users::user_id.eq(user_id))
                    .first::<User>(&_connection);
                if target_user.is_ok() {
                    let _target_user = target_user.expect("E.");
                    if _target_user.is_user_see_all(user_id) {
                        self.delete_new_subscriber(user_id);
                    }
                }
                
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }

    pub fn frend_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_followers_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::{
            follows::dsl::follows,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let _new_friend = NewFriend {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let new_friend = diesel::insert_into(schema::friends::table)
            .values(&_new_friend)
            .execute(&_connection);

        let del = diesel::delete (
            follows
                .filter(schema::follows::user_id.eq(user_id))
                .or_filter(schema::follows::target_id.eq(self.user_id))
                .filter(schema::follows::target_id.eq(user_id))
                .or_filter(schema::follows::user_id.eq(self.user_id))
            )
            .execute(&_connection);
        if del.is_ok() && new_friend.is_ok() {
            let mut is_user_see_all = false;
            let target_user = users
                .filter(schema::users::user_id.eq(user_id))
                .first::<User>(&_connection);
            if target_user.is_ok() {
                let _target_user = target_user.expect("E.");
                is_user_see_all = _target_user.is_user_see_all(user_id);
            }

            self.delete_user_featured_object(user_id);
            if !is_user_see_all {
                self.add_new_user_subscriber(user_id);
                self.get_or_create_featured_objects(user_id, &_connection);
            }
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unfrend_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_connected_with_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::{
            friends::dsl::friends,
            users::dsl::users,
        };

        let _connection = establish_connection();

        let del = diesel::delete (
            friends
                .filter(schema::friends::user_id.eq(self.user_id))
                .filter(schema::friends::target_id.eq(user_id))
            )
            .execute(&_connection);

        let _new_follow = NewFollow {
            user_id:   user_id,
            target_id: self.user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);
        if del.is_ok() && new_follow.is_ok() {
            let mut is_user_see_all = false;
            let target_user = users
                .filter(schema::users::user_id.eq(user_id))
                .first::<User>(&_connection);
            if target_user.is_ok() {
                let _target_user = target_user.expect("E.");
                is_user_see_all = _target_user.is_user_see_all(user_id);
            }
            if !is_user_see_all {
                self.delete_new_subscriber(user_id);
            }
            self.get_or_create_featured_objects(user_id, &_connection);
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn block_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || self.is_user_in_block(user_id) {
            return 0;
        }

        let _connection = establish_connection();

        if self.is_connected_with_user_with_id(user_id) {
            use crate::schema::friends::dsl::friends;
            diesel::delete (
                friends
                    .filter(schema::friends::user_id.eq(self.user_id))
                    .filter(schema::friends::target_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else if self.is_followers_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete (
                follows
                    .filter(schema::follows::target_id.eq(self.user_id))
                    .filter(schema::follows::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else if self.is_following_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            diesel::delete(
                follows
                    .filter(schema::follows::user_id.eq(self.user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }

        // удалим id блокируемого из таблицы включений / исключений
        diesel::delete (
            schema::user_visible_perms::table
                .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                .filter(schema::user_visible_perms::target_id.eq(user_id))
            )
            .execute(&_connection)
            .expect("E");

        let _user_block = NewUserVisiblePerm {
            user_id:   self.user_id,
            target_id: user_id,
            types:     20,
        };
        diesel::insert_into(schema::user_visible_perms::table)
            .values(&_user_block)
            .execute(&_connection)
            .expect("Error.");

        self.delete_new_subscriber(user_id);
        self.delete_user_featured_object(user_id);
        self.delete_notification_subscriber(user_id);

        return 1;
    }
    pub fn unblock_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_user_in_block(user_id) {
            return 0;
        }
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        let del = diesel::delete (
            user_visible_perms
                .filter(schema::user_visible_perms::user_id.eq(self.user_id))
                .filter(schema::user_visible_perms::target_id.eq(user_id))
            )
            .execute(&_connection);
        if del.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn is_user_in_ban(&self, community_id: i32) -> bool {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        return community_visible_perms
            .filter(schema::community_visible_perms::target_id.eq(self.user_id))
            .filter(schema::community_visible_perms::community_id.eq(community_id))
            .filter(schema::community_visible_perms::types.eq(20))
            .select(schema::community_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn join_community(&self, community_id: i32) -> i16 {
        use crate::models::NewCommunitiesMembership;

        if self.is_member_of_community(community_id) || self.is_user_in_ban(community_id) {
            return 0;
        }
        let _connection = establish_connection();
        let new_member = NewCommunitiesMembership {
            user_id: self.user_id,
            community_id: community_id,
            level: 1,
        };
        diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member)
            .execute(&_connection)
            .expect("Error.");
        self.add_new_community_subscriber(community_id);
        return 1;
    }
    pub fn leave_community(&self, community_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        if !self.is_member_of_community(community_id) {
            return 0;
        }
        let _connection = establish_connection();
        self.delete_new_subscriber(community_id);
        diesel::delete ( 
            communities_memberships
                .filter(schema::communities_memberships::user_id.eq(self.user_id))
                .filter(schema::communities_memberships::community_id.eq(community_id))
            )
            .execute(&_connection)
            .expect("E");
        return 1;
    }

    pub fn get_gender_a(&self) -> String {
        if self.is_man {
            return "".to_string();
        }
        return "a".to_string();
    }
    pub fn get_gender(&self) -> String {
        if self.is_man {
            return "Мужской".to_string();
        }
        return "Женский".to_string();
    }

    pub fn is_supermanager(&self) -> bool {
        return self.types == 25;
    }
    pub fn is_administrator(&self) -> bool {
        return self.types > 20 && self.types < 30;
    }
    pub fn is_advertiser(&self) -> bool {
        return self.types > 17 && self.types < 30;
    }
    pub fn is_manager(&self) -> bool {
        return self.types > 13 && self.types < 30;
    }
    pub fn is_moderator(&self) -> bool {
        return self.types > 9 && self.types < 30;
    }
}

/*
Friend
id друзей пользователя, для приватности
записываем id пользователей основного сервиса пользователей.
*/
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Friend {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user_id:   i32,
    pub target_id: i32,
}

/*
Follow
id подписчиков пользователя, для приватности
записываем id пользователей основного сервиса пользователей.
*/
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Follow {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:   i32,
    pub target_id: i32,
}

/*
UserVisiblePerm
types
0 может видеть профиль открытым
1 может видеть записи
2 может видеть комменты к записям
3 может создавать записи
4 может создавать комменты к записям
5 может копировать списки / записи

10 не может видеть профиль открытым
11 не может видеть записи
12 не может видеть комменты к записям
13 не может создавать записи
14 не может создавать комменты к записям
15 не может копировать списки / записи

101 список может видеть записи
102 список может видеть комменты к записи
103 список может создавать записи
104 список может создавать комменты к записи
105 список может копировать списки / записи

111 список не может видеть записи
112 список не может видеть комменты к записи
113 список не может создавать записи
114 список не может создавать комменты к записи
115 список не может копировать списки / записи

20 пользователь заблокирован у владельца записей

51 не создает уведомление о комментарии
52 не создает уведомление о ответе
53 не создает уведомление о упоминании в посте
54 не создает уведомление о упоминании в комменте
55 не создает уведомление о репосте
56 не создает уведомление о реакции

61 не создает уведомление о комментарии
62 не создает уведомление о ответе
63 не создает уведомление о упоминании в посте
64 не создает уведомление о упоминании в комменте
65 не создает уведомление о репосте
66 не создает уведомление о реакции
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:        i32,
    pub user_id:   i32, // какой пользователь добавляет
    pub target_id: i32, // кокого пользователя добавляет
    pub types:     i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}
