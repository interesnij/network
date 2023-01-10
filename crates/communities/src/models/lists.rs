use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema::{
    communities_lists, community_list_items, community_list_perms,
    memberships_lists, memberships_list_items, memberships_list_perms,
    friends_lists, friends_list_items, friends_list_perms,
    follows_lists, follows_list_items, follows_list_perms,
};
use crate::errors::Error;
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    EditListJson, CardCommunitiesList,
    RespListJson, CardUserJson, 
};

use crate::models::{
    User, Community,
};

/*
СПИСКИ СООБЩЕСТВ ПОЛЬЗОВАТЕЛЕЙ
Тип списка
0 основной список
5 пользовательский список
45 удаленный пользовательский список
80 закрытый основной список
85 закрытый пользовательский список
120 замороженный основной список
125 замороженный пользовательский список
165 полностью удаленный пользовательский список
190 полностью удаленный пользовательский список приватный

Приватность списка see_el
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

20 Все друзья и списки подписчиков, кроме
21 Все друзья и некоторые списки подписчиков
22 Все подписчики и списки друзей, кроме
23 Все подписчики и некоторые списки друзей
26 Списки друзей, кроме
27 Некоторые списки друзей
28 Списки подписчиков, кроме
29 Некоторые списки подписчиков
*/

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunitiesList {
    pub id:       i32,
    pub name:     String,
    pub user_id:  i32,
    pub types:    i16,
    pub position: i16,
    pub count:    i32,
    pub repost:   i32,
    pub see_el:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="communities_lists"]
pub struct NewCommunitiesList {
    pub name:     String,
    pub user_id:  i32,
    pub types:    i16,
    pub position: i16,
    pub count:    i32,
    pub repost:   i32,
    pub see_el:   i16,
}

