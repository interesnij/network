use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    NullableExpressionMethods,
    ExpressionMethods,
    PgTextExpressionMethods,
    QueryDsl,
};
use crate::schema::{
    communities_lists,
    community_list_items,
    community_list_perms,
};
use crate::errors::Error;
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_communities_list,
    get_limit_offset, CardCommunityJson,
    EditListJson, CardCommunitiesList,
    DataListJson, RespListJson, CardUserJson, 
};
use actix_web::web::Json;
use crate::models::{
    User, Community,
};

/*
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

Приватность списка
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

14 Все пользователи
15 Подписчики
16 Персонал
17 Администраторы
18 Подписчики, кроме
19 Некоторые подписчики
20 Владелец сообщества
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
        let _connection = establish_connection();
        let new_item = NewCommunityItem {
            list_id: list_id,
            community_id: community_id,
            visited: 0,
        };
        diesel::insert_into(schema::community_list_items::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");
        
        diesel::update(self)
            .set(schema::communities_lists::count.eq(count + 1))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _users: Option<Vec<AttachOwner>>
    ) -> i16 {
        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 19 || (is_ie_mode && _users.is_none()) {
            return 0;
        }

        let _connection = establish_connection();
        let _update_field = match field {
            "see_el" => diesel::update(self)
                .set(schema::photo_lists::see_el.eq(value))
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
                _ => 0,
            };
        };
        if _users.is_some() && is_ie_mode {
            for _user in _users.unwrap().iter() {
                let _new_perm = NewCommunitiesListPerms {
                    user_id: _user.id,
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
    pub fn get_owner_meta(&self) -> Result<CardUserJson, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
            
        let _user = users
            .filter(schema::users::id.eq(self.user_id))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .first::<CardUserJson>(&_connection)
            .expect("E");

        return Ok(CardOwnerJson {
            name:  _user.first_name.clone() + &" ".to_string() + &_user.last_name.clone(),
            link:  _user.link,
            image: _user.image,
        })
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
        let ids = community_list_items
            .filter(schema::community_list_items::list_id.eq(self.id))
            .select(schema::community_list_items::community_id)
            .load::<i32>(&_connection)
            .expect("E.");

        return communitys
            .filter(schema::communitys::id.eq_any(ids))
            .filter(schema::communitys::types.lt(31))
            .order(schema::communitys::members.desc())
            .select(schema::communitys::id)
            .load::<i32>(&_connection)
            .expect("E.");
    }

    pub fn search_items (
        &self,
        q:       &String,
        limit:   Option<i64>,
        offset:  Option<i64>,
    ) -> Vec<CardCommunityJson> {
        use crate::schema::communitys::dsl::communitys;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();

        return communitys
            .filter(schema::communitys::list_id.eq(self.id))
            .filter(schema::communitys::name.ilike(&q))
            .filter(schema::communitys::types.lt(31))
            .limit(_limit)
            .offset(_offset)
            .order(schema::communitys::members.desc())
            .select((
                schema::communitys::id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::image,
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
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
        user_id: i32,
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
            .filter(schema::community_list_perms::user_id.eq(user_id))
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .select(schema::community_list_perms::user_id)
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
            community_list_perms::dsl::community_list_perms,
            follows::dsl::follows,
        };

        let _connection = establish_connection();
        return community_list_perms
            .filter(schema::community_list_perms::user_id.eq(user_id))
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .select(schema::community_list_perms::user_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        follows
            .filter(schema::follows::target_id.eq(self.user_id))
            .filter(schema::follows::user_id.eq(user_id))
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
            photo_list_perms::dsl::photo_list_perms,
            item_users::dsl::item_users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = community_list_perms
            .filter(schema::community_list_perms::list_id.eq(self.id))
            .filter(schema::community_list_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::community_list_perms::user_id)
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
        use crate::models::NewCommunitiesList;

        let _connection = establish_connection();
        let _name: String;
        let c_name = data.name.clone();
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
                for user_id in see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewCommunityListPerm {
                        user_id: *user_id,
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
                for user_id in see_el_users.as_deref().unwrap() {
                    let _new_include = NewCommunityListPerm {
                        user_id: *user_id,
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
    pub fn edit_list(list_id: i32, name: String, position: i16) -> RespListJson {
        use crate::schema::community_list_perms::dsl::community_list_perms;

        let _connection = establish_connection();
        let _name: String;
        let c_name = data.name.clone();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        let list = get_communities_list(list_id).expect("E.");

        diesel::update(&list)
            .set((
                schema::communities_lists::name.eq(_name.clone()),
                schema::communities_lists::position.eq(position),
            ))
            .execute(&_connection)
            .expect("Error.");

        let exclude_vec = vec![3, 5, 9, 11];
        let include_vec = vec![4, 6, 10, 12];

        diesel::delete (
            community_list_perms
                .filter(schema::community_list_perms::list_id.eq(list_id))
                .filter(schema::community_list_perms::types.ne(20))
        )
        .execute(&_connection)
        .expect("E");

        if exclude_vec.iter().any(|&i| i==list.see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewCommunityListPerm {
                        user_id: *user_id,
                        list_id: list_id,
                        types:   11,
                    };
                    diesel::insert_into(schema::community_list_perms::table)
                        .values(&_new_exclude)
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==list.see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.as_deref().unwrap() {
                    let _new_include = NewCommunityListPerm {
                        user_id: *user_id,
                        list_id: list_id,
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
            id:   list_id,
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
pub struct CommunityListItems {
    pub id:           i32,
    pub list_id:      i32,
    pub community_id: i32,
    pub visited:      i32,
} 
#[derive(Deserialize, Insertable)]
#[table_name="community_list_items"]
pub struct NewCommunityListItems { 
    pub list_id:      i32,
    pub community_id: i32,
    pub visited:      i32,
}

impl CommunityListItems {
    pub fn delete_community_item (
        list_id: i32, community_id: i32,
    ) -> i16 { 
        use crate::schema::community_list_items::dsl::community_list_items;

        let _connection = establish_connection();
        diesel::delete (
            community_list_items
                .filter(schema::community_list_items::list_id.eq(list_id))
                .filter(schema::community_list_items::community_id.eq(community_id))
        )
        .execute(&_connection)
        .expect("E.");

        return 1;
    }
    pub fn plus_visited(&self) -> () {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::community_list_items::visited.eq(self.visited + 1))
            .execute(&_connection)
            .expect("E.");
    }
}


/*
CommunityListPerm
1 может видеть список
11 не может видеть список
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityListPerm {
    pub id:      i32,
    pub user_id: i32,
    pub list_id: i32,
    pub types:   i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_list_perms"]
pub struct NewCommunityListPerm {
    pub user_id: i32,
    pub list_id: i32,
    pub types:   i16,
}