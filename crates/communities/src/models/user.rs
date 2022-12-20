use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    get_limit,
    CardUserJson,
    CardCommunityJson,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgTextExpressionMethods,
    NullableExpressionMethods,
};
use crate::schema;
use crate::schema::{
    users,
    friends,
    follows,
    user_visible_perms,
};

/*
Типы пользоватетеля
1 стандартный тип пользователя
6 пославший запрос на идентификацию
7 идентифицированный

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
36 удаленный пославший запрос на идентификацию
37 удаленный идентифицированный

41 закрытый стандартный
46 закрытый пославший запрос на идентификацию
47 закрытый идентифицированный

51 приостановленный стандартный
56 приостановленный пославший запрос на идентификацию
57 приостановленный идентифицированный

61 закрытый баннером стандартный
66 закрытый баннером пославший запрос на идентификацию
67 закрытый баннером идентифицированный

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
*/

#[derive(Serialize, Identifiable, Queryable)]
pub struct User {
    pub id:            i32,
    pub user_id:       i32,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         i16,
    pub is_man:        bool,
    pub password:      String,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub see_all:       i16,
    pub see_community: i16,
    pub communities:   i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_id:       i32,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         i16,
    pub is_man:        bool,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub see_all:       i16,
    pub see_community: i16,
    pub communities:   i32,
}
#[derive(Deserialize, Serialize)]
pub struct NewUserJson {
    pub token:      Option<String>,
    pub user_id:    Option<i32>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub is_man:     Option<i16>,
    pub password:   Option<String>,
    pub link:       Option<String>,
}

