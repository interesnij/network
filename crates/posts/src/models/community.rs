use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    CardPostJson,
    CardUserJson,
    CardPostListJson,
    AttachOwner, KeyValue,
    CommunityEditPrivateResp, EditNotifyResp,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgTextExpressionMethods,
};
use crate::schema;
use crate::schema::{
    communitys,
    communities_memberships,
    community_visible_perms,
};
use crate::errors::Error;
use crate::models::{Post, PostList, SearchAllComments, CommunityPostNotification};

/*
Community

Тип сообщества
1 публичное сообщество
2 закрытое сообщество
3 публичное сообщество

7 публичное сообщество подало заявку
8 закрытое сообщество подало заявку
9 публичное сообщество подало заявку

13 публичное сообщество идентификацированное
14 закрытое сообщество идентификацированное
15 публичное сообщество идентификацированное

21 удалено публичное сообщество
22 удалено закрытое сообщество
23 удалено публичное сообщество

27 удалено публичное сообщество подало заявку
28 удалено закрытое сообщество подало заявку
29 удалено публичное сообщество подало заявку

33 удалено публичное сообщество идентификацированное
34 удалено закрытое сообщество идентификацированное
35 удалено публичное сообщество идентификацированное

41 баннер публичное сообщество
42 баннер закрытое сообщество
43 баннер публичное сообщество

47 баннер публичное сообщество подало заявку
48 баннер закрытое сообщество подало заявку
49 баннер публичное сообщество подало заявку

53 баннер публичное сообщество идентификацированное
54 баннер закрытое сообщество идентификацированное
55 баннер публичное сообщество идентификацированное

61 закрыто публичное сообщество
62 закрыто закрытое сообщество
63 закрыто публичное сообщество

67 закрыто публичное сообщество подало заявку
68 закрыто закрытое сообщество подало заявку
69 закрыто публичное сообщество подало заявку

73 закрыто публичное сообщество идентификацированное
74 закрыто закрытое сообщество идентификацированное
75 закрыто публичное сообщество идентификацированное

81 приостановлено публичное сообщество
82 приостановлено закрытое сообщество
83 приостановлено публичное сообщество

87 приостановлено публичное сообщество подало заявку
88 приостановлено закрытое сообщество подало заявку
89 приостановлено публичное сообщество подало заявку

93 приостановлено публичное сообщество идентификацированное
94 приостановлено закрытое сообщество идентификацированное
95 приостановлено публичное сообщество идентификацированное

Приватность
1 Все пользователи
2 Подписчики
3 Персонал
4 Администраторы
5 Владелец сообщества
6 Подписчики, кроме
7 Некоторые подписчики
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Community {
    pub id:             i32,
    pub community_id:   i32,
    pub user_id:        i32,
    pub name:           String,
    pub types:          i16,
    pub link:           String,
    pub s_avatar:       Option<String>,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_list:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="communitys"]
pub struct NewCommunity {
    pub community_id:   i32,
    pub user_id:        i32,
    pub name:           String,
    pub types:          i16,
    pub link:           String,
    pub s_avatar:       Option<String>,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_list:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,

    pub lists:          i32,
    pub posts:          i32,
    pub comments:       i32,
}

#[derive(Deserialize)]
pub struct NewCommunityJson {
    pub token:        Option<String>, 
    pub community_id: Option<i32>,
    pub user_id:      Option<i32>,
    pub name:         Option<String>,
    pub types:        Option<i16>,
    pub link:         Option<String>,
    pub s_avatar:     Option<String>,
}

impl Community {
    pub fn get_notify_field(value: i16) -> KeyValue {
        let info = match value {
            1 => "Все пользователи",
            2 => "Подписчики",
            3 => "Персонал",
            4 => "Администраторы",
            5 => "Отключено",
            6 => "Подписчики, кроме",
            7 => "Некоторые подписчики",
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
            2 => "Подписчики",
            3 => "Персонал",
            4 => "Администраторы",
            5 => "Владелец",
            6 => "Подписчики, кроме",
            7 => "Некоторые подписчики",
            _ => "Ошибка",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    }
    pub fn get_private_json(&self) -> CommunityEditPrivateResp {
        let see_el_users:         Option<Vec<CardUserJson>>;
        let see_comment_users:    Option<Vec<CardUserJson>>;
        let create_el_users:      Option<Vec<CardUserJson>>;
        let create_comment_users: Option<Vec<CardUserJson>>;
        let copy_el_users:        Option<Vec<CardUserJson>>; 

        if self.see_el == 6 {
            see_el_users = Some(self.get_limit_see_el_exclude_members(Some(20), Some(0)));
        }
        else if self.see_el == 7 {
            see_el_users = Some(self.get_limit_see_el_include_members(Some(20), Some(0)));
        }
        else {
            see_el_users = None;
        }

        if self.see_comment == 6 {
            see_comment_users = Some(self.get_limit_see_comment_exclude_members(Some(20), Some(0)));
        }
        else if self.see_comment == 7 {
            see_comment_users = Some(self.get_limit_see_comment_include_members(Some(20), Some(0)));
        }
        else {
            see_comment_users = None;
        }

        if self.create_el == 6 {
            create_el_users = Some(self.get_limit_create_el_exclude_members(Some(20), Some(0)));
        }
        else if self.create_el == 7 {
            create_el_users = Some(self.get_limit_create_el_include_members(Some(20), Some(0)));
        }
        else {
            create_el_users = None;
        }

        if self.create_comment == 6 {
            create_comment_users = Some(self.get_limit_create_comment_exclude_members(Some(20), Some(0)));
        }
        else if self.create_comment == 7 {
            create_comment_users = Some(self.get_limit_create_comment_include_members(Some(20), Some(0)));
        }
        else {
            create_comment_users = None;
        }

        if self.copy_el == 6 {
            copy_el_users = Some(self.get_limit_copy_el_exclude_members(Some(20), Some(0)));
        }
        else if self.copy_el == 7 {
            copy_el_users = Some(self.get_limit_copy_el_include_members(Some(20), Some(0)));
        }
        else {
            copy_el_users = None;
        }
    
        return CommunityEditPrivateResp {
            see_el:               Community::get_private_field(self.see_el),
            see_comment:          Community::get_private_field(self.see_comment),
            create_el:            Community::get_private_field(self.create_el),
            create_comment:       Community::get_private_field(self.create_comment),
            copy_el:              Community::get_private_field(self.copy_el),
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
        
        if notify.comment == 6 {
            comment_users = Some(self.get_limit_comment_exclude_members(Some(20), Some(0)));
        }
        else if notify.comment == 7 {
            comment_users = Some(self.get_limit_comment_include_members(Some(20), Some(0)));
        }
        else {
            comment_users = None;
        }

        if notify.comment_reply == 6 {
            comment_reply_users = Some(self.get_limit_comment_reply_exclude_members(Some(20), Some(0)));
        }
        else if notify.comment_reply == 7 {
            comment_reply_users = Some(self.get_limit_comment_reply_include_members(Some(20), Some(0)));
        }
        else {
            comment_reply_users = None;
        }

        if notify.mention == 6 {
            mention_users = Some(self.get_limit_mention_exclude_members(Some(20), Some(0)));
        }
        else if notify.mention == 7 {
            mention_users = Some(self.get_limit_mention_include_members(Some(20), Some(0)));
        }
        else {
            mention_users = None;
        }

        if notify.comment_mention == 6 {
            comment_mention_users = Some(self.get_limit_comment_mention_exclude_members(Some(20), Some(0)));
        }
        else if notify.comment_mention == 7 {
            comment_mention_users = Some(self.get_limit_comment_mention_include_members(Some(20), Some(0)));
        }
        else {
            comment_mention_users = None;
        }

        if notify.repost == 6 {
            repost_users = Some(self.get_limit_repost_exclude_members(Some(20), Some(0)));
        }
        else if notify.repost == 7 {
            repost_users = Some(self.get_limit_repost_include_members(Some(20), Some(0)));
        }
        else {
            repost_users = None;
        }

        if notify.reactions == 6 {
            reactions_users = Some(self.get_limit_reactions_exclude_members(Some(20), Some(0)));
        }
        else if notify.reactions == 7 {
            reactions_users = Some(self.get_limit_reactions_include_members(Some(20), Some(0)));
        }
        else {
            reactions_users = None;
        }
    
        return EditNotifyResp {
            comment:               Community::get_notify_field(notify.comment),
            comment_reply:         Community::get_notify_field(notify.comment_reply),
            mention:               Community::get_notify_field(notify.mention),
            comment_mention:       Community::get_notify_field(notify.comment_mention),
            repost:                Community::get_notify_field(notify.repost),
            reactions:             Community::get_notify_field(notify.reactions),
            comment_users:         comment_users,
            comment_reply_users:   comment_reply_users,
            mention_users:         mention_users,
            comment_mention_users: comment_mention_users,
            repost_users:          repost_users,
            reactions_users:       reactions_users,
        };
    }
    pub fn get_notify_model(&self) -> Result<CommunityPostNotification, Error> {
        let notify = self.find_notify_model();
        if notify.is_ok() {
            return notify;
        }
        else {
            return self.create_notify_model();
        }
    }
    pub fn create_notify_model(&self) -> Result<CommunityPostNotification, Error> {
        use crate::models::NewCommunityPostNotification;

        let _connection = establish_connection();
        let _new_notify = NewCommunityPostNotification {
            community_id:    self.id, 
            comment:         1,
            comment_reply:   1,
            mention:         1,
            comment_mention: 1,
            repost:          1,
            reactions:       1,
        };
        let _notify = diesel::insert_into(schema::community_post_notifications::table)
            .values(&_new_notify)
            .get_result::<CommunityPostNotification>(&_connection)?;

        return Ok(_notify);
    }
    pub fn find_notify_model(&self) -> Result<CommunityPostNotification, Error> {
        use crate::schema::community_post_notifications::dsl::community_post_notifications;

        let _connection = establish_connection();
        let notify = community_post_notifications
            .filter(schema::community_post_notifications::community_id.eq(self.id))
            .first(&_connection)?;
        return Ok(notify);
    }
    pub fn get_main_post_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let _post_list = post_lists
            .filter(schema::post_lists::community_id.eq(self.id))
            .filter(schema::post_lists::types.eq(0))
            .first::<PostList>(&_connection);
        if _post_list.is_ok() {
            return _post_list.expect("E.");
        }
        else {
            use crate::models::{NewPostList, NewCommunityPostListPosition};
            let new_list = NewPostList {
                name:            "Основной список".to_string(),
                community_id:    Some(self.id),
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
                create_el:       17,
                create_comment:  1,
                copy_el:         1,
                reactions:       Some("1, 2".to_string()),
            };
            let _posts_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_list)
                .get_result::<PostList>(&_connection)
                .expect("Error saving post_list.");

            let _new_posts_list_position = NewCommunityPostListPosition {
                community_id:  self.id,
                list_id:  _posts_list.id,
                position: 1,
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::community_post_list_positions::table)
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
            .filter(schema::post_lists::community_id.eq(self.id))
            .filter(schema::post_lists::types.eq(0))
            .first::<PostList>(&_connection);
 
        if _post_list.is_err() {
            use crate::models::{NewPostList, NewCommunityPostListPosition};
            let new_list = NewPostList {
                    name:            "Основной список".to_string(),
                    community_id:    Some(self.id),
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
                    create_el:       17,
                    create_comment:  1,
                    copy_el:         1,
                    reactions:       Some("1, 2".to_string()),
            };
            let _posts_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_list)
                .get_result::<PostList>(&_connection)
                .expect("Error saving post_list.");

            let _new_posts_list_position = NewCommunityPostListPosition {
                community_id:  self.id,
                list_id:  _posts_list.id,
                position: 1,
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::community_post_list_positions::table)
                .values(&_new_posts_list_position)
                .execute(&_connection)
                .expect("Error saving post_list_position.");
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
                .set(schema::community_post_notifications::comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "comment_reply" => diesel::update(&notify)
                .set(schema::community_post_notifications::comment_reply.eq(value))
                .execute(&_connection)
                .expect("E."),
            "mention" => diesel::update(&notify)
                .set(schema::community_post_notifications::mention.eq(value))
                .execute(&_connection)
                .expect("E."),
            "comment_mention" => diesel::update(&notify)
                .set(schema::community_post_notifications::comment_mention.eq(value))
                .execute(&_connection)
                .expect("E."),
            "repost" => diesel::update(&notify)
                .set(schema::community_post_notifications::repost.eq(value))
                .execute(&_connection)
                .expect("E."),
            "reactions" => diesel::update(&notify)
                .set(schema::community_post_notifications::reactions.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
        };
        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::community_visible_perms::dsl::community_visible_perms;
            match value { 
                51 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(61))
                    )
                    .execute(&_connection)
                    .expect("E"),
                52 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(62))
                    )
                    .execute(&_connection)
                    .expect("E"),
                53 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(63))
                    )
                    .execute(&_connection)
                    .expect("E"),
                54 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(64))
                    )
                    .execute(&_connection)
                    .expect("E"),
                55 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(65))
                    )
                    .execute(&_connection)
                    .expect("E"),
                56 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(66))
                    )
                    .execute(&_connection)
                    .expect("E"),
                61 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(51))
                    )
                    .execute(&_connection)
                    .expect("E"),
                62 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(52))
                    )
                    .execute(&_connection)
                    .expect("E"),
                63 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(53))
                    )
                    .execute(&_connection)
                    .expect("E"),
                64 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(54))
                    )
                    .execute(&_connection)
                    .expect("E"),
                65 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(55))
                    )
                    .execute(&_connection)
                    .expect("E"),
                66 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(56))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if _users.is_some() && is_ie_mode {
            for user_id in _users.unwrap().iter() {
                let _new_perm = NewCommunityVisiblePerm {
                    community_id: self.community_id,
                    target_id:    *user_id,
                    types:        value,
                };
                diesel::insert_into(schema::community_visible_perms::table)
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
        let is_ie_mode = vec![6,7].iter().any(|&i| i==value);
        if value < 1 || value > 120 || (is_ie_mode && items_ids.is_none()) {
            return 0;
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_el" => diesel::update(self)
                .set(schema::communitys::see_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_comment" => diesel::update(self)
                .set(schema::communitys::see_comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "create_el" => diesel::update(self)
                .set(schema::communitys::create_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            "create_comment" => diesel::update(self)
                .set(schema::communitys::create_comment.eq(value))
                .execute(&_connection)
                .expect("E."),
            "create_list" => diesel::update(self)
                .set(schema::communitys::create_list.eq(value))
                .execute(&_connection)
                .expect("E."),
            "copy_el" => diesel::update(self)
                .set(schema::communitys::copy_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
        };

        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::community_visible_perms::dsl::community_visible_perms;
            match value { 
                6 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(16))
                    )
                    .execute(&_connection)
                    .expect("E"),
                1 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(11))
                    )
                    .execute(&_connection)
                    .expect("E"),
                2 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(12))
                    )
                    .execute(&_connection)
                    .expect("E"),
                3 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(13))
                    )
                    .execute(&_connection)
                    .expect("E"),
                4 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(14))
                    )
                    .execute(&_connection)
                    .expect("E"),
                5 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(15))
                    )
                    .execute(&_connection)
                    .expect("E"),
                16 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(6))
                    )
                    .execute(&_connection)
                    .expect("E"),
                11 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(1))
                    )
                    .execute(&_connection)
                    .expect("E"),
                12 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(2))
                    )
                    .execute(&_connection)
                    .expect("E"),
                13 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(3))
                    )
                    .execute(&_connection)
                    .expect("E"),
                14 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(4))
                    )
                    .execute(&_connection)
                    .expect("E"),
                15 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(5))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if items_ids.is_some() && is_ie_mode {
            for item_id in items_ids.unwrap().iter() {
                let _new_perm = NewCommunityVisiblePerm {
                    community_id: self.community_id,
                    item_id:      *item_id,
                    types:        value,
                };
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }
    pub fn is_user_member(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_staff(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.ne(1))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_admin(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.eq(5))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn get_post_lists (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>,
    ) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::community_id.eq(self.community_id))
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
        offset: Option<i64>,
    ) -> Vec<CardPostListJson> {
        use crate::schema::post_lists::dsl::post_lists;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let mut lists_json = Vec::new();
        let lists =  post_lists
            .filter(schema::post_lists::community_id.eq(self.community_id))
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
        offset:  Option<i64>,
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
                .filter(schema::post_comments::community_id.eq(self.id))
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

    pub fn edit_name(&self, name: &str) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::communitys::name.eq(name),
            ))
            .execute(&_connection)
            .expect("E.");

        let some_item_community = item_communitys
            .filter(schema::item_communitys::community_id.eq(self.community_id))
            .first::<ItemCommunity>(&_connection);
        if some_item_community.is_ok() {
            let i_e = some_item_community.expect("E.");
            let _i = diesel::update(&i_e)
                .set((  
                    schema::item_communitys::name.eq(name),
                ))
                .execute(&_connection);
        }
        return 1;
    }
    pub fn edit_link(&self, link: &str) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::communitys::link.eq(link),
            ))
            .execute(&_connection)
            .expect("E.");

        let some_item_community = item_communitys
            .filter(schema::item_communitys::community_id.eq(self.community_id))
            .first::<ItemCommunity>(&_connection);
        if some_item_community.is_ok() {
            let i_e = some_item_community.expect("E.");
            let _i = diesel::update(&i_e)
                .set((  
                    schema::item_communitys::link.eq(link),
                ))
                .execute(&_connection);
        }
        return 1;
    }
    pub fn edit_avatar(&self, avatar: &str) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::communitys::s_avatar.eq(avatar),
            ))
            .execute(&_connection)
            .expect("E.");

        let some_item_community = item_communitys
            .filter(schema::item_communitys::community_id.eq(self.community_id))
            .first::<ItemCommunity>(&_connection);
        if some_item_community.is_ok() {
            let i_e = some_item_community.expect("E.");
            let _i = diesel::update(&i_e)
                .set((  
                    schema::item_communitys::s_avatar.eq(avatar),
                ))
                .execute(&_connection);
        }
        return 1;
    }

    pub fn delete_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            1 => 21,
            2 => 22,
            3 => 23,
            7 => 27,
            8 => 28,
            9 => 29,
            13 => 33,
            14 => 34,
            15 => 35,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
        }

            if o.is_ok() {
                return 1;
            }
        }
        return 0;
    }
    pub fn restore_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            21 => 1,
            22 => 2,
            23 => 3,
            27 => 7,
            28 => 8,
            29 => 9,
            33 => 13,
            34 => 14,
            35 => 15,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
            }
            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn close_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            1 => 61,
            2 => 62,
            3 => 63,
            7 => 67,
            8 => 68,
            9 => 69,
            13 => 73,
            14 => 74,
            15 => 75,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
            }
            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn unclose_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            61 => 1,
            62 => 2,
            63 => 3,
            67 => 7,
            68 => 8,
            69 => 9,
            73 => 13,
            74 => 14,
            75 => 15,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
            }
            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn suspend_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            1 => 81,
            2 => 82,
            3 => 83,
            7 => 87,
            8 => 88,
            9 => 89,
            13 => 93,
            14 => 94,
            15 => 95,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
            }
            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn unsuspend_item(&self) -> i16 {
        use crate::schema::item_communitys::dsl::item_communitys;
        use crate::models::ItemCommunity;

        let _connection = establish_connection();
        let _case = match self.types {
            81 => 1,
            82 => 2,
            83 => 3,
            87 => 7,
            88 => 8,
            89 => 9,
            93 => 13,
            94 => 14,
            95 => 15,
            _ => 0,
        };
        if _case != 0 {
            let o = diesel::update(self)
                .set(schema::communitys::types.eq(_case))
                .execute(&_connection);

            let some_item_community = item_communitys
                .filter(schema::item_communitys::community_id.eq(self.community_id))
                .first::<ItemCommunity>(&_connection);
            if some_item_community.is_ok() {
                let i_e = some_item_community.expect("E.");
                let _i = diesel::update(&i_e)
                    .set(schema::item_communitys::types.eq(_case))
                    .execute(&_connection);
            }
            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }

    pub fn get_fixed_posts_ids(&self) -> Vec<i32> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::community_id.eq(self.community_id))
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

    pub fn create_community (
        community_id: i32,
        user_id:      i32,
        name:         String,
        types:        i16,
        link:         String,  
        s_avatar:     Option<String>,
    ) -> i16 {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        if communitys
            .filter(schema::communitys::community_id.eq(community_id))
            .select(schema::communitys::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }
        let new_community_form = NewCommunity {
            community_id:   community_id,
            user_id:        user_id,
            name:           name.clone(),
            types:          types,
            link:           link.clone(),
            s_avatar:       s_avatar.clone(),

            see_el:         1,
            see_comment:    1,
            create_list:    4,
            create_el:      4,
            create_comment: 1,
            copy_el:        1,

            lists:          0,
            posts:          0,
            comments:       0,
        };
        let _community = diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .get_result::<Community>(&_connection)
            .expect("Error.");
        _community.create_main_post_list();
        return 1;
    }

    pub fn plus_lists(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::communitys::lists.eq(self.lists + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_lists(&self, count: i32) -> bool {
        if self.lists > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::communitys::lists.eq(self.lists - count))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_posts(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::communitys::posts.eq(self.posts + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_posts(&self, count: i32) -> bool {
        if self.posts > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::communitys::posts.eq(self.posts - count))
                .execute(&_connection)
                .expect("Error.");
            return true;
        }
        return false;
    }
    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::communitys::comments.eq(self.comments + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        if self.comments > 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::communitys::comments.eq(self.comments - count))
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
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_identified(&self) -> bool {
        return self.types > 12 || self.types < 16;
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types > 6 || self.types < 10;
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn get_code(&self) -> String {
        return "com".to_string() + &self.get_str_id();
    }
    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }

    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 20 || self.types < 40;
    }
    pub fn is_suspended(&self) -> bool {
        return self.types > 80 || self.types < 100;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 60 || self.types < 80;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types > 40 || self.types < 60;
    }
    pub fn is_private(&self) -> bool {
        return self.types == 3 || self.types == 9 || self.types == 15;
    }
    pub fn is_close(&self) -> bool {
        return self.types == 2 || self.types == 8 || self.types == 14;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 1 || self.types == 7 || self.types == 13;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 20;
    }
    pub fn is_user_in_ban(&self, user_id: i32) -> bool {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        return community_visible_perms
            .filter(schema::community_visible_perms::target_id.eq(user_id))
            .filter(schema::community_visible_perms::community_id.eq(self.community_id))
            .filter(schema::community_visible_perms::types.eq(20))
            .select(schema::community_visible_perms::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn create_banned_user(&self, user_id: i32) -> i16 {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();

        let del = diesel::delete (
            community_visible_perms
                .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                .filter(schema::community_visible_perms::target_id.eq(user_id))
            )
            .execute(&_connection);

        let new_banned_user = NewCommunityVisiblePerm {
            community_id: self.community_id,
            target_id:    user_id,
            types:        20,
        };
        let ok_1 = diesel::insert_into(schema::community_visible_perms::table)
            .values(&new_banned_user)
            .execute(&_connection);

        if del.is_ok() && ok_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn delete_banned_user(&self, user_id: i32) -> i16 {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let del = diesel::delete (
            community_visible_perms
                .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                .filter(schema::community_visible_perms::target_id.eq(user_id))
                .filter(schema::community_visible_perms::types.eq(20))
            )
            .execute(&_connection);
        if del.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn create_administrator(&self, user_id: i32) -> i16 {
        // нужно создавать объект уведомлений для сообщества для нового админа
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.is_user_member(user_id) {
            return 0;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection)
            .expect("E");

        let ok = diesel::update(&member)
            .set(schema::communities_memberships::level.eq(5))
            .execute(&_connection);
        if ok.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn create_editor(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.is_user_member(user_id) {
            return 0;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection)
            .expect("E");

        let ok = diesel::update(&member)
            .set(schema::communities_memberships::level.eq(3))
            .execute(&_connection);
        if ok.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn create_moderator(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.is_user_member(user_id) {
            return 0;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection)
            .expect("E");

        let ok = diesel::update(&member)
            .set(schema::communities_memberships::level.eq(2))
            .execute(&_connection);
        if ok.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn create_advertisor(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.is_user_member(user_id) {
            return 0;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection)
            .expect("E");

        let ok = diesel::update(&member)
            .set(schema::communities_memberships::level.eq(4))
            .execute(&_connection);
        if ok.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn delete_staff_member(&self, user_id: i32) -> i16 {
        // нужно удалять объект уведомлений для сообщества
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.is_user_staff(user_id) {
            return 0;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection)
            .expect("E");

        let ok = diesel::update(&member)
            .set(schema::communities_memberships::level.eq(1))
            .execute(&_connection);
        if ok.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn get_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_6_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .limit(6)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_staff_users_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.ne(1))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items;
    }
    pub fn get_administrators_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.eq(5))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_moderators_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.eq(2))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_editors_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::level.eq(3))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    
    pub fn is_user_perm_exists (
        &self,
        user_id: i32,
        types:   i16, 
    ) -> bool {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            communities_memberships::dsl::communities_memberships,
        };

        let _connection = establish_connection();
        return community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.community_id))
            .filter(schema::community_visible_perms::target_id.eq(user_id))
            .filter(schema::community_visible_perms::types.eq(types))
            .select(schema::community_visible_perms::target_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_ie_members_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            item_users::dsl::item_users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.community_id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::community_visible_perms::target_id)
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

    pub fn get_limit_see_el_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_el_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(1, limit, offset); 
    } 
    pub fn get_limit_see_comment_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_comment_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(2, limit, offset); 
    }
    pub fn get_limit_create_el_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(13, limit, offset); 
    }
    pub fn get_limit_create_el_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(3, limit, offset); 
    }
    pub fn get_limit_create_comment_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(14, limit, offset); 
    }
    pub fn get_limit_create_comment_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(4, limit, offset); 
    }
    pub fn get_limit_copy_el_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(15, limit, offset); 
    }
    pub fn get_limit_copy_el_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(5, limit, offset); 
    }
    pub fn get_limit_create_list_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(16, limit, offset); 
    }
    pub fn get_limit_create_list_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(6, limit, offset); 
    }

    pub fn get_limit_comment_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(61, limit, offset); 
    }
    pub fn get_limit_comment_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(51, limit, offset); 
    } 
    pub fn get_limit_comment_reply_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(62, limit, offset); 
    }
    pub fn get_limit_comment_reply_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(52, limit, offset); 
    }
    pub fn get_limit_mention_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(63, limit, offset); 
    }
    pub fn get_limit_mention_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(53, limit, offset); 
    }
    pub fn get_limit_comment_mention_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(64, limit, offset); 
    }
    pub fn get_limit_comment_mention_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(54, limit, offset); 
    }
    pub fn get_limit_repost_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(65, limit, offset); 
    }
    pub fn get_limit_repost_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(55, limit, offset); 
    }
    pub fn get_limit_reactions_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(66, limit, offset); 
    }
    pub fn get_limit_reactions_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(56, limit, offset); 
    }
    

    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        // может ли пользователь просматривать все списки и посты
        // данного сообщества
        return match self.see_el {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 11),
            7 => self.is_user_perm_exists(user_id, 1),
            _ => false
        };
    }
    pub fn is_user_see_comment(&self, user_id: i32) -> bool {
        return match self.see_comment {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 12),
            7 => self.is_user_perm_exists(user_id, 2),
            _ => false
        };
    }
    pub fn is_user_create_list(&self, user_id: i32) -> bool {
        return match self.create_el {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 16),
            7 => self.is_user_perm_exists(user_id, 6),
            _ => false
        };
    }
    pub fn is_user_create_el(&self, user_id: i32) -> bool {
        return match self.create_el {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 13),
            7 => self.is_user_perm_exists(user_id, 3),
            _ => false
        };
    }
    pub fn is_user_create_comment(&self, user_id: i32) -> bool {
        return match self.create_comment {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 14),
            7 => self.is_user_perm_exists(user_id, 4),
            _ => false
        };
    }
    pub fn is_user_copy_el(&self, user_id: i32) -> bool {
        return match self.copy_el {
            1 => true,
            2 => self.is_user_member(user_id),
            3 => self.is_user_staff(user_id),
            4 => self.is_user_admin(user_id),
            5 => self.user_id == user_id,
            6 => !self.is_user_perm_exists(user_id, 15),
            7 => self.is_user_perm_exists(user_id, 5),
            _ => false
        };
    }

    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1;
    }
    pub fn is_anon_user_see_comment(&self) -> bool {
        return self.see_comment == 1;
    }

    pub fn set_user_visible_perms(&self, users: String, types: i16) -> bool {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let mut users_ids = Vec::new();
        let v: Vec<&str> = users.split(", ").collect();
        for item in v.iter() {
            if !item.is_empty() {
                let pk: i32 = item.parse().unwrap();
                users_ids.push(pk);
            }
        }

        // нужно удалить из списка тех, кто был туда внесен
        // с противоположными правами.
        match types {
            1 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(11))
                )
                .execute(&_connection)
                .expect("E"),
            11 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(1))
                )
                .execute(&_connection)
                .expect("E"),
            2 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(12))
                )
                .execute(&_connection)
                .expect("E"),
            12 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(2))
                )
                .execute(&_connection)
                .expect("E"),
            3 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(13))
                )
                .execute(&_connection)
                .expect("E"),
            13 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(3))
                )
                .execute(&_connection)
                .expect("E"),
            4 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(14))
                )
                .execute(&_connection)
                .expect("E"),
            14 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(4))
                )
                .execute(&_connection)
                .expect("E"),
            5 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(15))
                )
                .execute(&_connection)
                .expect("E"),
            15 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.community_id))
                        .filter(schema::community_visible_perms::types.eq(5))
                )
                .execute(&_connection)
                .expect("E"),
            _ => 0,
        };
        for user_id in users_ids.iter() {
            let _new_perm = NewCommunityVisiblePerm {
                community_id: self.community_id,
                target_id:    *user_id,
                types:        types,
            };
            diesel::insert_into(schema::community_visible_perms::table)
                .values(&_new_perm)
                .execute(&_connection)
                .expect("Error.");
        }
        return true;
    }
    pub fn update_staff_member(&self, user_id: i32, level: i16) -> i16 { 
        use crate::schema::communities_memberships::dsl::communities_memberships;
 
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(level))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
        };
    }
}

/////// CommunitiesMembership //////
// level
// 1 подписчик
// 2 модератор
// 3 редактор
// 4 рекламщик
// 5 администратор
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunitiesMembership {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: i32,
    pub level:        i16,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="communities_memberships"]
pub struct NewCommunitiesMembership {
    pub user_id:      i32,
    pub community_id: i32,
    pub level:        i16,
}
impl CommunitiesMembership {
    pub fn create_membership (
        user_id: i32,
        community: &Community,
        level: i16
    ) -> CommunitiesMembership {
        let _connection = establish_connection();

        let new_member_form = NewCommunitiesMembership {
            user_id:      user_id,
            community_id: community.id,
            level:        level,
        };
        let new_member = diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member_form)
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("E.");
        return new_member;
    }
}

/*
включения и исключения для пользователей касательно конкретного сообщества
1 может видеть записи
2 может видеть комменты к записям
3 может создавать записи
4 может создавать комменты к записям
5 может копировать списки / записи
6 может создавать списки

11 не может видеть записи
12 не может видеть комменты к записям
13 не может создавать записи
14 не может создавать комменты к записям
15 не может копировать списки / записи
16 не может создавать списки 

20 пользователь заблокирован у сообщества записей

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
pub struct CommunityVisiblePerm {
    pub id:           i32,
    pub community_id: i32, // какое сообщество добавляет
    pub target_id:    i32, // кокого пользователя добавляет
    pub types:        i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_visible_perms"]
pub struct NewCommunityVisiblePerm {
    pub community_id: i32,
    pub target_id:    i32,
    pub types:        i16,
}