impl CommunitiesList {
    pub fn create_community_item (
        &self, community_id: i32,
    ) -> i16 {
        use crate::schema::community_list_items::dsl::community_list_items;

        let _connection = establish_connection();

        if community_list_items
            .filter(schema::community_list_items::list_id.eq(self.id))
            .filter(schema::community_list_items::community_id.eq(community_id))
            .select(schema::community_list_items::id)
            .first::<i32>(&_connection)
            .is_err()
            {
            let new_item = NewCommunityListItem {
                list_id:      self.id,
                community_id: community_id,
                visited:      0,
            };
            diesel::insert_into(schema::community_list_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
                
            diesel::update(self)
                .set(schema::communities_lists::count.eq(self.count + 1))
                .execute(&_connection)
                .expect("E.");
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn create_community_items (
        &self, communities_ids: Vec<i32>,
    ) -> i16 { 
        use crate::schema::community_list_items::dsl::community_list_items;

        let _connection = establish_connection();
        let mut count = 0;
        for id in communities_ids.iter() {
            if community_list_items
                .filter(schema::community_list_items::list_id.eq(self.id))
                .filter(schema::community_list_items::community_id.eq(*id))
                .select(schema::community_list_items::id)
                .first::<i32>(&_connection)
                .is_err()
                {
                let new_item = NewCommunityListItem {
                    list_id:      self.id,
                    community_id: *id,
                    visited:      0,
                };
                diesel::insert_into(schema::community_list_items::table)
                    .values(&new_item)
                    .execute(&_connection)
                    .expect("Error.");

                count += 1;
            }
        }
        
        diesel::update(self)
            .set(schema::communities_lists::count.eq(self.count + count))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _items: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 112 || (is_ie_mode && _items.is_none()) {
            return 0;
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_el" => diesel::update(self)
                .set(schema::communities_lists::see_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
            };

        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::community_list_perms::dsl::community_list_perms;
            match value { 
                1 => diesel::delete (
                    community_list_perms
                        .filter(schema::community_list_perms::list_id.eq(self.id))
                        .filter(schema::community_list_perms::types.eq(11))
                    )
                    .execute(&_connection)
                    .expect("E"),
                11 => diesel::delete (
                    community_list_perms
                        .filter(schema::community_list_perms::list_id.eq(self.id))
                        .filter(schema::community_list_perms::types.eq(1))
                    )
                    .execute(&_connection)
                    .expect("E"),
                101 => diesel::delete (
                    community_list_perms
                        .filter(schema::community_list_perms::list_id.eq(self.id))
                        .filter(schema::community_list_perms::types.eq(111))
                    )
                    .execute(&_connection)
                    .expect("E"),
                111 => diesel::delete (
                    community_list_perms
                        .filter(schema::community_list_perms::list_id.eq(self.id))
                        .filter(schema::community_list_perms::types.eq(101))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if _items.is_some() && is_ie_mode {
            for item_id in _items.unwrap().iter() {
                let _new_perm = NewCommunityListPerm {
                    item_id: *item_id,
                    list_id: self.id,
                    types:   value,
                };
                diesel::insert_into(schema::community_list_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }

    pub fn get_creator(&self) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::user_id.eq(self.user_id))
            .first::<User>(&_connection)?);
    }
    pub fn get_edit_list_json(&self) -> Result<EditListJson, Error> {
        return Ok(EditListJson {
            id:                   self.id,
            name:                 self.name.clone(),
            position:             self.position,
            see_el_exclude_users: self.get_limit_see_el_exclude_users(Some(20), Some(0)),
            see_el_include_users: self.get_limit_see_el_include_users(Some(20), Some(0)),
        }); 
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_community_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lco".to_string() + &self.get_str_id();
    }

    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }

    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn get_lists_for_attach(ids: Vec<i32>) -> Vec<CardCommunitiesList> {
        // выдача инфы для прикрепления списков сообществ
        // по запросу API
        use crate::schema::communities_lists::dsl::communities_lists;

        let _connection = establish_connection();
        return communities_lists
            .filter(schema::communities_lists::id.eq_any(ids))
            .filter(schema::communities_lists::types.lt(31))
            .select((
                schema::communities_lists::id,
                schema::communities_lists::name,
                schema::communities_lists::position,
                schema::communities_lists::count,
            ))
            .load::<CardCommunitiesList>(&_connection)
            .expect("E.");
    }

    pub fn get_items(&self) -> Vec<Community> {
        use crate::schema::{
            community_list_items::dsl::community_list_items,
            communitys::dsl::communitys,
        };

        let _connection = establish_connection();
        let ids = community_list_items
            .filter(schema::community_list_items::list_id.eq(self.id))
            .select(schema::community_list_items::community_id)
            .order(schema::community_list_items::visited.desc())
            .load::<i32>(&_connection)
            .expect("E.");

        return communitys
            .filter(schema::communitys::id.eq_any(ids))
            .filter(schema::communitys::types.lt(31))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn get_items_ids(&self) -> Vec<i32> {
        use crate::schema::community_list_items::dsl::community_list_items;

        let _connection = establish_connection();
        return community_list_items
            .filter(schema::community_list_items::list_id.eq(self.id))
            .select(schema::community_list_items::community_id)
            .load::<i32>(&_connection)
            .expect("E.");
    }

    pub fn get_paginate_items (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>,
    ) -> Vec<Community> {
        use crate::schema::{
            community_list_items::dsl::community_list_items,
            communitys::dsl::communitys,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let ids = community_list_items
            .filter(schema::community_list_items::list_id.eq(self.id))
            .select(schema::community_list_items::community_id)
            .order(schema::community_list_items::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");

        return communitys
            .filter(schema::communitys::id.eq_any(ids))
            .filter(schema::communitys::types.lt(31))
            .load::<Community>(&_connection)
            .expect("E.");
    }
    pub fn count_items(&self) -> String {
        if self.count > 0 {
            return self.count.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn count_items_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " сообщество".to_string(),
            " сообщества".to_string(),
            " сообществ".to_string(),
        );
    }
    pub fn is_friend_perm_exists (
        &self,
        item_id: i32,
        types:   i16, 
    ) -> bool {
        // проверяем, если ли пользователь в вкл/искл списках пользователя 
        // и дружит ли он с self
        use crate::schema::{
            community_list_perms::dsl::community_list_perms,
            friends::dsl::friends,
        };

        let _connection = establish_connection();
        return community_list_perms
            .filter(schema::community_list_perms::item_id.eq(item_id))
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .select(schema::community_list_perms::item_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        friends 
            .filter(schema::friends::target_id.eq(self.user_id))
            .filter(schema::friends::user_id.eq(item_id))
            .select(schema::friends::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_follow_perm_exists (
        &self,
        item_id: i32,
        types:   i16, 
    ) -> bool {
        // проверяем, если ли пользователь в вкл/искл списках пользователя 
        // и подписан ли он на self
        use crate::schema::{
            community_list_perms::dsl::community_list_perms,
            follows::dsl::follows,
        };

        let _connection = establish_connection();
        return community_list_perms
            .filter(schema::community_list_perms::item_id.eq(item_id))
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .select(schema::community_list_perms::item_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(item_id))
            .select(schema::follows::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_ie_users_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            community_list_perms::dsl::community_list_perms,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = community_list_perms
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::community_list_perms::item_id)
            .load::<i32>(&_connection) 
            .expect("E");

        return users
            .filter(schema::users::user_id.eq_any(items_ids))
            .filter(schema::users::types.lt(31))
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

    pub fn get_limit_see_el_exclude_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_users_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_el_include_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_users_for_types(1, limit, offset); 
    }

    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        let private_field = self.see_el;
        if self.user_id == user_id || self.see_el == 1 {
            return true;
        }
        
        let creator = self.get_creator().expect("E");
        return match private_field {
            1 => true,
            2 => creator.is_connected_with_user_with_id(user_id) || creator.is_self_followers_user_with_id(user_id),
            3 => creator.is_connected_with_user_with_id(user_id) || !self.is_follow_perm_exists(user_id, 11),
            4 => creator.is_connected_with_user_with_id(user_id) || self.is_follow_perm_exists(user_id, 1),
            5 => creator.is_self_followers_user_with_id(user_id) || !self.is_friend_perm_exists(user_id, 11),
            6 => creator.is_self_followers_user_with_id(user_id) || self.is_friend_perm_exists(user_id, 1),
            7 => creator.is_connected_with_user_with_id(user_id),
            8 => creator.is_self_followers_user_with_id(user_id),
            9 => !self.is_friend_perm_exists(user_id, 11),
            10 => self.is_friend_perm_exists(user_id, 1),
            11 => !self.is_follow_perm_exists(user_id, 11),
            12 => self.is_follow_perm_exists(user_id, 1),
            _ => false,
        };
    }

    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1 || self.see_el == 14;
    }

    pub fn get_new_position_for_user(user_id: i32) -> i16 {
        use crate::utils::get_user;
        let _user = get_user(user_id).expect("E.");
        let count = _user.count_lists() + 1;
        _user.plus_lists(1);
        return count.try_into().unwrap();
    }

    pub fn create_list (
        name: String,
        user_id: i32,
        see_el: i16,
        see_el_users: Option<Vec<i32>>
    ) -> RespListJson {
        let _connection = establish_connection();
        let _name: String;
        let c_name = name.clone();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        let new_list = NewCommunitiesList {
            name:     _name.clone(),
            user_id:  user_id,
            types:    5,
            position: CommunitiesList::get_new_position_for_user(user_id),
            count:    0,
            repost:   0,
            see_el:   see_el,
        };
        let new_list = diesel::insert_into(schema::communities_lists::table)
            .values(&new_list)
            .get_result::<CommunitiesList>(&_connection)
            .expect("Error.");
        let exclude_vec = vec![3, 5, 9, 11];
        let include_vec = vec![4, 6, 10, 12];

        if exclude_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for item_id in see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewCommunityListPerm {
                        item_id: *item_id,
                        list_id: new_list.id,
                        types:   11,
                    };
                    diesel::insert_into(schema::community_list_perms::table)
                        .values(&_new_exclude)
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for item_id in see_el_users.as_deref().unwrap() {
                    let _new_include = NewCommunityListPerm {
                        item_id: *item_id,
                        list_id: new_list.id,
                        types:   1,
                    };
                    diesel::insert_into(schema::community_list_perms::table)
                        .values(&_new_include)
                        .execute(&_connection)
                        .expect("Error.");
                }   
            }
        }

        return RespListJson {
            id:   new_list.id,
            name: _name,
        };
    }
    pub fn edit_list (
        &self, 
        name:     String, 
        see_el:   i16,
        position: i16,
        _users:   Option<Vec<i32>>
    ) -> RespListJson {
        use crate::schema::community_list_perms::dsl::community_list_perms;

        let _connection = establish_connection();
        let _name: String;
        let c_name = name.clone();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        diesel::update(self)
            .set((
                schema::communities_lists::name.eq(_name.clone()),
                schema::communities_lists::see_el.eq(see_el),
                schema::communities_lists::position.eq(position),
            ))
            .execute(&_connection)
            .expect("Error.");

        let exclude_vec = vec![3, 5, 9, 11];
        let include_vec = vec![4, 6, 10, 12];

        diesel::delete (
            community_list_perms
                .filter(schema::community_list_perms::list_id.eq(self.id))
                .filter(schema::community_list_perms::types.ne(20))
        )
        .execute(&_connection)
        .expect("E");

        if exclude_vec.iter().any(|&i| i==see_el) {
            if _users.is_some() {
                for item_id in _users.as_deref().unwrap() {
                    let _new_exclude = NewCommunityListPerm {
                        item_id: *item_id,
                        list_id: self.id,
                        types:   11,
                    };
                    diesel::insert_into(schema::community_list_perms::table)
                        .values(&_new_exclude)
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el) {
            if _users.is_some() {
                for item_id in _users.as_deref().unwrap() {
                    let _new_include = NewCommunityListPerm {
                        item_id: *item_id,
                        list_id: self.id,
                        types:   1,
                    };
                    diesel::insert_into(schema::community_list_perms::table)
                        .values(&_new_include)
                        .execute(&_connection)
                        .expect("Error.");
                }   
            }
        }

        return RespListJson {
            id:   self.id,
            name: _name,
        };
    }

    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types + 80))
            .execute(&_connection);
        let creator = self.get_creator().expect("E");
        creator.minus_lists(1);
        
        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types - 80))
            .execute(&_connection);

        let creator = self.get_creator().expect("E");
        creator.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();

        if self.types == 0 {
            return 0;
        }

        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types + 40))
            .execute(&_connection);

        let creator = self.get_creator().expect("E");
        creator.minus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();

        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types - 40))
            .execute(&_connection);

        let creator = self.get_creator().expect("E");
        creator.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn suspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types + 120))
            .execute(&_connection);

        let creator = self.get_creator().expect("E");
        creator.minus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unsuspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::communities_lists::types.eq(self.types - 120))
            .execute(&_connection);

        let creator = self.get_creator().expect("E");
        creator.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
}