impl User {
    pub fn get_communities (
        &self, 
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardCommunityJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            communitys::dsl::communitys,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let communities_ids = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .select(schema::communities_memberships::user_id)
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");
        return communitys
            .filter(schema::communitys::id.eq_any(communities_ids))
            .filter(schema::communitys::types.lt(20))
            .select((
                schema::communitys::user_id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::s_avatar.nullable(),
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
            .expect("E.");
    }
    pub fn get_limit_communities(&self, limit: Option<i64>) -> Vec<CardCommunityJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            communitys::dsl::communitys,
        };

        let _limit = get_limit(limit, 20);
        let _connection = establish_connection();
        let communities_ids = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .select(schema::communities_memberships::user_id)
            .limit(_limit)
            .load::<i32>(&_connection)
            .expect("E.");
        return communitys
            .filter(schema::communitys::id.eq_any(communities_ids))
            .filter(schema::communitys::types.lt(20))
            .select((
                schema::communitys::user_id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::s_avatar.nullable(),
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
            .expect("E.");
    }
    pub fn search_communities (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardCommunityJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            communitys::dsl::communitys,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let communities_ids = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return communitys
            .filter(schema::communitys::id.eq_any(communities_ids))
            .filter(schema::communitys::name.ilike(&q))
            .filter(schema::communitys::types.lt(20))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::communitys::user_id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::s_avatar.nullable(),
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
            .expect("E.");
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
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::users::first_name.eq(first_name),
                schema::users::last_name.eq(last_name)
            ))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_link(&self, link: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::link.eq(link))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_avatar(&self, s_avatar: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::s_avatar.eq(s_avatar))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }

    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _users: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 13 || (is_ie_mode && _users.is_none()) {
            return 0; 
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_all" => diesel::update(self)
                .set(schema::users::see_all.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_community" => diesel::update(self)
                .set(schema::users::see_community.eq(value))
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
                _ => 0,
            };
        };
        if _users.is_some() && is_ie_mode {
            for _user in _users.unwrap().iter() {
                let _new_perm = NewUserVisiblePerm {
                    user_id:   self.user_id,
                    target_id: *_user,
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
    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }

    pub fn count_communities(&self) -> i32 {
        return self.communities;
    }
    pub fn count_communities_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn count_communities_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_communities(),
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn plus_communities(&self, count: i32) -> bool {
        let _connection = establish_connection();
        let _u = diesel::update(self)
            .set(schema::users::communities.eq(self.communities + count))
            .execute(&_connection);
        if _u.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn minus_communities(&self, count: i32) -> bool {
        if self.communities > 0 {
            let _connection = establish_connection();
            let _u = diesel::update(self)
                .set(schema::users::communities.eq(self.communities - count))
                .execute(&_connection);
            if _u.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }
        return false;
    }

    pub fn create_user (
        user_id:    i32,
        first_name: String,
        last_name:  String,
        is_man:     bool,
        password:   String,
        link:       String,
    ) -> -> i32 {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        if users
            .filter(schema::users::user_id.eq(user.user_id))
            .select(schema::users::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }
        let new_form = NewUser {
            user_id:       user.user_id,
            first_name:    user.first_name.clone(),
            last_name:     user.last_name.clone(),
            types:         1,
            is_man:        user.is_man,
            password:      user.password.clone(),
            link:          user.link.clone(),
            s_avatar:      None,
            last_activity: chrono::Local::now().naive_utc(),
            see_all:       1,
            see_community: 1,
            communities:   0,
        };
        let _new_user = diesel::insert_into(schema::users::table)
            .values(&new_form)
            .execute(&_connection);
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
            .filter(schema::user_visible_perms::user_id.eq(self.id))
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
            users::dsl::users,
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

        return users
            .filter(schema::users::id.eq_any(items_ids))
            .filter(schema::users::types.lt(6))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
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
            users::dsl::users,
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

        return users
            .filter(schema::users::id.eq_any(items_ids))
            .filter(schema::users::types.lt(6))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
    }

    pub fn get_limit_see_all_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_all_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(1, limit, offset); 
    }
    pub fn get_limit_see_all_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_all_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(1, limit, offset); 
    }

    pub fn get_limit_see_community_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_community_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(2, limit, offset); 
    }
    pub fn get_limit_see_community_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_community_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(2, limit, offset); 
    }

    pub fn is_user_see_community(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_community {
            1 => true,
            2 => self.is_connected_with_user_with_id(user_id) || self.is_self_followers_user_with_id(user_id),
            3 => self.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 11),
            4 => self.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 1),
            5 => self.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 11),
            6 => self.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 1),
            7 => self.is_connected_with_user_with_id(user_id),
            8 => self.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 11),
            10 => self.is_friend_perm_exists(user_id, 1),
            11 => !self.is_follow_perm_exists(user_id, 11),
            12 => self.is_follow_perm_exists(user_id, 1),
            _ => false,
        };
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

    pub fn get_friends_ids(&self) -> Vec<i32> {
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

    pub fn delete_item(&self) -> i16 {
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
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
        }
        return 0;
    }
    pub fn restore_item(&self) -> i16 {
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
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

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
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

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
                .set(schema::users::types.eq(_case))
                .execute(&_connection);

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
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
    pub fn is_suspended(&self) -> bool {
        return self.types < 40 && self.types > 30;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types < 50 && self.types > 40;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types < 20 && self.types > 10;
    }
    pub fn is_closed(&self) -> bool {
        return self.types < 30 && self.types > 20;
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types == 6;
    }
    pub fn is_identified(&self) -> bool {
        return self.types > 6 && self.types < 30;
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
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::target_id.eq(user_id))
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(user_id))
            .filter(schema::user_visible_perms::target_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_connected_with_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::friends::dsl::friends;

        let _connection = establish_connection();
        return friends
            .filter(schema::friends::user_id.eq(user_id))
            .filter(schema::friends::target_id.eq(self.id))
            .select(schema::friends::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_following_user_with_id(&self, user_id: i32) -> bool {
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::user_id.eq(self.user_id))
            .filter(schema::follows::target_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_followers_user_with_id(&self, user_id: i32) -> bool {
        // подписан ли user_id на self
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_followers_user_with_id(&self, user_id: i32) -> bool {
        // подписан ли self на user_id
        use crate::schema::follows::dsl::follows;

        let _connection = establish_connection();
        return follows
            .filter(schema::follows::target_id.eq(user_id))
            .filter(schema::follows::user_id.eq(self.user_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection).is_ok();
    }

    pub fn is_anon_user_see_community(&self) -> bool {
        return self.see_community == 1;
    }
    pub fn is_anon_user_see_all(&self) -> bool {
        return self.see_all == 1;
    }
    pub fn follow_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || self.is_self_user_in_block(user_id) || self.is_followers_user_with_id(user_id) || self.is_following_user_with_id(user_id) {
            return 0;
        }

        let _connection = establish_connection();
        let _new_follow = NewFollow {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);
        if new_follow.is_ok() {
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
        use crate::schema::follows::dsl::follows;

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
                return 1;
            }
            else {
                return 0;
            }
        }
        else {
            return 0;
        }
    }

    pub fn frend_user(&self, user_id: i32) -> i16 {
        // тут друзья создаются всего в одном экземпляре, где
        // self.user_id - это id создающего, а user_id -
        // id создаваемого. Это нужно для фильтрации приватности по
        // друзьям.
        if self.user_id == user_id || !self.is_followers_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::follows::dsl::follows;

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
        use crate::schema::friends::dsl::friends;

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
                    .filter(schema::friends::target_id.eq(user_id)))
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
            diesel::delete (
                follows
                    .filter(schema::follows::user_id.eq(self.user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }

        let _user_block = NewUserVisiblePerm {
            user_id:   self.user_id,
            target_id: user_id,
            types:     20,
        };
        diesel::insert_into(schema::user_visible_perms::table)
            .values(&_user_block)
            .execute(&_connection)
            .expect("Error.");
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

    pub fn join_community(&self, community_id: i32) -> i16 {
        use crate::models::NewCommunitiesMembership;

        if self.is_member_of_community(community_id) || self.is_user_in_ban(community_id) {
            return 0;
        }
        let _connection = establish_connection();
        let new_member = NewCommunitiesMembership {
            user_id:      self.user_id,
            community_id: community_id,
            level:        1,
            created:      chrono::Local::now().naive_utc(),
            visited:      1, 
        };
        diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member)
            .execute(&_connection)
            .expect("Error.");
        self.plus_communities(1);
        return 1;
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
    pub fn is_user_in_ban(&self, community_id: i32) -> bool {
        use crate::schema::community_banned_users::dsl::community_banned_users;

        let _connection = establish_connection();
        return community_banned_users
            .filter(schema::community_banned_users::user_id.eq(self.user_id))
            .filter(schema::community_banned_users::community_id.eq(community_id))
            .select(schema::community_banned_users::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn leave_community(&self, community_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        if !self.is_member_of_community(community_id) {
            return 0;
        }
        let _connection = establish_connection();
        diesel::delete ( 
            communities_memberships
                .filter(schema::communities_memberships::user_id.eq(self.user_id))
                .filter(schema::communities_memberships::community_id.eq(community_id))
            )
            .execute(&_connection)
            .expect("E");
        self.minus_communities(1);
        return 1;
    }
}

/////// Friend //////
// id друзей пользователя, для приватности
// записываем id пользователей основного сервиса пользователей.
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Friend {
    pub id:        i32,
    pub user_id:   i32, // кто
    pub target_id: i32, // на кого
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
    pub user_id:   i32, // кто
    pub target_id: i32, // на кого
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:   i32,
    pub target_id: i32,
}

/*
включения и исключения для пользователей касательно конкретного пользоватетеля
Связь - с пользователями сервиса пользователей, так как
могут сюда внести того, кого в этом сервисе нет.
ведь запрос передается либо для анонима, либо с id основного
пользоваетля.

0 может видеть профиль
1 может видеть сообщества
10 не может видеть профиль
11 не может видеть сообщества
20 пользователь заблокирован у владельца блока сообществ
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id:   i32,
    pub target_id: i32,
    pub types:     i16,
}
