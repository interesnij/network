use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use crate::schema::photo_lists;

use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_photo_list,
    PhotoListDetailJson,
    PhotoListPageJson,
    UserListJson,
    CardUserJson,
    CardOwnerJson,
};
use actix_web::web::Json;
use crate::models::{
    Photo, User, Community,
    UserPhotoListCollection, NewUserPhotoListCollection,
    UserPhotoListPosition, CommunityPhotoListPosition,
    CommunityPhotoListCollection, NewCommunityPhotoListCollection,
    PhotoListPerm, NewPhotoListPerm,
};

/////// PhotoList //////
////////// Тип списка
    // 1 основной список
    // 2 пользовательский список
    // 3 список предложки
    // 4 Фото со страницы
    // 5 Фото со стены

    // 11 удаленный основной список
    // 12 удаленный пользовательский список
    // 13 удаленный список предложки
    // 14 удаленный Фото со страницы
    // 15 удаленный Фото со стены

    // 21 закрытый основной список
    // 22 закрытый пользовательский список
    // 23 закрытый список предложки
    // 24 закрытый Фото со страницы
    // 25 закрытый Фото со стены

    // 31 замороженный основной список
    // 32 замороженный пользовательский список
    // 33 замороженный список предложки
    // 34 замороженный Фото со страницы
    // 35 замороженный Фото со стены

//////////// Приватность списка
    // 1 Все пользователи
    // 2 Все друзья и все подписчики
    // 3 Все друзья и подписчики, кроме
    // 4 Все друзья и некоторые подписчики
    // 5 Все подписчики и друзья, кроме
    // 6 Все подписчики и некоторые друзья
    // 7 Все друзья
    // 8 Друзья, кроме
    // 9 Некоторые друзья
    // 10 Подписчики, кроме
    // 11 Некоторые подписчики
    // 12 Только я

    // 14 Все пользователи
    // 15 Подписчики
    // 16 Персонал
    // 17 Администраторы
    // 18 Подписчики, кроме
    // 19 Некоторые подписчики
    // 20 Владелец сообщества

/////// PhotoList //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PhotoList {
    pub id:             i32,
    pub name:           String,
    pub community_id:   Option<i32>,
    pub user_id:        i32,

    pub types:          i16,
    pub description:    Option<String>,
    pub image:          Option<String>,
    pub created:        chrono::NaiveDateTime,

    pub count:          i32,
    pub repost:         i32,
    pub copy:           i32,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,
    pub reactions:      Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_lists"]
pub struct NewPhotoList {
    pub name:           String,
    pub community_id:   Option<i32>,
    pub user_id:        i32,

    pub types:          i16,
    pub description:    Option<String>,
    pub image:          Option<String>,
    pub created:        chrono::NaiveDateTime,

    pub count:          i32,
    pub repost:         i32,
    pub copy:           i32,

    pub see_el:         i16,
    pub see_comment:    i16,
    pub create_el:      i16,
    pub create_comment: i16,
    pub copy_el:        i16,
    pub reactions:      Option<String>,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="photo_lists"]
pub struct EditPhotoList {
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

impl PhotoList {
    pub fn get_creator(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.user_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_owner_meta(&self) -> CardOwnerJson {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::communitys::dsl::communitys;

            let _community = communitys
                .filter(schema::communitys::id.eq(self.community_id.unwrap()))
                .filter(schema::communitys::types.lt(10))
                .select((
                    schema::communitys::name,
                    schema::communitys::link,
                    schema::communitys::s_avatar.nullable(),
                ))
                .load::<CardOwnerJson>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();
            return _community;
        }
        else {
            use crate::schema::users::dsl::users;

            let _user = users
                .filter(schema::users::id.eq(self.user_id))
                .filter(schema::users::types.lt(10))
                .select((
                    schema::users::user_id,
                    schema::users::first_name,
                    schema::users::last_name,
                    schema::users::link,
                    schema::users::s_avatar.nullable(),
                ))
                .load::<CardUserJson>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            return CardOwnerJson {
                name:  _user.first_name.clone() + &" ".to_string() + &_user.last_name.clone(),
                link:  _user.link,
                image: _user.image,
            }
        }
    }