#[derive(Debug, Serialize, Identifiable)]
pub struct CommunityListItem {
    pub id:           i32,
    pub list_id:      i32,
    pub community_id: i32,
    pub visited:      i32,
} 
#[derive(Deserialize, Insertable)]
#[table_name="community_list_items"]
pub struct NewCommunityListItem { 
    pub list_id:      i32,
    pub community_id: i32,
    pub visited:      i32,
}

impl CommunityListItem {
    pub fn delete_community_item (
        list_id: i32, community_id: i32,
    ) -> i16 { 
        use crate::schema::community_list_items::dsl::community_list_items;
        use crate::utils::get_communities_list;

        let _connection = establish_connection();
        diesel::delete (
            community_list_items
                .filter(schema::community_list_items::list_id.eq(list_id))
                .filter(schema::community_list_items::community_id.eq(community_id))
        )
        .execute(&_connection)
        .expect("E.");

        let list = get_communities_list(list_id).expect("E.");
        diesel::update(&list)
            .set(schema::communities_lists::count.eq(self.count - 1))
            .execute(&_connection)
            .expect("E.");

        return 1;
    }
    pub fn delete_community_items (
        list_ids: Vec<i32>, community_id: i32,
    ) -> i16 { 
        let _connection = establish_connection();
        for i in list_ids.iter() {
            CommunityListItem::delete_community_item(i, community_id);
        }
        return 1;
    }
    pub fn plus_visited(&self) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::community_list_items::visited.eq(self.visited + 1))
            .execute(&_connection)
            .expect("E.");
    }
}


