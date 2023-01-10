use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    get_limit, get_user,
    CardUserJson, KeyValue,
    CardCommunityJson, SectionJson,
    EditUserPrivateResp, CardCommunitiesList,
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
    community_follows,
    community_invites,
};
use crate::models::{
    CommunitiesList, FollowsList, FriendsList,
    Community
};

/*
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
    pub invite:        i16,
    pub lists:         i16,
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
    pub password:      String,
    pub link:          String,
    pub s_avatar:      Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub see_all:       i16,
    pub see_community: i16,
    pub invite:        i16,
    pub lists:         i16,
    pub communities:   i32,
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
    pub friends_list: Option<i32>,
    pub follows_list: Option<i32>,
}

impl User {
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
            13 => "Отключено",
            _ => "Ошибка",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    }
    pub fn get_private_json(&self) -> EditUserPrivateResp {
        let see_community_users: Option<Vec<CardUserJson>>;
        let invite_users:        Option<Vec<CardUserJson>>;
        
        if self.see_community == 5 || self.see_community == 9 {
            see_community_users = Some(self.get_limit_see_community_exclude_friends(Some(20), Some(0)));
        }
        else if self.see_community == 3 || self.see_community == 11 {
            see_community_users = Some(self.get_limit_see_community_exclude_follows(Some(20), Some(0)));
        }
        else if self.see_community == 6 || self.see_community == 10 {
            see_community_users = Some(self.get_limit_see_community_include_friends(Some(20), Some(0)));
        }
        else if self.see_community == 4 || self.see_community == 12 {
            see_community_users = Some(self.get_limit_see_community_include_follows(Some(20), Some(0)));
        }
        else {
            see_community_users = None;
        }

        if self.invite == 5 || self.invite == 9 {
            invite_users = Some(self.get_limit_invite_exclude_friends(Some(20), Some(0)));
        }
        else if self.invite == 3 || self.invite == 11 {
            invite_users = Some(self.get_limit_invite_exclude_follows(Some(20), Some(0)));
        }
        else if self.invite == 6 || self.invite == 10 {
            invite_users = Some(self.get_limit_invite_include_friends(Some(20), Some(0)));
        }
        else if self.invite == 4 || self.invite == 12 {
            invite_users = Some(self.get_limit_invite_include_follows(Some(20), Some(0)));
        }
        else {
            invite_users = None;
        }
    
        return EditUserPrivateResp {
            see_community:       User::get_private_field(self.see_community),
            invite:              User::get_private_field(self.invite),
            see_community_users: see_community_users,
            invite_users:        invite_users,
        };
    }
    pub fn get_common_friends_of_community (
        &self, 
        community_id: i32, 
        limit: Option<i64>, 
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            communities_memberships::dsl::communities_memberships,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let self_friends = self.get_friends_ids();
        let members_of_community = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        let mut count = 0;
        for member_id in members_of_community.iter() {
            if self_friends.iter().any(|i| i==member_id) && count < _limit {
                stack.push(member_id);
                count += 1;
            }
        }
        return users
            .filter(schema::users::user_id.eq_any(stack))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
    }

    pub fn search_common_friends_of_community (
        &self,
        community_id: i32,
        q: &String,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            users::dsl::users,
            communities_memberships::dsl::communities_memberships,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);

        let self_friends = self.get_friends_ids();
        let members_of_community = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(community_id))
            .limit(_limit)
            .offset(_offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        let mut count = 0;
        for member_id in members_of_community.iter() {
            if self_friends.iter().any(|i| i==member_id) && count < _limit {
                stack.push(member_id);
                count += 1;
            }
        }
        return users
            .filter(schema::users::user_id.eq_any(stack))
            .filter(schema::users::types.lt(31))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::first_name.ilike(&q))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
    }

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

    pub fn get_section(&self) -> SectionJson {
        return SectionJson {
            lists: self.get_communities_lists(Some(20), None),
            items: self.get_communities_of_list(Some(self.get_selected_list_id()), Some(20), None),
        };
    }
    pub fn get_communities_lists (
        &self,
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> Vec<CardCommunitiesList> {
        use crate::schema::communities_lists::dsl::communities_lists;
  
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return communities_lists
            .filter(schema::communities_lists::user_id.eq(self.id))
            .order(schema::communities_lists::position.desc())
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::communities_lists::id,
                schema::communities_lists::name,
                schema::communities_lists::position,
                schema::communities_lists::count
            ))
            .load::<CardCommunitiesList>(&_connection)
            .expect("E.");
    }

    pub fn get_communities_of_list (
        &self,
        list_id: Option<i32>, 
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> Vec<CardCommunityJson> {
        use crate::schema::{
            community_list_items::dsl::community_list_items,
            communitys::dsl::communitys,
        };
        let current_list_id: i32;
        if list_id.is_some() {
            current_list_id = list_id.unwrap();
        }
        else {
            current_list_id = self.get_selected_list_id();
        }
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let communities_ids = community_list_items
            .filter(schema::community_list_items::list_id.eq(current_list_id))
            .order(schema::community_list_items::visited.desc())
            .select(schema::community_list_items::community_id)
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");
        return communitys
            .filter(schema::communitys::id.eq_any(communities_ids))
            .filter(schema::communitys::types.lt(20))
            .select((
                schema::communitys::id,
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

        let _limit = get_limit(limit, 6);
        let _connection = establish_connection();
        let communities_ids = communities_memberships
            .filter(schema::communities_memberships::user_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .select(schema::communities_memberships::community_id)
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
        field:     &str, 
        value:     i16, 
        items_ids: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 130 || (is_ie_mode && items_ids.is_none()) {
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
            "invite" => diesel::update(self)
                .set(schema::users::invite.eq(value))
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

    pub fn count_lists(&self) -> i16 {
        return self.lists;
    }
    pub fn count_lists_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_lists().into(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
        );
    }
    pub fn count_lists_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_lists().into(),
            " список".to_string(),
            " списка".to_string(),
            " списков".to_string(),
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
    pub fn plus_lists(&self, count: i16) -> bool {
        let _connection = establish_connection();
        let _u = diesel::update(self)
            .set(schema::users::lists.eq(self.lists + count))
            .execute(&_connection);
        if _u.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn minus_lists(&self, count: i16) -> bool {
        if self.communities > 0 {
            let _connection = establish_connection();
            let _u = diesel::update(self)
                .set(schema::users::lists.eq(self.lists - count))
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

    pub fn edit_password(&self, password: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::users::password.eq(password))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }

    pub fn create_user (
        user_id:      i32,
        first_name:   String,
        last_name:    String,
        types:        i16,
        is_man:       bool,
        password:     String,
        link:         String,
        friends_list: i32,
        follows_list: i32,
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
            user_id:       user_id,
            first_name:    first_name.clone(),
            last_name:     last_name.clone(),
            types:         types,
            is_man:        is_man,
            password:      password.clone(),
            link:          link.clone(),
            s_avatar:      None,
            last_activity: chrono::Local::now().naive_utc(),
            see_all:       1,
            see_community: 1,
            invite:        1,
            lists:         0,
            communities:   0,
        };
        let _new_user = diesel::insert_into(schema::users::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("E.");

        CommunitiesList::create_list("Сообщества".to_string(), user_id, 1, None);
        FollowsList::create_list(follows_list, user_id);
        FriendsList::create_list(friends_list, user_id);

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
            .filter(schema::user_visible_perms::item_id.eq(user_id))
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
            .filter(schema::user_visible_perms::item_id.eq(user_id))
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
            .filter(schema::user_visible_perms::item_id.eq_any(self.get_friends_ids()))
            .filter(schema::user_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::item_id)
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
            .filter(schema::user_visible_perms::item_id.eq_any(self.get_follows_ids()))
            .filter(schema::user_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::user_visible_perms::item_id)
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
        return self.get_ie_follows_for_types(10, limit, offset); 
    }
    pub fn get_limit_see_all_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(0, limit, offset); 
    }
    pub fn get_limit_see_all_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(10, limit, offset); 
    }
    pub fn get_limit_see_all_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(0, limit, offset); 
    }

    pub fn get_limit_see_community_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_community_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(1, limit, offset); 
    }
    pub fn get_limit_see_community_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_community_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(1, limit, offset); 
    }

    pub fn get_limit_invite_exclude_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(12, limit, offset); 
    }
    pub fn get_limit_invite_include_follows(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_follows_for_types(2, limit, offset); 
    }
    pub fn get_limit_invite_exclude_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_friends_for_types(12, limit, offset); 
    }
    pub fn get_limit_invite_include_friends(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
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
    pub fn is_user_invite(&self, user_id: i32) -> bool {
        if self.user_id == user_id {
            return true;
        }
        return match self.see_all {
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
            1 => 31,
            2 => 32,
            3 => 33,
            4 => 34,
            5 => 35,
            6 => 36,
            _ => 31,
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
            31 => 1,
            32 => 2,
            33 => 2,
            34 => 4,
            35 => 5,
            36 => 6,
            _ => 1,
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
            1 => 41,
            2 => 42,
            3 => 43,
            4 => 44,
            5 => 45,
            6 => 46,
            _ => 41,
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
            41 => 1,
            42 => 2,
            43 => 2,
            44 => 4,
            45 => 5,
            46 => 6,
            _ => 1,
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

    pub fn suspend_item(&self) -> i16 {
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
    pub fn unsuspend_item(&self) -> i16 {
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
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(user_id))
            .filter(schema::user_visible_perms::item_id.eq(self.user_id))
            .filter(schema::user_visible_perms::types.eq(20))
            .select(schema::user_visible_perms::id)
            .first::<i32>(&_connection).is_ok();
    }
    pub fn is_self_user_in_block(&self, user_id: i32) -> bool {
        use crate::schema::user_visible_perms::dsl::user_visible_perms;

        let _connection = establish_connection();
        return user_visible_perms
            .filter(schema::user_visible_perms::user_id.eq(self.user_id))
            .filter(schema::user_visible_perms::item_id.eq(user_id))
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

    pub fn get_follows_lists_ids(&self) -> Vec<i32> {
        use crate::schema::follows_lists::dsl::follows_lists;

        let _connection = establish_connection();
        return follows_lists
            .filter(schema::follows_lists::user_id.eq(self.user_id))
            .select(schema::follows_lists::list_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_main_follows_list(&self) -> FollowsList {
        use crate::schema::follows_lists::dsl::follows_lists;

        let _connection = establish_connection();
        return follows_lists
            .filter(schema::follows_lists::user_id.eq(self.user_id))
            .filter(schema::follows_lists::types.eq(0))
            .first::<FollowsList>(&_connection)
            .expect("E.");
    }
    pub fn get_follows_list(list_id: i32) -> FollowsList {
        use crate::schema::follows_lists::dsl::follows_lists;

        let _connection = establish_connection();
        return follows_lists
            .filter(schema::follows_lists::list_id.eq(list_id))
            .first::<FollowsList>(&_connection)
            .expect("E.");
    }
    pub fn follow_user(&self, list_id: i32, user_id: i32) -> i16 {
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

        let self_user_list = User::get_follows_list(list_id);
        self_user_list.create_follow_item(user_id);
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
        use crate::models::FollowsListItem;

        let _connection = establish_connection();
        FollowsListItem::delete_follows_items(self.get_follows_lists_ids(), user_id);

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

    pub fn get_friends_lists_ids(&self) -> Vec<i32> {
        use crate::schema::friends_lists::dsl::friends_lists;

        let _connection = establish_connection();
        return friends_lists
            .filter(schema::friends_lists::user_id.eq(self.user_id))
            .select(schema::friends_lists::list_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_main_friends_list(&self) -> FriendsList {
        use crate::schema::friends_lists::dsl::friends_lists;

        let _connection = establish_connection();
        return friends_lists
            .filter(schema::friends_lists::user_id.eq(self.user_id))
            .filter(schema::friends_lists::types.eq(0))
            .first::<FriendsList>(&_connection)
            .expect("E.");
    }
    pub fn get_friends_list(list_id: i32) -> FriendsList {
        use crate::schema::friends_lists::dsl::friends_lists;

        let _connection = establish_connection();
        return friends_lists
            .filter(schema::friends_lists::list_id.eq(list_id))
            .first::<FriendsList>(&_connection)
            .expect("E.");
    }
    pub fn frend_user(&self, list_id: i32, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_followers_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::follows::dsl::follows;
        use crate::models::FollowsListItem;

        let _connection = establish_connection();
        
        let _new_form = NewFriend {
            user_id:   self.user_id,
            target_id: user_id,
        };
        let _new_friend = diesel::insert_into(schema::friends::table)
            .values(&_new_form)
            .execute(&_connection);
        let _new_form = NewFriend {
            user_id:   user_id,
            target_id: self.user_id,
        };
        let _new_friend = diesel::insert_into(schema::friends::table)
            .values(&_new_form)
            .execute(&_connection);

        let target_user = get_user(user_id).expect("E.");
        FollowsListItem::delete_follows_items(target_user.get_follows_lists_ids(), self.user_id);
        
        let target_user_list = target_user.get_main_friends_list();
        let self_user_list = User::get_friends_list(list_id);
        target_user_list.create_friend_item(self.user_id);
        self_user_list.create_friend_item(user_id);

        let _del = diesel::delete (
            follows
                .filter(schema::follows::user_id.eq(user_id))
                .or_filter(schema::follows::target_id.eq(self.user_id))
                .filter(schema::follows::target_id.eq(user_id))
                .or_filter(schema::follows::user_id.eq(self.user_id))
            )
            .execute(&_connection);

        return 1;
        
    }
    pub fn unfrend_user(&self, user_id: i32) -> i16 {
        if self.user_id == user_id || !self.is_connected_with_user_with_id(user_id) {
            return 0;
        }
        use crate::schema::friends::dsl::friends;
        use crate::models::FriendsListItem;

        let _connection = establish_connection();

        let del = diesel::delete (
            friends
                .filter(schema::friends::user_id.eq(self.user_id))
                .or_filter(schema::friends::user_id.eq(user_id))
                .filter(schema::friends::target_id.eq(self.user_id))
                .or_filter(schema::friends::target_id.eq(user_id))      
            )
            .execute(&_connection);

        let _new_follow = NewFollow {
            user_id:   user_id,
            target_id: self.user_id,
        };
        let new_follow = diesel::insert_into(schema::follows::table)
            .values(&_new_follow)
            .execute(&_connection);

        let target_user = get_user(user_id).expect("E.");
        FriendsListItem::delete_friends_items(target_user.get_friends_lists_ids(), self.user_id);
        FriendsListItem::delete_friends_items(self.get_friends_lists_ids(), user_id);
        
        let self_user_list = self.get_main_follows_list();
        self_user_list.create_follow_item(user_id);

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
        let target_user = get_user(user_id).expect("E.");

        if self.is_connected_with_user_with_id(user_id) {
            use crate::schema::friends::dsl::friends;
            use crate::models::FriendsListItem;

            diesel::delete (
                friends
                    .filter(schema::friends::user_id.eq(self.user_id))
                    .or_filter(schema::friends::user_id.eq(user_id))
                    .filter(schema::friends::target_id.eq(self.user_id))
                    .or_filter(schema::friends::target_id.eq(user_id))      
                )
                .execute(&_connection)
                .expect("E");

            FriendsListItem::delete_friends_items(target_user.get_friends_lists_ids(), self.user_id);
            FriendsListItem::delete_friends_items(self.get_friends_lists_ids(), user_id);
        }
        else if self.is_followers_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            use crate::models::FollowsListItem;

            diesel::delete (
                follows
                    .filter(schema::follows::target_id.eq(self.user_id))
                    .filter(schema::follows::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
            FollowsListItem::delete_follows_items(self.get_follows_lists_ids(), user_id);
        }
        else if self.is_following_user_with_id(user_id) {
            use crate::schema::follows::dsl::follows;
            use crate::models::FollowsListItem;

            diesel::delete (
                follows
                    .filter(schema::follows::user_id.eq(self.user_id))
                    .filter(schema::follows::target_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");

            FollowsListItem::delete_follows_items(target_user.get_follows_lists_ids(), self.user_id);
        }

        let _user_block = NewUserVisiblePerm {
            user_id: self.user_id,
            item_id: user_id,
            types:   20,
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
                .filter(schema::user_visible_perms::item_id.eq(user_id))
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

    pub fn get_selected_list_id(&self) -> i32 {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        let list_id = communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .filter(schema::communities_lists::types.eq(0))
            .order(schema::communities_lists::position.desc())
            .select(schema::communities_lists::id)
            .first::<i32>(&_connection);
        
        if list_id.is_ok() {
            return list_id.expect("E.");
        }
        else {
            return self.get_main_communities_list().id;
        }
    }
    pub fn get_selected_list(&self) -> CommunitiesList {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        let list = communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .filter(schema::communities_lists::types.eq(0))
            .order(schema::communities_lists::position.desc())
            .first::<CommunitiesList>(&_connection);
        
        if list.is_ok() {
            return list.expect("E.");
        }
        else {
            return self.get_main_communities_list();
        }
    }

    pub fn is_have_friends_lists(&self) -> usize {
        // нужно ли показывать в приватности опцию выбора списков?
        // если созданных списков друзей нет, то и показывать опции необязательно
        use crate::schema::friends_lists::dsl::friends_lists;

        let _connection = establish_connection();
        return friends_lists
            .filter(schema::friends_lists::user_id.eq(self.user_id))
            .filter(schema::friends_lists::types.eq(5))
            .select(schema::friends_lists::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_have_follows_lists(&self) -> usize {
        use crate::schema::follows_lists::dsl::follows_lists;

        let _connection = establish_connection();
        return follows_lists
            .filter(schema::follows_lists::user_id.eq(self.user_id))
            .filter(schema::follows_lists::types.eq(5))
            .select(schema::follows_lists::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_have_communities_lists(&self) -> usize {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        return communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .filter(schema::communities_lists::types.eq(5))
            .select(schema::communities_lists::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_main_communities_list(&self) -> CommunitiesList {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        let list = communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .filter(schema::communities_lists::types.eq(0))
            .first::<CommunitiesList>(&_connection);
        
        if list.is_ok() {
            return list.expect("E.");
        }
        else {
            use crate::models::NewCommunitiesList;

            let new_list_f = NewCommunitiesList { 
                name:     "Сообщества".to_string(),
                user_id:  self.user_id,
                types:    0,
                position: 1,
                count:    0,
                repost:   0, 
                see_el:   1
            };
            let new_list = diesel::insert_into(schema::communities_lists::table)
                .values(&new_list_f)
                .get_result::<CommunitiesList>(&_connection)
                .expect("Error.");
            
            return new_list;
        }
    }

    pub fn follow_community(&self, community: Community) -> i16 {
        if self.is_follow_of_community(community.id) 
            || 
            !community.is_close() 
            || 
            self.is_member_of_community(community.id) 
            || 
            self.is_user_in_ban(community.id) {
            return 0;
        } 
        let _connection = establish_connection();

        let new_follow = NewCommunityFollow {
            user_id:      self.user_id,
            community_id: community.id,
            view:         false,
            visited:      1, 
        };
        diesel::insert_into(schema::community_follows::table)
            .values(&new_follow)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn unfollow_community(&self, community_id: i32) -> i16 {
        use crate::schema::community_follows::dsl::community_follows;

        if !self.is_follow_of_community(community_id) {
            return 0;
        } 
        let _connection = establish_connection();
        diesel::delete (
            community_follows
                .filter(schema::community_follows::user_id.eq(self.user_id))
                .filter(schema::community_follows::community_id.eq(community_id))
            )
          .execute(&_connection)
          .expect("E");
        
        return 1;
    }

    pub fn invite_in_community(&self, community_id: i32, users_ids: Vec<i32>) -> i16 {
        if !self.is_member_of_community(community_id) {
            return 0;
        } 
        let _connection = establish_connection();
        for user_id in users_ids.iter() {
            let user = get_user(*user_id).expect("Error.");
            if user.is_user_invite(self.user_id) && (
                !user.is_invite_of_community(community_id)
                || 
                !user.is_member_of_community(community_id)
                ||
                !user.is_user_in_ban(community_id)
                ||
                !user.is_follow_of_community(community_id)
            ) {
                let new_invite = NewCommunityInvite {
                    user_id:        *user_id,
                    community_id:   community_id,
                    invite_creator: self.user_id,
                };
                diesel::insert_into(schema::community_invites::table)
                    .values(&new_invite)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        return 1;
    }
    pub fn uninvite_community(&self, community_id: i32) -> i16 {
        use crate::schema::community_invites::dsl::community_invites;

        if !self.is_invite_of_community(community_id) {
            return 0;
        } 
        let _connection = establish_connection();
        diesel::delete (
            community_invites
                .filter(schema::community_invites::user_id.eq(self.user_id))
                .filter(schema::community_invites::community_id.eq(community_id))
            )
          .execute(&_connection)
          .expect("E");
        
        return 1;
    }

    pub fn join_community(&self, community: Community) -> i16 {
        use crate::models::{
            CommunitiesMembership,
        };

        if self.is_member_of_community(community.id) || self.is_user_in_ban(community.id) {
            return 0;
        } 
        let _connection = establish_connection();

        if community.is_private() {
            use crate::schema::community_invites::dsl::community_invites;

            if !self.is_invite_of_community(community.id) {
                    return 0;
            }
            diesel::delete (
                community_invites
                    .filter(schema::community_invites::user_id.eq(self.user_id))
                    .filter(schema::community_invites::community_id.eq(community.id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else if community.is_close() {
            use crate::schema::community_follows::dsl::community_follows;

            if !self.is_follow_of_community(community.id) {
                return 0;
            }
            diesel::delete (
                community_follows
                    .filter(schema::community_follows::user_id.eq(self.user_id))
                    .filter(schema::community_follows::community_id.eq(community.id))
                )
                .execute(&_connection)
                .expect("E");
        }

        let communities_list = self.get_main_communities_list();
        communities_list.create_community_item(community.id);
        
        let memberships_list = community.get_main_memberships_list();
        memberships_list.create_membership_item(self.id);
        
        CommunitiesMembership::create_membership(self.id, &community, 1);      
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
    pub fn is_follow_of_community(&self, community_id: i32) -> bool {
        use crate::schema::community_follows::dsl::community_follows;

        let _connection = establish_connection();
        return community_follows
            .filter(schema::community_follows::user_id.eq(self.user_id))
            .filter(schema::community_follows::community_id.eq(community_id))
            .select(schema::community_follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_invite_of_community(&self, community_id: i32) -> bool {
        use crate::schema::community_invites::dsl::community_invites;

        let _connection = establish_connection();
        return community_invites
            .filter(schema::community_invites::user_id.eq(self.user_id))
            .filter(schema::community_invites::community_id.eq(community_id))
            .select(schema::community_invites::id)
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

    pub fn get_communities_lists_obj(&self) -> Vec<CommunitiesList> {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        return communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .load::<CommunitiesList>(&_connection)
            .expect("E");
    }
    pub fn get_communities_lists_ids(&self) -> Vec<i32> {
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        return communities_lists
            .filter(schema::communities_lists::user_id.eq(self.user_id))
            .select(schema::communities_lists::id)
            .load::<i32>(&_connection)
            .expect("E");
    }

    pub fn leave_community(&self, community: &Community) -> i16 {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            community_list_items::dsl::community_list_items,
            memberships_list_items::dsl::memberships_list_items,
        };

        if !self.is_member_of_community(community.id) {
            return 0;
        }
        let _connection = establish_connection();
        diesel::delete ( 
            communities_memberships
                .filter(schema::communities_memberships::user_id.eq(self.user_id))
                .filter(schema::communities_memberships::community_id.eq(community.id))
            )
            .execute(&_connection)
            .expect("E");
        self.minus_communities(1);
        community.minus_members(1);

        for list in self.get_communities_lists_obj().iter() {
            diesel::delete (
                community_list_items
                    .filter(schema::community_list_items::list_id.eq(list.id))
                    .filter(schema::community_list_items::community_id.eq(community.id))
            )
            .execute(&_connection)
            .expect("E");

            diesel::update(list)
                .set(schema::communities_lists::count.eq(list.count - 1))
                .execute(&_connection)
                .expect("E.");
        }
        
        for list in community.get_memberships_lists_obj().iter() {
            diesel::delete (
                memberships_list_items
                    .filter(schema::memberships_list_items::list_id.eq(list.id))
                    .filter(schema::memberships_list_items::user_id.eq(self.id))
            )
            .execute(&_connection)
            .expect("E");
            diesel::update(list)
                .set(schema::memberships_lists::count.eq(list.count - 1))
                .execute(&_connection)
                .expect("E.");
        }

        return 1;
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
2 может приглашать в сообщества
10 не может видеть профиль
11 не может видеть сообщества
12 не может приглашать в сообщества
20 пользователь заблокирован у владельца блока сообществ

101 список может видеть сообщества
102 список может приглашать в сообщества
111 список не может видеть сообщества
112 список не может приглашать в сообщества
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserVisiblePerm {
    pub id:      i32,
    pub user_id: i32,
    pub item_id: i32,
    pub types:   i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_visible_perms"]
pub struct NewUserVisiblePerm {
    pub user_id: i32,
    pub item_id: i32,
    pub types:   i16,
}

/////// CommunityFollow //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityFollow {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_follows"]
pub struct NewCommunityFollow {
    pub user_id:      i32, 
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i16,
}

/////// CommunityInvite //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityInvite {
    pub id:             i32,
    pub user_id:        i32,
    pub community_id:   i32,
    pub invite_creator: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_invites"]
pub struct NewCommunityInvite {
    pub user_id:        i32,
    pub community_id:   i32,
    pub invite_creator: i32,
}