    pub fn get_json_user_photo_page(user_id: i32, page: i32, limit: i32) -> Json<PhotoListPageJson> {
        use crate::utils::CardPhotoListJson;

        let mut next_page_number = 0;
        let selected_photo_list_pk = PhotoList::get_user_selected_photo_list_pk(user_id);
        let list = get_photo_list(selected_photo_list_pk);
        let lists: Vec<PhotoList>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            lists = PhotoList::get_user_photo_lists(user_id, limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            lists = PhotoList::get_user_photo_lists(user_id, limit.into(), 0);
        }
        if PhotoList::get_user_photo_lists(user_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        let mut lists_json = Vec::new();
        let list_owner = list.get_owner_meta();

        for i in lists.iter() {
            let owner = i.get_owner_meta();
            lists_json.push (
                CardPhotoListJson {
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

        let data = PhotoListPageJson {
            selected_list_id: selected_photo_list_pk,
            owner_name:       list_owner.name.clone(),
            owner_link:       list_owner.link.clone(),
            owner_image:      list_owner.image.clone(),
            image:            list.image,
            lists:            lists_json,
            next_page:        next_page_number,
        };
        return Json(data);
    }
    pub fn get_json_community_photo_page(community_id: i32, page: i32, limit: i32) -> Json<PhotoListPageJson> {
        use crate::utils::CardPhotoListJson;

        let mut next_page_number = 0;
        let selected_photo_list_pk = PhotoList::get_community_selected_photo_list_pk(community_id);
        let list = get_photo_list(selected_photo_list_pk);
        let lists: Vec<PhotoList>;

        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            lists = PhotoList::get_community_photo_lists(community_id, limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            lists = PhotoList::get_community_photo_lists(community_id, limit.into(), 0);
        }
        if PhotoList::get_community_photo_lists(community_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        let mut lists_json = Vec::new();
        let list_owner = list.get_owner_meta();
        for i in lists.iter() {
            let owner = i.get_owner_meta();
            lists_json.push (
                CardPhotoListJson {
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


        let data = PhotoListPageJson {
            selected_list_id: selected_photo_list_pk,
            owner_name:       list_owner.name.clone(),
            owner_link:       list_owner.link.clone(),
            owner_image:      list_owner.image.clone(),
            image:            list.image,
            lists:            lists_json,
            next_page:        next_page_number,
        };
        return Json(data);
    }

    pub fn get_json_photo_list (
        user_id: i32,
        list_id: i32,
        page: i32,
        limit: i32
    ) -> Json<PhotoListDetailJson> {
        use crate::utils::CardPhotoListJson;

        let mut next_page_number = 0;
        let list = get_photo_list(list_id);
        let lists: Vec<PhotoList>;
        if list.community_id.is_some() {
            lists = PhotoList::get_community_photo_lists(list.community_id.unwrap(), 20, 0);
        }
        else {
            lists = PhotoList::get_user_photo_lists(user_id, 20, 0);
        }
        let mut lists_json = Vec::new();
        let list_owner = list.get_owner_meta();

        for i in lists.iter() {
            let owner = i.get_owner_meta();
            lists_json.push (
                CardPhotoListJson {
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

        let photos: Vec<Photo>;
        let have_next: i32;
        let reactions_list = list.get_reactions_list();

        if page > 1 {
            have_next = page * limit + 1;
            photos = list.get_paginate_items(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            photos = list.get_paginate_items(limit.into(), 0);
        }
        if list.get_paginate_items(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        let mut photos_json = Vec::new();
        for i in photos.iter() {
            photos_json.push ( i.get_photo_json() )
        }

        let data = PhotoListDetailJson {
            id:                list.id,
            name:              list.name.clone(),
            owner_name:        list_owner.name.clone(),
            owner_link:        list_owner.link.clone(),
            owner_image:       list_owner.image.clone(),
            image:             list.image.clone(),
            types:             list.types,
            count:             list.count,
            reactions_list:    reactions_list,
            photos:            photos_json,
            lists:             lists_json,
            next_page:         next_page_number,
            is_user_create_el: list.is_user_create_el(user_id),
        };
        return Json(data);
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_photo_list(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "lpo".to_string() + &self.get_str_id();
    }

    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }
    pub fn get_reactions_list(&self) -> Vec<i32> {
        let mut stack = Vec::new();
        if self.reactions.is_some() {
            let react_string = self.reactions.as_ref().unwrap().to_string();
            if !react_string.is_empty() {
                let v: Vec<&str> = react_string.split(", ").collect();
                for item in v.iter() {
                    if !item.is_empty() {
                        let pk: i32 = item.parse().unwrap();
                        stack.push(pk);
                    }
                }
            }
        }
        return stack;
    }
    pub fn count_reactions_list(&self) -> usize {
        return self.get_reactions_list().len();
    }
    pub fn count_reactions_list_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_reactions_list().try_into().unwrap(),
            " реакция".to_string(),
            " реакции".to_string(),
            " реакций".to_string(),
        );
    }

    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }

    pub fn get_description(&self) -> String {
        return "<a data-photolist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user_id: i32) -> bool {
        return self.user_id == user_id;
    }
    pub fn is_community_list(&self, community_id: i32) -> bool {
        return self.community_id.unwrap() == community_id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_photo_list_collections::dsl::user_photo_list_collections;

        let _connection = establish_connection();
        let ids = user_photo_list_collections
            .filter(schema::user_photo_list_collections::photo_list_id.eq(self.id))
            .select(schema::user_photo_list_collections::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return ids;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_photo_list_collections::dsl::community_photo_list_collections;

        let _connection = establish_connection();
        let ids = community_photo_list_collections
            .filter(schema::community_photo_list_collections::photo_list_id.eq(self.id))
            .select(schema::community_photo_list_collections::community_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return ids;
    }
    pub fn is_user_collection_list(&self, user_id: i32) -> bool {
        return self.get_users_ids().iter().any(|&i| i==user_id);
    }
    pub fn is_community_collection_list(&self, community_id: i32) -> bool {
        return self.get_communities_ids().iter().any(|&i| i==community_id);
    }
    pub fn count_reposts(&self) -> String {
        if self.repost > 0 {
            return self.repost.to_string()
        }
        else {
            return "".to_string()
        }
    }
    pub fn get_items(&self) -> Vec<Photo> {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        return photos
            .filter(schema::photos::photo_list_id.eq(self.id))
            .filter(schema::photos::types.eq(1))
            .order(schema::photos::created.desc())
            .load::<Photo>(&_connection)
            .expect("E.");
    }
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Photo> {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        return photos
            .filter(schema::photos::photo_list_id.eq(self.id))
            .filter(schema::photos::types.eq(1))
            .limit(limit)
            .offset(offset)
            .order(schema::photos::created.desc())
            .load::<Photo>(&_connection)
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
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn get_see_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(11))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(1))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_see_el_exclude_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_see_el_exclude(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_see_el_exclude(limit.into(), 0);
        }
        if self.get_see_el_exclude(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_see_el_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_see_el_exclude_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_see_el_include_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;

        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_see_el_include(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_see_el_include(limit.into(), 0);
        }
        if self.get_see_el_include(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_see_el_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_see_el_include_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_see_comment_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(12))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_see_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(2))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }

    pub fn get_see_comment_exclude_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_see_comment_exclude(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_see_comment_exclude(limit.into(), 0);
        }
        if self.get_see_comment_exclude(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_see_comment_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_see_comment_exclude_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_see_comment_include_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_see_comment_include(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_see_comment_include(limit.into(), 0);
        }
        if self.get_see_comment_include(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_see_comment_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_see_comment_include_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(13))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(3))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }

    pub fn get_create_el_exclude_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_create_el_exclude(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_create_el_exclude(limit.into(), 0);
        }
        if self.get_create_el_exclude(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_create_el_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_create_el_exclude_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_el_include_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_create_el_include(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_create_el_include(limit.into(), 0);
        }
        if self.get_create_el_include(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_create_el_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_create_el_include_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_comment_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(14))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_create_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(4))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }

    pub fn get_create_comment_exclude_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_create_comment_exclude(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_create_comment_exclude(limit.into(), 0);
        }
        if self.get_create_comment_exclude(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_create_comment_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_create_comment_exclude_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_create_comment_include_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_create_comment_include(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_create_comment_include(limit.into(), 0);
        }
        if self.get_create_comment_include(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_create_comment_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_create_comment_include_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_copy_el_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(15))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let items = photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.eq(5))
            .select(schema::photo_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }

    pub fn get_copy_el_exclude_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_copy_el_exclude(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_copy_el_exclude(limit.into(), 0);
        }
        if self.get_copy_el_exclude(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_copy_el_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_copy_el_exclude_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn get_copy_el_include_json(&self, page: i32, limit: i32) -> Json<UserListJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;
        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_copy_el_include(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            users = self.get_copy_el_include(limit.into(), 0);
        }
        if self.get_copy_el_include(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return Json(UserListJson {
            users:     users,
            next_page: next_page_number,
        });
    }
    pub fn get_copy_el_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let items = users
            .filter(schema::users::user_id.eq_any(self.get_copy_el_include_users_ids()))
            .filter(schema::users::types.lt(10))
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return items;
    }

    pub fn is_user_see_el(&self, user_id: i32) -> bool {
        let private_field = self.see_el;
        if self.user_id == user_id || self.see_el == 1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match private_field {
                14 => true,
                15 => community.get_members_ids().iter().any(|&i| i==user_id),
                16 => community.get_staff_users_ids().iter().any(|&i| i==user_id),
                17 => community.get_administrators_ids().iter().any(|&i| i==user_id),
                18 => !self.get_see_el_exclude_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                19 => self.get_see_el_include_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                20 => community.user_id == user_id,
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match private_field {
                1 => true,
                2 => creator.get_friends_ids().iter().any(|&i| i==user_id) || creator.get_follows_ids().iter().any(|&i| i==user_id),
                3 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                4 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                5 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                6 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                7 => creator.get_friends_ids().iter().any(|&i| i==user_id),
                8 => !self.get_see_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                9 => self.get_see_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                10 => !self.get_see_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                11 => self.get_see_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                12 => creator.user_id == user_id,
                _ => false,
            };
        }
    }

    pub fn is_user_see_comment(&self, user_id: i32) -> bool {
        let private_field = &self.see_comment;
        if self.user_id == user_id || private_field == &1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match private_field {
                14 => true,
                15 => community.get_members_ids().iter().any(|&i| i==user_id),
                16 => community.get_staff_users_ids().iter().any(|&i| i==user_id),
                17 => community.get_administrators_ids().iter().any(|&i| i==user_id),
                18 => !self.get_see_comment_exclude_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                19 => self.get_see_comment_include_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                20 => community.user_id == user_id,
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match private_field {
                1 => true,
                2 => creator.get_friends_ids().iter().any(|&i| i==user_id) || creator.get_follows_ids().iter().any(|&i| i==user_id),
                3 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_see_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                4 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_see_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                5 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_see_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                6 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_see_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                7 => creator.get_friends_ids().iter().any(|&i| i==user_id),
                8 => !self.get_see_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                9 => self.get_see_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                10 => !self.get_see_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                11 => self.get_see_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                12 => creator.user_id == user_id,
                _ => false,
            };
        }
    }
    pub fn is_user_create_el(&self, user_id: i32) -> bool {
        let private_field = &self.create_el;
        if self.user_id == user_id || private_field == &1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match private_field {
                14 => true,
                15 => community.get_members_ids().iter().any(|&i| i==user_id),
                16 => community.get_staff_users_ids().iter().any(|&i| i==user_id),
                17 => community.get_administrators_ids().iter().any(|&i| i==user_id),
                18 => !self.get_create_el_exclude_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                19 => self.get_create_el_include_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                20 => community.user_id == user_id,
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match private_field {
                1 => true,
                2 => creator.get_friends_ids().iter().any(|&i| i==user_id) || creator.get_follows_ids().iter().any(|&i| i==user_id),
                3 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_create_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                4 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_create_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                5 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_create_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                6 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_create_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                7 => creator.get_friends_ids().iter().any(|&i| i==user_id),
                8 => !self.get_create_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                9 => self.get_create_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                10 => !self.get_create_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                11 => self.get_create_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                12 => creator.user_id == user_id,
                _ => false,
            };
        }
    }
    pub fn is_user_create_comment(&self, user_id: i32) -> bool {
        let private_field = &self.create_comment;
        if self.user_id == user_id || private_field == &1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match private_field {
                14 => true,
                15 => community.get_members_ids().iter().any(|&i| i==user_id),
                16 => community.get_staff_users_ids().iter().any(|&i| i==user_id),
                17 => community.get_administrators_ids().iter().any(|&i| i==user_id),
                18 => !self.get_create_comment_exclude_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                19 => self.get_create_comment_include_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                20 => community.user_id == user_id,
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match private_field {
                1 => true,
                2 => creator.get_friends_ids().iter().any(|&i| i==user_id) || creator.get_follows_ids().iter().any(|&i| i==user_id),
                3 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_create_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                4 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_create_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                5 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_create_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                6 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_create_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                7 => creator.get_friends_ids().iter().any(|&i| i==user_id),
                8 => !self.get_create_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                9 => self.get_create_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                10 => !self.get_create_comment_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                11 => self.get_create_comment_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                12 => creator.user_id == user_id,
                _ => false,
            };
        }
    }
    pub fn is_user_copy_el(&self, user_id: i32) -> bool {
        let private_field = &self.copy_el;
        if self.user_id == user_id || private_field == &1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community();
            return match private_field {
                14 => true,
                15 => community.get_members_ids().iter().any(|&i| i==user_id),
                16 => community.get_staff_users_ids().iter().any(|&i| i==user_id),
                17 => community.get_administrators_ids().iter().any(|&i| i==user_id),
                18 => !self.get_copy_el_exclude_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                19 => self.get_copy_el_include_users_ids().iter().any(|&i| i==user_id) && community.get_members_ids().iter().any(|&i| i==user_id),
                20 => community.user_id == user_id,
                _ => false,
            };
        }
        else {
            let creator = self.get_creator();
            return match private_field {
                1 => true,
                2 => creator.get_friends_ids().iter().any(|&i| i==user_id) || creator.get_follows_ids().iter().any(|&i| i==user_id),
                3 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (!self.get_copy_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                4 => creator.get_friends_ids().iter().any(|&i| i==user_id) || (self.get_copy_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id)),
                5 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (!self.get_copy_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                6 => creator.get_follows_ids().iter().any(|&i| i==user_id) || (self.get_copy_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id)),
                7 => creator.get_friends_ids().iter().any(|&i| i==user_id),
                8 => !self.get_copy_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                9 => self.get_copy_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_friends_ids().iter().any(|&i| i==user_id),
                10 => !self.get_copy_el_exclude_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                11 => self.get_copy_el_include_users_ids().iter().any(|&i| i==user_id) && creator.get_follows_ids().iter().any(|&i| i==user_id),
                12 => creator.user_id == user_id,
                _ => false,
            };
        }
    }
    pub fn is_anon_user_see_el(&self) -> bool {
        return self.see_el == 1 || self.see_el == 14;
    }
    pub fn is_anon_user_see_comment(&self) -> bool {
        return self.see_comment == 1 || self.see_comment == 14;
    }
    pub fn is_anon_user_create_item(&self) -> bool {
        return self.create_el == 1 || self.create_el == 14;
    }
    pub fn is_anon_user_create_comment(&self) -> bool {
        return self.create_comment == 1 || self.create_comment == 14;
    }
    pub fn is_anon_user_copy_el(&self) -> bool {
        return self.copy_el == 1 || self.copy_el == 14;
    }

    pub fn get_community_selected_photo_list_pk(community_id: i32) -> i32 {
        use crate::schema::community_photo_list_positions::dsl::community_photo_list_positions;

        let _connection = establish_connection();
        let _photo_list_positions = community_photo_list_positions
            .filter(schema::community_photo_list_positions::community_id.eq(community_id))
            .filter(schema::community_photo_list_positions::types.eq(1))
            .limit(1)
            .select(schema::community_photo_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if _photo_list_positions.len() > 0 {
            return _photo_list_positions
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            return PhotoList::get_community_photo_list(community_id).id;
        }
    }
    pub fn get_user_selected_photo_list_pk(user_id: i32) -> i32 {
        let _connection = establish_connection();

        use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;

        let _photo_list_positions = user_photo_list_positions
            .filter(schema::user_photo_list_positions::user_id.eq(user_id))
            .filter(schema::user_photo_list_positions::types.eq(1))
            .limit(1)
            .select(schema::user_photo_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if _photo_list_positions.len() > 0 {
            return _photo_list_positions
            .into_iter()
            .nth(0)
            .unwrap();
        }
        else {
            return PhotoList::get_user_photo_list(user_id).id;
        }
    }
    pub fn get_user_photo_list(user_id: i32) -> PhotoList {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let lists = photo_lists
            .filter(schema::photo_lists::user_id.eq(user_id))
            .filter(schema::photo_lists::community_id.is_null())
            .filter(schema::photo_lists::types.eq(1))
            .load::<PhotoList>(&_connection)
            .expect("E.");

        return lists.into_iter().nth(0).unwrap();
    }
    pub fn get_community_photo_list(community_id: i32) -> PhotoList {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let lists = photo_lists
            .filter(schema::photo_lists::community_id.eq(community_id))
            .filter(schema::photo_lists::types.eq(1))
            .load::<PhotoList>(&_connection)
            .expect("E.");
        return lists.into_iter().nth(0).unwrap();
    }

    pub fn get_user_photo_lists(user_id: i32, limit: i64, offset: i64) -> Vec<PhotoList> {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::user_id.eq(user_id))
            .filter(schema::photo_lists::community_id.is_null())
            .filter(schema::photo_lists::types.lt(10))
            .order(schema::photo_lists::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<PhotoList>(&_connection)
            .expect("E.");
    }
    pub fn count_user_photo_lists(user_id: i32) -> usize {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::user_id.eq(user_id))
            .filter(schema::photo_lists::community_id.is_null())
            .filter(schema::photo_lists::types.lt(10))
            .select(schema::photo_lists::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_community_photo_lists(community_id: i32, limit: i64, offset: i64) -> Vec<PhotoList> {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::community_id.eq(community_id))
            .filter(schema::photo_lists::types.lt(10))
            .order(schema::photo_lists::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<PhotoList>(&_connection)
            .expect("E.");
    }

    pub fn count_community_photo_lists(community_id: i32) -> usize {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::community_id.eq(community_id))
            .filter(schema::photo_lists::types.lt(10))
            .select(schema::photo_lists::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_user_photo_lists_new_position(user_id: i32) -> i16 {
        return (PhotoList::count_user_photo_lists(user_id) + 1).try_into().unwrap();
    }
    pub fn get_community_photo_lists_new_position(community_id: i32) -> i16 {
        return (PhotoList::count_community_photo_lists(community_id) + 1).try_into().unwrap();
    }

    pub fn create_list (
        name:                 String,
        community_id:         Option<i32>,
        creator_id:           i32,
        description:          Option<String>,
        image:                Option<String>,
        see_el:               i16,
        see_comment:          i16,
        create_el:            i16,
        create_comment:       i16,
        copy_el:              i16,
        see_el_users:         Option<Vec<i32>>,
        see_comment_users:    Option<Vec<i32>>,
        create_el_users:      Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,
        copy_el_users:        Option<Vec<i32>>,
        reactions:            Option<String>
    ) -> PhotoList {
        use crate::models::{
            NewCommunityPhotoListPosition,
            NewUserPhotoListPosition,
        };

        let _connection = establish_connection();
        let _name: String;
        if name.len() > 99 {
            _name = name[..100].to_string();
        }
        else {
            _name = name;
        }

        let new_photo_list = NewPhotoList {
            name:           _name,
            community_id:   community_id,
            user_id:        creator_id,
            types:          2,
            description:    description,
            image:          image,
            created:        chrono::Local::now().naive_utc(),
            count:          0,
            repost:         0,
            copy:           0,
            see_el:         see_el,
            see_comment:    see_comment,
            create_el:      create_el,
            create_comment: create_comment,
            copy_el:        copy_el,
            reactions:      reactions,
        };
        let new_list = diesel::insert_into(schema::photo_lists::table)
            .values(&new_photo_list)
            .get_result::<PhotoList>(&_connection)
            .expect("Error.");

        if community_id.is_some() {
            let community_pk = community_id.unwrap();
            let _new_photos_list_position = NewCommunityPhotoListPosition {
                community_id: community_pk,
                list_id:      new_list.id,
                position:     PhotoList::get_community_photo_lists_new_position(community_pk),
                types:        1,
            };
            let _photos_list_position = diesel::insert_into(schema::community_photo_list_positions::table)
                .values(&_new_photos_list_position)
                .get_result::<CommunityPhotoListPosition>(&_connection)
                .expect("Error.");
        }
        else {
            let _new_photos_list_position = NewUserPhotoListPosition {
                user_id:  creator_id,
                list_id:  new_list.id,
                position: PhotoList::get_user_photo_lists_new_position(creator_id),
                types:    1,
            };
            let _photos_list_position = diesel::insert_into(schema::user_photo_list_positions::table)
                .values(&_new_photos_list_position)
                .get_result::<UserPhotoListPosition>(&_connection)
                .expect("Error.");
        }
        let exclude_vec = vec![3, 5, 8, 10, 18];
        let include_vec = vec![4, 6, 9, 11, 19];

        if exclude_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:      user_id,
                        photo_list_id: new_list.id,
                        types:        11,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         1,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==see_comment) {
            if see_comment_users.is_some() {
                for user_id in see_comment_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         12,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_comment) {
            if see_comment_users.is_some() {
                for user_id in see_comment_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         2,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==create_el) {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         13,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==create_el) {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         3,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==create_comment) {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         14,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==create_comment) {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         4,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error saving photo_list_position.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==copy_el) {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         15,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error saving photo_list_position.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==copy_el) {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: new_list.id,
                        types:         5,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error saving photo_list_position.");
                }
            }
        }
        return new_list;
    }
    pub fn edit_list (
        &self,
        name:                 String,
        description:          Option<String>,
        image:                Option<String>,
        see_el:               i16,
        see_comment:          i16,
        create_el:            i16,
        create_comment:       i16,
        copy_el:              i16,
        see_el_users:         Option<Vec<i32>>,
        see_comment_users:    Option<Vec<i32>>,
        create_el_users:      Option<Vec<i32>>,
        create_comment_users: Option<Vec<i32>>,
        copy_el_users:        Option<Vec<i32>>,
        reactions:            Option<String>,
    ) -> &PhotoList {

        use crate::schema::photo_list_perms::dsl::photo_list_perms;

        let _connection = establish_connection();
        let _name: String;
        if name.len() > 99 {
            _name = name[..100].to_string();
        }
        else {
            _name = name;
        }
        let mut descr: Option<String> = Some("".to_string());
        let mut react: Option<String> = Some("".to_string());
        if description.is_some() {
            descr = description;
        }
        if reactions.is_some() {
            react = reactions;
        }

        let edit_photo_list = EditPhotoList {
            name:           _name,
            description:    descr,
            image:          image,
            see_el:         see_el,
            see_comment:    see_comment,
            create_el:      create_el,
            create_comment: create_comment,
            copy_el:        copy_el,
            reactions:      react,
        };
        diesel::update(self)
            .set(edit_photo_list)
            .get_result::<PhotoList>(&_connection)
            .expect("Error.");
        let exclude_vec = vec![3, 5, 8, 10, 18];
        let include_vec = vec![4, 6, 9, 11, 19];

        diesel::delete (
          photo_list_perms
            .filter(schema::photo_list_perms::photo_list_id.eq(self.id))
            .filter(schema::photo_list_perms::types.ne(20))
        )
        .execute(&_connection)
        .expect("E");

        if exclude_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         11,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el) {
            if see_el_users.is_some() {
                for user_id in see_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         1,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==see_comment) {
            if see_comment_users.is_some() {
                for user_id in see_comment_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         12,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error saving photo_list_position.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_comment) {
            if see_comment_users.is_some() {
                for user_id in see_comment_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         2,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==create_el) {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         13,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==create_el) {
            if create_el_users.is_some() {
                for user_id in create_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         3,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==create_comment) {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         14,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==create_comment) {
            if create_comment_users.is_some() {
                for user_id in create_comment_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         4,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==copy_el) {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_exclude = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         15,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==copy_el) {
            if copy_el_users.is_some() {
                for user_id in copy_el_users.unwrap() {
                    let _new_include = NewPhotoListPerm {
                        user_id:       user_id,
                        photo_list_id: self.id,
                        types:         5,
                    };
                    diesel::insert_into(schema::photo_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PhotoListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        return self;
    }
    pub fn get_order(&self) -> UserPhotoListPosition {
        use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;

        let _connection = establish_connection();
        return user_photo_list_positions
            .filter(schema::user_photo_list_positions::list_id.eq(self.id))
            .filter(schema::user_photo_list_positions::types.eq(1))
            .load::<UserPhotoListPosition>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn add_in_community_collections(&self, community_id: i32) -> () {
        use crate::models::NewCommunityPhotoListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community_id) && self.community_id.is_some() && self.community_id.unwrap() == community_id {
            return;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityPhotoListCollection {
            community_id:  community_id,
            photo_list_id: self.id,
        };
        diesel::insert_into(schema::community_photo_list_collections::table)
            .values(&new_item)
            .get_result::<CommunityPhotoListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityPhotoListPosition {
            community_id: community_id,
            list_id:      self.id,
            position:     PhotoList::get_community_photo_lists_new_position(community_id),
            types:        1,
        };
        diesel::insert_into(schema::community_photo_list_positions::table)
            .values(&new_pos)
            .get_result::<CommunityPhotoListPosition>(&_connection)
            .expect("Error.");
    }
    pub fn remove_in_community_collections(&self, community_id: i32) -> () {
        use crate::schema::community_photo_list_collections::dsl::community_photo_list_collections;
        use crate::schema::community_photo_list_positions::dsl::community_photo_list_positions;

        if self.get_communities_ids().iter().any(|&i| i==community_id) {
            return;
        }
        let _connection = establish_connection();
        diesel::delete(community_photo_list_collections
            .filter(schema::community_photo_list_collections::community_id.eq(community_id))
            .filter(schema::community_photo_list_collections::photo_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(community_photo_list_positions
            .filter(schema::community_photo_list_positions::community_id.eq(community_id))
            .filter(schema::community_photo_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
    }

    pub fn add_in_user_collections(&self, user_id: i32) -> () {
        use crate::models::NewUserPhotoListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user_id) && self.user_id == user_id {
            return;
        }
        let _connection = establish_connection();
        let new_item = NewUserPhotoListCollection {
            user_id:       user_id,
            photo_list_id: self.id,
        };
        diesel::insert_into(schema::user_photo_list_collections::table)
            .values(&new_item)
            .get_result::<UserPhotoListCollection>(&_connection)
            .expect("Error.");

        let new_pos = NewUserPhotoListPosition {
            user_id:  user_id,
            list_id:  self.id,
            position: PhotoList::get_user_photo_lists_new_position(user_id),
            types:    1,
        };
        diesel::insert_into(schema::user_photo_list_positions::table)
            .values(&new_pos)
            .get_result::<UserPhotoListPosition>(&_connection)
            .expect("Error.");
    }
    pub fn remove_in_user_collections(&self, user_id: i32) -> () {
        use crate::schema::user_photo_list_collections::dsl::user_photo_list_collections;
        use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;

        if self.get_users_ids().iter().any(|&i| i==user_id) {
            return;
        }
        let _connection = establish_connection();
        diesel::delete(user_photo_list_collections
            .filter(schema::user_photo_list_collections::user_id.eq(user_id))
            .filter(schema::user_photo_list_collections::photo_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete(user_photo_list_positions
            .filter(schema::user_photo_list_positions::user_id.eq(user_id))
            .filter(schema::user_photo_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
    }

    pub fn copy_item(pk: i32, user_or_communities: Vec<String>) -> () {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let lists = photo_lists
            .filter(schema::photo_lists::id.eq(pk))
            .filter(schema::photo_lists::types.lt(10))
            .load::<PhotoList>(&_connection)
            .expect("E.");
        if lists.len() > 0 {
            let list = lists.into_iter().nth(0).unwrap();
            for item in user_or_communities.iter() {
                let first = item.chars().nth(0).unwrap();
                if first == 'c' {
                    let c_id: i32 = item[..1].parse().unwrap();
                    list.add_in_community_collections(c_id);
                }
                else if first == 'u' {
                    let u_id: i32 = item[..1].parse().unwrap();
                    list.add_in_user_collections(u_id);
                }
            }
        }
    }
    pub fn get_photos_ids(&self) -> Vec<i32> {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        let fix_list_ids = photos
            .filter(schema::photos::photo_list_id.eq(self.id))
            .filter(schema::photos::types.lt(10))
            .select(schema::photos::id)
            .load::<i32>(&_connection)
            .expect("E.");
        return fix_list_ids;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<PhotoList> {
        use crate::schema::user_photo_list_collections::dsl::user_photo_list_collections;
        use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let position_lists = user_photo_list_positions
            .filter(schema::user_photo_list_positions::user_id.eq(user_pk))
            .filter(schema::user_photo_list_positions::types.eq(1))
            .select(schema::user_photo_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            return photo_lists
                .filter(schema::photo_lists::id.eq_any(position_lists))
                .filter(schema::photo_lists::types.lt(10))
                .load::<PhotoList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = photo_lists
            .filter(schema::photo_lists::user_id.eq(user_pk))
            .filter(schema::photo_lists::types.lt(10))
            .select(schema::photo_lists::id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item);
        };
        let user_collections = user_photo_list_collections
            .filter(schema::user_photo_list_collections::user_id.eq(user_pk))
            .select(schema::user_photo_list_collections::photo_list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item);
        };
        return photo_lists
            .filter(schema::photo_lists::id.eq_any(stack))
            .filter(schema::photo_lists::types.lt(10))
            .load::<PhotoList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<PhotoList> {
        use crate::schema::community_photo_list_collections::dsl::community_photo_list_collections;
        use crate::schema::community_photo_list_positions::dsl::community_photo_list_positions;
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let position_lists = community_photo_list_positions
            .filter(schema::community_photo_list_positions::community_id.eq(community_pk))
            .filter(schema::community_photo_list_positions::types.eq(1))
            .select(schema::community_photo_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            return photo_lists
                .filter(schema::photo_lists::id.eq_any(position_lists))
                .filter(schema::photo_lists::types.lt(10))
                .load::<PhotoList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = photo_lists
            .filter(schema::photo_lists::community_id.eq(community_pk))
            .filter(schema::photo_lists::types.lt(10))
            .select(schema::photo_lists::id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item);
        };
        let community_collections = community_photo_list_collections
            .filter(schema::community_photo_list_collections::community_id.eq(community_pk))
            .select(schema::community_photo_list_collections::photo_list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item);
        };
        return photo_lists
            .filter(schema::photo_lists::id.eq_any(stack))
            .filter(schema::photo_lists::types.lt(10))
            .load::<PhotoList>(&_connection)
            .expect("E.");

    }
    pub fn close_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 21,
            2 => 22,
            3 => 23,
            4 => 24,
            5 => 25,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //hide_wall_notify_items(20, self.id);
    }
    pub fn unclose_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            21 => 1,
            22 => 2,
            23 => 3,
            24 => 4,
            25 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //show_wall_notify_items(20, self.id);
    }

    pub fn delete_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_photo_list_positions::dsl::community_photo_list_positions;

            let list_positions = community_photo_list_positions
                .filter(schema::community_photo_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_photo_list_positions::list_id.eq(self.id))
                .load::<CommunityPhotoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_photo_list_positions::types.eq(2))
                  .get_result::<CommunityPhotoListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;

            let list_positions = user_photo_list_positions
                .filter(schema::user_photo_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_photo_list_positions::list_id.eq(self.id))
                .load::<UserPhotoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_photo_list_positions::types.eq(2))
                  .get_result::<UserPhotoListPosition>(&_connection)
                  .expect("Error.");
            }
        }
        let user_types = self.types;
        let close_case = match user_types {
            1 => 11,
            2 => 12,
            3 => 13,
            4 => 14,
            5 => 15,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //hide_wall_notify_items(20, self.id);
    }
    pub fn restore_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_photo_list_positions::dsl::community_photo_list_positions;

            let list_positions = community_photo_list_positions
                .filter(schema::community_photo_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_photo_list_positions::list_id.eq(self.id))
                .load::<CommunityPhotoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::community_photo_list_positions::types.eq(1))
                  .get_result::<CommunityPhotoListPosition>(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_photo_list_positions::dsl::user_photo_list_positions;

            let list_positions = user_photo_list_positions
                .filter(schema::user_photo_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_photo_list_positions::list_id.eq(self.id))
                .load::<UserPhotoListPosition>(&_connection)
                .expect("E.");
            if list_positions.len() > 0 {
                let list_position = list_positions.into_iter().nth(0).unwrap();
                diesel::update(&list_position)
                  .set(schema::user_photo_list_positions::types.eq(1))
                  .get_result::<UserPhotoListPosition>(&_connection)
                  .expect("Error.");
            }
        }
        let user_types = self.types;
        let close_case = match user_types {
            11 => 1,
            12 => 2,
            13 => 3,
            14 => 4,
            15 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //show_wall_notify_items(20, self.id);
    }

    pub fn suspend_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            1 => 31,
            2 => 32,
            3 => 33,
            4 => 34,
            5 => 35,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //hide_wall_notify_items(20, self.id);
    }
    pub fn unsuspend_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = self.types;
        let close_case = match user_types {
            31 => 1,
            32 => 2,
            33 => 3,
            34 => 4,
            35 => 5,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::photo_lists::types.eq(close_case))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        //show_wall_notify_items(20, self.id);
    }

    pub fn create_photo (
        &self,
        community_id: Option<i32>,
        user_id: i32,
        preview: String,
        file: String
    ) -> Photo {
        use crate::models::NewPhoto;

        let _connection = establish_connection();
        diesel::update(self)
          .set(schema::photo_lists::count.eq(self.count + 1))
          .get_result::<PhotoList>(&_connection)
          .expect("Error.");

        let _types: i16;
        //let mut _content: Option<String> = None;
        //let creator = get_user(user_id);

        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}
        let new_photo_form = NewPhoto {
          community_id:  community_id,
          user_id:       user_id,
          photo_list_id: self.id,
          types:         1,
          preview:       preview.clone(),
          file:          file.clone(),
          description:   None,
          comments_on:   true,
          created:       chrono::Local::now().naive_utc(),
          comment:       0,
          view:          0,
          repost:        0,
          copy:          0,
          position:      (self.count).try_into().unwrap(),
          reactions:     0,
        };
        let new_photo = diesel::insert_into(schema::photos::table)
            .values(&new_photo_form)
            .get_result::<Photo>(&_connection)
            .expect("Error.");

        //if attach.is_some() {
            //use crate::models::NewAttachItem;
            //use crate::schema::attach_items::dsl::attach_items;

            //let v: Vec<&str> = attach.split(",").collect();
            //for item in v.iter() {
            //    if item.len() > 3 {
            //        let pk: i32 = item[3..].parse().unwrap();
            //        let code = &item[..3];
            //    }
            //}
        //}
        return new_photo;
    }
}