/*
CommunityListPerm
1 может видеть список
11 не может видеть список

101 список может видеть список
111 список не может видеть список
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityListPerm {
    pub id:      i32,
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_list_perms"]
pub struct NewCommunityListPerm {
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}

////////////////////////////////////////////
////////////////////////////////////////////
/*
СПИСКИ ПОДПИСЧИКОВ СООБЩЕСТВА
Тип списка
0 основной список
5 пользовательский список
45 удаленный пользовательский список
80 закрытый основной список
85 закрытый пользовательский список
120 замороженный основной список
125 замороженный пользовательский список
165 полностью удаленный пользовательский список
190 полностью удаленный пользовательский список приватный

Приватность списка see_el
1 Все пользователи
2 Подписчики
3 Персонал
4 Администраторы
5 Владелец сообщества
6 Подписчики, кроме
7 Некоторые подписчики
*/

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct MembershipsList {
    pub id:           i32,
    pub name:         String,
    pub community_id: i32,
    pub types:        i16,
    pub position:     i16,
    pub count:        i32,
    pub repost:       i32,
    pub see_el:       i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="memberships_lists"]
pub struct NewMembershipsList {
    pub name:         String,
    pub community_id: i32,
    pub types:        i16,
    pub position:     i16,
    pub count:        i32,
    pub repost:       i32,
    pub see_el:       i16,
}

impl MembershipsList {
    pub fn create_membership_item (
        &self, user_id: i32,
    ) -> i16 {
        let _connection = establish_connection();
        let new_item = NewMembershipsListItem {
            list_id:      self.id,
            user_id: user_id,
            visited:      0,
        };
        diesel::insert_into(schema::memberships_list_items::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");
        
        diesel::update(self)
            .set(schema::memberships_lists::count.eq(self.count + 1))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn create_membership_items (
        &self, user_ids: Vec<i32>,
    ) -> i16 {
        let _connection = establish_connection();
        let mut count = 0;
        for id in user_ids.iter() {
            let new_item = NewMembershipsListItem {
                list_id: self.id,
                user_id: *id,
                visited: 0,
            };
            diesel::insert_into(schema::memberships_list_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
            
            count += 1;
        }
        
        diesel::update(self)
            .set(schema::memberships_lists::count.eq(self.count + count))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _items: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![6,7].iter().any(|&i| i==value);
        if value < 1 || value > 120 || (is_ie_mode && _items.is_none()) {
            return 0;
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_el" => diesel::update(self)
                .set(schema::memberships_lists::see_el.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
            };

        if is_ie_mode {
            // нужно удалить из списка тех, кто был туда внесен
            // с противоположными правами.
            use crate::schema::memberships_list_perms::dsl::memberships_list_perms;
            match value { 
                1 => diesel::delete (
                    memberships_list_perms
                        .filter(schema::memberships_list_perms::list_id.eq(self.id))
                        .filter(schema::memberships_list_perms::types.eq(11))
                    )
                    .execute(&_connection)
                    .expect("E"),
                11 => diesel::delete (
                    memberships_list_perms
                        .filter(schema::memberships_list_perms::list_id.eq(self.id))
                        .filter(schema::memberships_list_perms::types.eq(1))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        };
        if _items.is_some() && is_ie_mode {
            for item_id in _items.unwrap().iter() {
                let _new_perm = NewMembershipsListPerm {
                    item_id: *item_id,
                    list_id: self.id,
                    types:   value,
                };
                diesel::insert_into(schema::memberships_list_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }

    pub fn get_community(&self) -> Result<Community, Error> {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return Ok(communitys
            .filter(schema::communitys::id.eq(self.id))
            .first::<Community>(&_connection)?);
    }
    pub fn get_edit_list_json(&self) -> Result<EditListJson, Error> {
        return Ok(EditListJson {
            id:                   self.id,
            name:                 self.name.clone(),
            position:             self.position,
            see_el_exclude_users: self.get_limit_see_el_exclude_users(Some(20), Some(0)),
            see_el_include_users: self.get_limit_see_el_include_users(Some(20), Some(0)),
        }); 
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_memberships_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lms".to_string() + &self.get_str_id();
    }

    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }

    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn get_lists_for_attach(ids: Vec<i32>) -> Vec<CardCommunitiesList> {
        // выдача инфы для прикрепления списков сообществ
        // по запросу API
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        return memberships_lists
            .filter(schema::memberships_lists::id.eq_any(ids))
            .filter(schema::memberships_lists::types.lt(31))
            .select((
                schema::memberships_lists::id,
                schema::memberships_lists::name,
                schema::memberships_lists::position,
                schema::memberships_lists::count,
            ))
            .load::<CardCommunitiesList>(&_connection)
            .expect("E.");
    }

    pub fn get_items(&self) -> Vec<User> {
        use crate::schema::{
            memberships_list_items::dsl::memberships_list_items,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let ids = memberships_list_items
            .filter(schema::memberships_list_items::list_id.eq(self.id))
            .select(schema::memberships_list_items::user_id)
            .order(schema::memberships_list_items::visited.desc())
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_items_ids(&self) -> Vec<i32> {
        use crate::schema::memberships_list_items::dsl::memberships_list_items;

        let _connection = establish_connection();
        return memberships_list_items
            .filter(schema::memberships_list_items::list_id.eq(self.id))
            .select(schema::memberships_list_items::user_id)
            .order(schema::memberships_list_items::visited.desc())
            .load::<i32>(&_connection)
            .expect("E.");
    }

    pub fn get_paginate_items (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>,
    ) -> Vec<User> {
        use crate::schema::{
            memberships_list_items::dsl::memberships_list_items,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let ids = memberships_list_items
            .filter(schema::memberships_list_items::list_id.eq(self.id))
            .select(schema::memberships_list_items::user_id)
            .order(schema::memberships_list_items::visited.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::user_id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn count_items(&self) -> String {
        if self.count > 0 {
            return self.count.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn count_items_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " подписчик".to_string(),
            " подписчик".to_string(),
            " подписчик".to_string(),
        );
    }
    pub fn is_members_perm_exists (
        &self,
        user_id: i32,
        //types:   i16, 
    ) -> bool {
        // проверяем, если ли пользователь в вкл/искл списках пользователя 
        // и дружит ли он с self
        use crate::schema::{
            memberships_list_items::dsl::memberships_list_items,
            communities_memberships::dsl::communities_memberships,
        };

        let _connection = establish_connection();
        return memberships_list_items 
            .filter(schema::memberships_list_items::user_id.eq(user_id))
            .filter(schema::memberships_list_items::list_id.eq(self.id))
            .select(schema::memberships_list_items::user_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        communities_memberships 
            .filter(schema::communities_memberships::community_id.eq(self.community_id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn get_ie_users_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            memberships_list_perms::dsl::memberships_list_perms,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = memberships_list_perms
            .filter(schema::memberships_list_perms::list_id.eq(self.id))
            .filter(schema::memberships_list_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::memberships_list_perms::item_id)
            .load::<i32>(&_connection) 
            .expect("E");

        return users
            .filter(schema::users::user_id.eq_any(items_ids))
            .filter(schema::users::types.lt(31))
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

    pub fn get_limit_see_el_exclude_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_users_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_el_include_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_users_for_types(1, limit, offset); 
    }

    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        let community = self.get_community().expect("E");
        return match self.see_el {
            1 => true,
            2 => community.is_user_member(user_id),
            3 => community.is_user_staff(user_id),
            4 => community.is_user_admin(user_id),
            5 => community.user_id == user_id,
            6 => !community.is_user_perm_exists(user_id, 16),
            7 => community.is_user_perm_exists(user_id, 6),
            _ => false
        };
    }

    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1;
    }

    pub fn get_new_position_for_community(community_id: i32) -> i16 {
        use crate::utils::get_community;
        let community = get_community(community_id).expect("E.");
        let count = community.count_lists() + 1;
        community.plus_lists(1);
        return count.try_into().unwrap();
    }

    pub fn create_list (
        name:         String,
        community_id: i32,
        see_el:       i16,
        see_el_users: Option<Vec<i32>>
    ) -> RespListJson {
        let _connection = establish_connection();
        let _name: String;
        let c_name = name.clone();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        let new_list = NewMembershipsList {
            name:         _name.clone(),
            community_id: community_id,
            types:        5,
            position:     MembershipsList::get_new_position_for_community(community_id),
            count:        0,
            repost:       0,
            see_el:       see_el,
        };
        let new_list = diesel::insert_into(schema::memberships_lists::table)
            .values(&new_list)
            .get_result::<MembershipsList>(&_connection)
            .expect("Error.");
        let exclude_vec = vec![6];
        let include_vec = vec![7];

        if exclude_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for item_id in see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewMembershipsListPerm {
                        item_id: *item_id,
                        list_id: new_list.id,
                        types:   11,
                    };
                    diesel::insert_into(schema::memberships_list_perms::table)
                        .values(&_new_exclude)
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for item_id in see_el_users.as_deref().unwrap() {
                    let _new_include = NewMembershipsListPerm {
                        item_id: *item_id,
                        list_id: new_list.id,
                        types:   1, 
                    };
                    diesel::insert_into(schema::memberships_list_perms::table)
                        .values(&_new_include)
                        .execute(&_connection)
                        .expect("Error.");
                }   
            }
        }

        return RespListJson {
            id:   new_list.id,
            name: _name,
        };
    }
    pub fn edit_list (
        &self,
        name:     String,
        see_el:   i16,
        position: i16,
        _users:   Option<Vec<i32>>
    ) -> RespListJson {
        use crate::schema::memberships_list_perms::dsl::memberships_list_perms;

        let _connection = establish_connection();
        let _name: String;
        let c_name = name.clone();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        diesel::update(self)
            .set((
                schema::memberships_lists::name.eq(_name.clone()),
                schema::memberships_lists::position.eq(position),
                schema::memberships_lists::see_el.eq(see_el),
            ))
            .execute(&_connection)
            .expect("Error.");

        let exclude_vec = vec![6];
        let include_vec = vec![7];

        diesel::delete (
            memberships_list_perms
                .filter(schema::memberships_list_perms::list_id.eq(self.id))
                .filter(schema::memberships_list_perms::types.ne(20))
        )
        .execute(&_connection)
        .expect("E");

        if exclude_vec.iter().any(|&i| i==self.see_el) {
            if _users.is_some() {
                for item_id in _users.as_deref().unwrap() {
                    let _new_exclude = NewMembershipsListPerm {
                        item_id: *item_id,
                        list_id: self.id,
                        types:   11,
                    };
                    diesel::insert_into(schema::memberships_list_perms::table)
                        .values(&_new_exclude)
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==self.see_el) {
            if _users.is_some() {
                for item_id in _users.as_deref().unwrap() {
                    let _new_include = NewMembershipsListPerm {
                        item_id: *item_id,
                        list_id: self.id,
                        types:   1,
                    };
                    diesel::insert_into(schema::memberships_list_perms::table)
                        .values(&_new_include)
                        .execute(&_connection)
                        .expect("Error.");
                }   
            }
        }

        return RespListJson {
            id:   self.id,
            name: _name,
        };
    }

    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types + 80))
            .execute(&_connection);
        let community = self.get_community().expect("E");
        community.minus_lists(1);
        
        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types - 80))
            .execute(&_connection);

        let community = self.get_community().expect("E");
        community.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        if self.types == 0 {
            return 0;
        }

        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types + 40))
            .execute(&_connection);

        let community = self.get_community().expect("E");
        community.minus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();

        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types - 40))
            .execute(&_connection);

        let community = self.get_community().expect("E");
        community.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn suspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types + 120))
            .execute(&_connection);

        let community = self.get_community().expect("E");
        community.minus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unsuspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::memberships_lists::types.eq(self.types - 120))
            .execute(&_connection);

        let community = self.get_community().expect("E");
        community.plus_lists(1);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
}


#[derive(Debug, Serialize, Identifiable)]
pub struct MembershipsListItem {
    pub id:      i32,
    pub list_id: i32,
    pub user_id: i32,
    pub visited: i32,
} 
#[derive(Deserialize, Insertable)]
#[table_name="memberships_list_items"]
pub struct NewMembershipsListItem { 
    pub list_id: i32,
    pub user_id: i32,
    pub visited: i32,
}

impl MembershipsListItem {
    pub fn delete_memberships_items (
        list_ids: Vec<i32>, user_id: i32,
    ) -> i16 { 
        use crate::schema::memberships_list_items::dsl::memberships_list_items;

        let _connection = establish_connection();
        diesel::delete (
            memberships_list_items
                .filter(schema::memberships_list_items::list_id.eq_any(list_ids))
                .filter(schema::memberships_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
    pub fn delete_memberships_item (
        list_id: i32, user_id: i32,
    ) -> i16 { 
        use crate::schema::memberships_list_items::dsl::memberships_list_items;

        let _connection = establish_connection();
        diesel::delete (
            memberships_list_items
                .filter(schema::memberships_list_items::list_id.eq(list_id))
                .filter(schema::memberships_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
    pub fn plus_visited(&self) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::memberships_list_items::visited.eq(self.visited + 1))
            .execute(&_connection)
            .expect("E.");
    }
}


/*
MembershipsListPerm
1 может видеть список
11 не может видеть список

101 список может видеть список
111 список не может видеть список
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct MembershipsListPerm {
    pub id:      i32,
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="memberships_list_perms"]
pub struct NewMembershipsListPerm {
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}


/////////////////////////////////////////
/////////////////////////////////////////

/*
СПИСКИ ДРУЗЕЙ ПОЛЬЗОВАТЕЛЕЙ
Тип списка
0 основной список
5 пользовательский список
45 удаленный пользовательский список
80 закрытый основной список
85 закрытый пользовательский список
120 замороженный основной список
125 замороженный пользовательский список
165 полностью удаленный пользовательский список
190 полностью удаленный пользовательский список приватный

Приватность списка see_el
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

20 Все друзья и списки подписчиков, кроме
21 Все друзья и некоторые списки подписчиков
22 Все подписчики и списки друзей, кроме
23 Все подписчики и некоторые списки друзей
26 Списки друзей, кроме
27 Некоторые списки друзей
28 Списки подписчиков, кроме
29 Некоторые списки подписчиков
*/

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct FriendsList {
    pub id:      i32,
    pub list_id: i32,
    pub user_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends_lists"]
pub struct NewFriendsList {
    pub list_id: i32,
    pub user_id: i32,
    pub types:   i16,
}

impl FriendsList {
    pub fn create_friend_items (
        &self, user_ids: Vec<i32>,
    ) -> i16 {
        let _connection = establish_connection();
        for id in user_ids.iter() {
            let new_item = NewFriendsListItem {
                list_id: self.id,
                user_id: *id,
            };
            diesel::insert_into(schema::friends_list_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
            
        }
        return 1;
    }
    pub fn create_friend_item (
        &self, user_id: i32,
    ) -> i16 {
        let _connection = establish_connection();
        let new_item = NewFriendsListItem {
            list_id: self.list_id,
            user_id: user_id,
        };
        diesel::insert_into(schema::friends_list_items::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    
    pub fn get_creator(&self) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::user_id.eq(self.user_id))
            .first::<User>(&_connection)?);
    }

    pub fn get_items(&self) -> Vec<User> {
        use crate::schema::{
            friends_list_items::dsl::friends_list_items,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let ids = friends_list_items
            .filter(schema::friends_list_items::list_id.eq(self.list_id))
            .select(schema::friends_list_items::user_id)
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_items_ids(&self) -> Vec<i32> {
        use crate::schema::friends_list_items::dsl::friends_list_items;

        let _connection = establish_connection();
        return friends_list_items
            .filter(schema::friends_list_items::list_id.eq(self.list_id))
            .select(schema::friends_list_items::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
    }

    pub fn get_paginate_items (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>,
    ) -> Vec<User> {
        use crate::schema::{
            friends_list_items::dsl::friends_list_items,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let ids = friends_list_items
            .filter(schema::friends_list_items::list_id.eq(self.list_id))
            .select(schema::friends_list_items::user_id)
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }

    pub fn create_list(list_id: i32, user_id: i32) -> i16 {
        let _connection = establish_connection();

        let new_list = NewFriendsList {
            list_id: list_id,
            user_id: user_id,
            types:   5,
        };
        let _new_list = diesel::insert_into(schema::friends_lists::table)
            .values(&new_list)
            .get_result::<FriendsList>(&_connection)
            .expect("Error.");

        return 1;
    }

    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types + 80))
            .execute(&_connection);
        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types - 80))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        if self.types == 0 {
            return 0;
        }

        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types + 40))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();

        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types - 40))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn suspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types + 120))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unsuspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::friends_lists::types.eq(self.types - 120))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
}


#[derive(Debug, Serialize, Identifiable)]
pub struct FriendsListItem {
    pub id:      i32,
    pub list_id: i32,
    pub user_id: i32,
} 
#[derive(Deserialize, Insertable)]
#[table_name="friends_list_items"]
pub struct NewFriendsListItem { 
    pub list_id: i32,
    pub user_id: i32,
}

impl FriendsListItem {
    pub fn delete_friends_item (
        list_id: i32, user_id: i32,
    ) -> i16 { 
        use crate::schema::friends_list_items::dsl::friends_list_items;

        let _connection = establish_connection();
        diesel::delete (
            friends_list_items
                .filter(schema::friends_list_items::list_id.eq(list_id))
                .filter(schema::friends_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
    pub fn delete_friends_items (
        list_ids: Vec<i32>, user_id: i32,
    ) -> i16 { 
        use crate::schema::friends_list_items::dsl::friends_list_items;

        let _connection = establish_connection();
        diesel::delete (
            friends_list_items
                .filter(schema::friends_list_items::list_id.eq_any(list_ids))
                .filter(schema::friends_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
}


/*
FriendsListPerm
1 пользователь может видеть список 
11 пользователь не может видеть список
101 список может видеть список 
111 список не может видеть список
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct FriendsListPerm {
    pub id:      i32,
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends_list_perms"]
pub struct NewFriendsListPerm {
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}


/////////////////////////////////////////
/////////////////////////////////////////

/*
СПИСКИ ПОДПИСЧИКОВ ПОЛЬЗОВАТЕЛЕЙ
Тип списка
0 основной список
5 пользовательский список
45 удаленный пользовательский список
80 закрытый основной список
85 закрытый пользовательский список
120 замороженный основной список
125 замороженный пользовательский список
165 полностью удаленный пользовательский список
190 полностью удаленный пользовательский список приватный

Приватность списка see_el
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

Поле list_id нужно для синхронизации списка с его оригиналом на
сервисе пользователей. Чтобы обращаться к нему правильно.
*/

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct FollowsList {
    pub id:      i32,
    pub list_id: i32,
    pub user_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows_lists"]
pub struct NewFollowsList {
    pub list_id: i32,
    pub user_id: i32,
    pub types:   i16,
}

impl FollowsList {
    pub fn create_follow_items (
        &self, user_ids: Vec<i32>,
    ) -> i16 {
        let _connection = establish_connection();
        for id in user_ids.iter() {
            let new_item = NewFollowsListItem {
                list_id: self.id,
                user_id: *id,
            };
            diesel::insert_into(schema::follows_list_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn create_follow_item (
        &self, user_id: i32,
    ) -> i16 {
        let _connection = establish_connection();
        let new_item = NewFollowsListItem {
            list_id: self.list_id,
            user_id: user_id,
        };
        diesel::insert_into(schema::follows_list_items::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    
    pub fn get_creator(&self) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::user_id.eq(self.user_id))
            .first::<User>(&_connection)?);
    }

    pub fn get_items(&self) -> Vec<User> {
        use crate::schema::{
            follows_list_items::dsl::follows_list_items,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let ids = follows_list_items
            .filter(schema::follows_list_items::list_id.eq(self.list_id))
            .select(schema::follows_list_items::user_id)
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }
    pub fn get_items_ids(&self) -> Vec<i32> {
        use crate::schema::follows_list_items::dsl::follows_list_items;

        let _connection = establish_connection();
        return follows_list_items
            .filter(schema::follows_list_items::list_id.eq(self.list_id))
            .select(schema::follows_list_items::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
    }

    pub fn get_paginate_items (
        &self,
        limit:  Option<i64>,
        offset: Option<i64>,
    ) -> Vec<User> {
        use crate::schema::{
            follows_list_items::dsl::follows_list_items,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let ids = follows_list_items
            .filter(schema::follows_list_items::list_id.eq(self.list_id))
            .select(schema::follows_list_items::user_id)
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");

        return users
            .filter(schema::users::id.eq_any(ids))
            .filter(schema::users::types.lt(31))
            .load::<User>(&_connection)
            .expect("E.");
    }

    pub fn create_list(list_id: i32, user_id: i32) -> i16 {
        let _connection = establish_connection();

        let new_list = NewFollowsList {
            list_id: list_id,
            user_id: user_id,
            types:   5,
        };
        let _new_list = diesel::insert_into(schema::follows_lists::table)
            .values(&new_list)
            .get_result::<FollowsList>(&_connection)
            .expect("Error.");

        return 1;
    }

    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types + 80))
            .execute(&_connection);
        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types - 80))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        if self.types == 0 {
            return 0;
        }

        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types + 40))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();

        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types - 40))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn suspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types + 120))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unsuspend_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::follows_lists::types.eq(self.types - 120))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
}


#[derive(Debug, Serialize, Identifiable)]
pub struct FollowsListItem {
    pub id:      i32,
    pub list_id: i32,
    pub user_id: i32,
} 
#[derive(Deserialize, Insertable)]
#[table_name="follows_list_items"]
pub struct NewFollowsListItem { 
    pub list_id: i32,
    pub user_id: i32,
}

impl FollowsListItem {
    pub fn delete_follows_item (
        list_id: i32, user_id: i32,
    ) -> i16 { 
        use crate::schema::follows_list_items::dsl::follows_list_items;

        let _connection = establish_connection();
        diesel::delete (
            follows_list_items
                .filter(schema::follows_list_items::list_id.eq(list_id))
                .filter(schema::follows_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
    pub fn delete_follows_items (
        list_ids: Vec<i32>, user_id: i32,
    ) -> i16 { 
        use crate::schema::follows_list_items::dsl::follows_list_items;

        let _connection = establish_connection();
        diesel::delete (
            follows_list_items
                .filter(schema::follows_list_items::list_id.eq_any(list_ids))
                .filter(schema::follows_list_items::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
}


/*
follows_list_perms
1 пользователь может видеть список 
11 пользователь не может видеть список
101 список может видеть список 
111 список не может видеть список
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct FollowsListPerm {
    pub id:      i32,
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows_list_perms"]
pub struct NewFollowsListPerm {
    pub item_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
