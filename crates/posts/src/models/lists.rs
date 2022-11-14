use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    NullableExpressionMethods,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema::post_lists;
use crate::errors::Error;
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_post_list,
    PostListDetailJson,
    PostListPageJson,
    CardUserJson,
    CardOwnerJson,
    ReactionsJson,
    EditListJson, RespListJson, DataListJson,
    DataNewPost, RespPost,
};
use actix_web::web::Json;
use crate::models::{
    Post, User, Community,
    UserPostListCollection, NewUserPostListCollection,
    UserPostListPosition, CommunityPostListPosition,
    CommunityPostListCollection, NewCommunityPostListCollection,
    PostListPerm, NewPostListPerm,
};

////////// Тип списка
    // 0 основной список
    // 5 пользовательский список
    // 10 список предложки
    // 15 Фото со страницы
    // 20 Фото со стены
    // 25 основной список приватный
    // 30 пользовательский список приватный

    // 45 удаленный пользовательский список
    // 70 удаленный пользовательский список приватный

    // 80 закрытый основной список
    // 85 закрытый пользовательский список
    // 95 закрытый Фото со страницы
    // 100 закрытый Фото со стены
    // 105 закрытый основной список приватный
    // 110 закрытый пользовательский список приватный

    // 120 замороженный основной список
    // 125 замороженный пользовательский список
    // 135 замороженный Фото со страницы
    // 140 замороженный Фото со стены
    // 145 замороженный основной список приватный
    // 150 замороженный пользовательский список приватный

    // 165 полностью удаленный пользовательский список
    // 190 полностью удаленный пользовательский список приватный

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

/////// PostList //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostList {
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
#[table_name="post_lists"]
pub struct NewPostList {
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
#[table_name="post_lists"]
pub struct EditPostList {
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

impl PostList {
    pub fn get_creator(&self) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return Ok(users
            .filter(schema::users::user_id.eq(self.user_id))
            .first::<User>(&_connection)?);
    }
    pub fn get_community(&self) -> Result<Community, Error> {
        use crate::schema::communitys::dsl::communitys;
        let _connection = establish_connection();
        return Ok(communitys
            .filter(schema::communitys::community_id.eq(self.community_id.unwrap()))
            .first::<Community>(&_connection)?);
    }
    pub fn get_add_list_json() -> Result<ReactionsJson, Error> {
        use crate::schema::reactions::dsl::reactions;
        use crate::utils::ReactionJson;

        let _connection = establish_connection();
        let _reactions = reactions
            .select((
                schema::reactions::image,
                schema::reactions::name,
            ))
            .load::<ReactionJson>(&_connection)?;
        return Ok(ReactionsJson {
            reactions: _reactions,
        });
    }
    pub fn get_edit_list_json(&self) -> Result<EditListJson, Error> {
        return Ok(EditListJson {
            id:                   self.id,
            name:                 self.name.clone(),
            description:          self.description.clone(),
            image:                self.image.clone(),
            see_el:               self.see_el,
            see_comment:          self.see_comment,
            create_el:            self.create_el,
            create_comment:       self.create_comment,
            copy_el:              self.copy_el,
            reactions:            self.reactions.clone(),

            see_el_include_users:         self.get_see_el_include_users_ids(),
            see_comment_include_users:    self.get_see_comment_include_users_ids(),
            create_el_include_users:      self.get_create_el_include_users_ids(),
            create_comment_include_users: self.get_create_comment_include_users_ids(),
            copy_el_include_users:        self.get_copy_el_include_users_ids(),

            see_el_exclude_users:         self.get_see_el_exclude_users_ids(),
            see_comment_exclude_users:    self.get_see_comment_exclude_users_ids(),
            create_el_exclude_users:      self.get_create_el_exclude_users_ids(),
            create_comment_exclude_users: self.get_create_comment_exclude_users_ids(),
            copy_el_exclude_users:        self.get_copy_el_exclude_users_ids(),
        });
    }
    pub fn get_owner_meta(&self) -> Result<CardOwnerJson, Error> {
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
                .first::<CardOwnerJson>(&_connection)?;
            return Ok(_community);
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
                .first::<CardUserJson>(&_connection)
                .expect("E");

            return Ok(CardOwnerJson {
                name:  _user.first_name.clone() + &" ".to_string() + &_user.last_name.clone(),
                link:  _user.link,
                image: _user.image,
            })
        }
    }

    pub fn get_user_post_page(user_id: i32) -> Option<PostListPageJson> {
        use crate::utils::CardPostListJson;

        let selected_post_list_pk = PostList::get_user_selected_post_list_pk(user_id);
        let list = get_post_list(selected_post_list_pk).expect("E.");

        let lists = PostList::get_user_post_lists(user_id, 10, 0);

        let mut lists_json = Vec::new();
        let list_owner = list.get_owner_meta().expect("E");

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

        let data = PostListPageJson {
            selected_list_id: selected_post_list_pk,
            owner_name:       list_owner.name.clone(),
            owner_link:       list_owner.link.clone(),
            owner_image:      list_owner.image.clone(),
            image:            list.image,
            lists:            lists_json,
        };
        return Some(data);
    }
    pub fn get_community_post_page(community_id: i32) -> PostListPageJson {
        use crate::utils::CardPostListJson;

        let selected_post_list_pk = PostList::get_community_selected_post_list_pk(community_id);
        let list = get_post_list(selected_post_list_pk).expect("E.");
        let lists = PostList::get_community_post_lists(community_id, 10, 0);

        let mut lists_json = Vec::new();
        let list_owner = list.get_owner_meta().expect("E");
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


        let data = PostListPageJson {
            selected_list_id: selected_post_list_pk,
            owner_name:       list_owner.name.clone(),
            owner_link:       list_owner.link.clone(),
            owner_image:      list_owner.image.clone(),
            image:            list.image,
            lists:            lists_json,
        };
        return data;
    }

    pub fn get_user_post_list_json (
        owner:   User,
        user_id: i32,
        list:    PostList,
        lists:   Vec<PostList>,
        limit:   i64,
        offset:  i64,
    ) -> Json<PostListDetailJson> {
        use crate::utils::CardPostListJson;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }

        let mut lists_json = Vec::new();
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

        let posts = list.get_paginate_items(limit, offset);
        let reactions_list = list.get_reactions_list();

        let mut posts_json = Vec::new();
        for i in posts.iter() {
            posts_json.push ( i.get_post_json(user_id, reactions_list.clone()) )
        }

        let data = PostListDetailJson {
            id:                list.id,
            name:              list.name.clone(),
            owner_name:        owner.get_full_name(),
            owner_link:        owner.link.clone(),
            owner_image:       owner.s_avatar.clone(),
            image:             list.image.clone(),
            types:             list.types,
            count:             list.count,
            reactions_list:    reactions_list,
            posts:             posts_json,
            lists:             lists_json,
            is_user_create_el: list.is_user_create_el(user_id),
        };
        return Json(data);
    }
    pub fn get_anon_user_post_list_json (
        owner:   User,
        list:    PostList,
        lists:   Vec<PostList>,
        limit:   i64,
        offset:  i64,
    ) -> Json<PostListDetailJson> {
        use crate::utils::CardPostListJson;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }

        let mut lists_json = Vec::new();
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

        let posts = list.get_paginate_items(limit, offset);
        let reactions_list = list.get_reactions_list();

        let mut posts_json = Vec::new();
        for i in posts.iter() {
            posts_json.push ( i.get_post_json(0, reactions_list.clone()) );
        }

        let data = PostListDetailJson {
            id:                list.id,
            name:              list.name.clone(),
            owner_name:        owner.get_full_name(),
            owner_link:        owner.link.clone(),
            owner_image:       owner.s_avatar.clone(),
            image:             list.image.clone(),
            types:             list.types,
            count:             list.count,
            reactions_list:    reactions_list,
            posts:             posts_json,
            lists:             lists_json,
            is_user_create_el: false,
        };
        return Json(data);
    }

    pub fn get_community_post_list_json (
        community: Community,
        user_id:   i32,
        list:      PostList,
        lists:     Vec<PostList>,
        limit:     i64,
        offset:    i64,
    ) -> Json<PostListDetailJson> {
        use crate::utils::CardPostListJson;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }

        let mut lists_json = Vec::new();
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

        let posts = list.get_paginate_items(limit, offset);
        let reactions_list = list.get_reactions_list();

        let mut posts_json = Vec::new();
        for i in posts.iter() {
            posts_json.push ( i.get_post_json(user_id, reactions_list.clone()) );
        }

        let data = PostListDetailJson {
            id:                list.id,
            name:              list.name.clone(),
            owner_name:        community.name.clone(),
            owner_link:        community.link.clone(),
            owner_image:       community.s_avatar.clone(),
            image:             list.image.clone(),
            types:             list.types,
            count:             list.count,
            reactions_list:    reactions_list,
            posts:             posts_json,
            lists:             lists_json,
            is_user_create_el: list.is_user_create_el(user_id),
        };
        return Json(data);
    }
    pub fn get_anon_community_post_list_json (
        community: Community,
        list:      PostList,
        lists:     Vec<PostList>,
        limit:     i64,
        offset:    i64,
    ) -> Json<PostListDetailJson> {
        use crate::utils::CardPostListJson;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }

        let mut lists_json = Vec::new();
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

        let posts = list.get_paginate_items(limit, offset);
        let reactions_list = list.get_reactions_list();

        let mut posts_json = Vec::new();
        for i in posts.iter() {
            posts_json.push ( i.get_post_json(0, reactions_list.clone()) );
        }

        let data = PostListDetailJson {
            id:                list.id,
            name:              list.name.clone(),
            owner_name:        community.name.clone(),
            owner_link:        community.link.clone(),
            owner_image:       community.s_avatar.clone(),
            image:             list.image.clone(),
            types:             list.types,
            count:             list.count,
            reactions_list:    reactions_list,
            posts:             posts_json,
            lists:             lists_json,
            is_user_create_el: false,
        };
        return Json(data);
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post_list(&self) -> bool {
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
        return "<a data-postlist='".to_string() + &self.get_str_id() + &"' class='ajax'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn is_user_list(&self, user_id: i32) -> bool {
        return self.user_id == user_id;
    }
    pub fn is_community_list(&self, community_id: i32) -> bool {
        return self.community_id.unwrap() == community_id;
    }
    pub fn get_users_ids(&self) -> Vec<i32> {
        use crate::schema::user_post_list_collections::dsl::user_post_list_collections;

        let _connection = establish_connection();
        let ids = user_post_list_collections
            .filter(schema::user_post_list_collections::post_list_id.eq(self.id))
            .select(schema::user_post_list_collections::user_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return ids;
    }
    pub fn get_communities_ids(&self) -> Vec<i32> {
        use crate::schema::community_post_list_collections::dsl::community_post_list_collections;

        let _connection = establish_connection();
        let ids = community_post_list_collections
            .filter(schema::community_post_list_collections::post_list_id.eq(self.id))
            .select(schema::community_post_list_collections::community_id)
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
    pub fn get_items(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::posts::types.lt(11))
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
            .expect("E.");
    }

    pub fn search_items (
        &self,
        q:       &String,
        user_id: i64,
        limit:   i64,
        offset:  i64,
    ) -> Vec<CardPostJson> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }
        let reactions_list = list.get_reactions_list();

        let mut posts_json = Vec::new();
        let items = posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::items::content.ilike(&q))
            .filter(schema::posts::types.lt(11))
            .limit(_limit)
            .offset(offset)
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
            .expect("E.");

        for i in items.iter() {
            posts_json.push ( i.get_post_json(user_id, reactions_list.clone()) )
        }
        return posts_json;
    }
    pub fn get_paginate_items(&self, limit: i64, offset: i64) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::posts::types.lt(11))
            .limit(limit)
            .offset(offset)
            .order(schema::posts::created.desc())
            .load::<Post>(&_connection)
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(11))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_see_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(1))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(12))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_see_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(2))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(13))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_create_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(3))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(14))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_create_comment_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(4))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
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
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(15))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
    }
    pub fn get_copy_el_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let items = post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(self.id))
            .filter(schema::post_list_perms::types.eq(5))
            .select(schema::post_list_perms::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        return items;
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
            let community = self.get_community().expect("E");
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
            let creator = self.get_creator().expect("E");
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
            let community = self.get_community().expect("E");
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
            let creator = self.get_creator().expect("E");
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
        if private_field == &1 {
            return true;
        }

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
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
            let creator = self.get_creator().expect("E");
            if self.user_id == user_id {
                return true;
            }
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
            let community = self.get_community().expect("E");
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
            let creator = self.get_creator().expect("E");
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
            let community = self.get_community().expect("E");
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
            let creator = self.get_creator().expect("E");
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

    pub fn get_community_selected_post_list_pk(community_id: i32) -> i32 {
        use crate::schema::community_post_list_positions::dsl::community_post_list_positions;

        let _connection = establish_connection();
        let _post_list_positions = community_post_list_positions
            .filter(schema::community_post_list_positions::community_id.eq(community_id))
            .filter(schema::community_post_list_positions::types.eq(1))
            .select(schema::community_post_list_positions::list_id)
            .first::<i32>(&_connection);
        if _post_list_positions.is_ok() {
            return _post_list_positions.expect("E.");
        }
        else {
            return PostList::get_community_post_list(community_id).id;
        }
    }
    pub fn get_user_selected_post_list_pk(user_id: i32) -> i32 {
        let _connection = establish_connection();

        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

        let _post_list_positions = user_post_list_positions
            .filter(schema::user_post_list_positions::user_id.eq(user_id))
            .filter(schema::user_post_list_positions::types.eq(1))
            .limit(1)
            .select(schema::user_post_list_positions::list_id)
            .load::<i32>(&_connection);
        if _post_list_positions.is_ok() {
            return _post_list_positions.expect("E.");
        }
        else {
            return PostList::get_user_post_list(user_id).id;
        }
    }
    pub fn get_user_post_list(user_id: i32) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let list = post_lists
            .filter(schema::post_lists::user_id.eq(user_id))
            .filter(schema::post_lists::community_id.is_null())
            .filter(schema::post_lists::types.eq(0))
            .first::<PostList>(&_connection);

        if  list.is_ok() {
            return list.expect("E.");
        }
        else {
            use crate::models::{
                NewCommunityPostListPosition,
                NewUserPostListPosition,
            };
            use crate::utils::get_user;

            let _connection = establish_connection();

            let new_post_list = NewPostList {
                name:           "Список записей".to_string(),
                community_id:   None,
                user_id:        user_id,
                types:          0,
                description:    None,
                image:          None,
                created:        chrono::Local::now().naive_utc(),
                count:          0,
                repost:         0,
                copy:           0,
                see_el:         1,
                see_comment:    1,
                create_el:      12,
                create_comment: 1,
                copy_el:        1,
                reactions:      None,
            };
            let new_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_post_list)
                .get_result::<PostList>(&_connection)
                .expect("Error.");

            let _user = get_user(user_id).expect("Error.");
            _user.plus_lists(1);

            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  user_id,
                list_id:  new_list.id,
                position: 1,
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .execute(&_connection)
                .expect("Error.");
            }
            return new_list;
        }
    }
    pub fn get_community_post_list(community_id: i32) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let list = post_lists
            .filter(schema::post_lists::community_id.eq(community_id))
            .filter(schema::post_lists::types.eq(0))
            .first::<PostList>(&_connection);

        if  list.is_ok() {
            return list.expect("E.");
        }
        else {
            use crate::models::{
                NewCommunityPostListPosition,
                NewCommunityPostListPosition,
            };
            use crate::utils::get_community;

            let _community = get_community(community_id).expect("Error.");
            let open_vec = vac![1,7,13];
            let create_type: i16;
            if open_vec.iter().any()(|&i| i==_community.types) {
                open_type = 14;
            }
            else {
                open_type = 15;
            }
            let _connection = establish_connection();

            let new_post_list = NewPostList {
                name:           "Список записей".to_string(),
                community_id:   Some(community_id),
                user_id:        community.user_id,
                types:          0,
                description:    None,
                image:          None,
                created:        chrono::Local::now().naive_utc(),
                count:          0,
                repost:         0,
                copy:           0,
                see_el:         open_type,
                see_comment:    open_type,
                create_el:      12,
                create_comment: open_type,
                copy_el:        open_type,
                reactions:      None,
            };
            let new_list = diesel::insert_into(schema::post_lists::table)
                .values(&new_post_list)
                .get_result::<PostList>(&_connection)
                .expect("Error.");

            _community.plus_lists(1);

            let _new_posts_list_position = NewCommunityPostListPosition {
                community_id: community_id,
                list_id:      new_list.id,
                position:     1,
                types:        1,
            };
            diesel::insert_into(schema::community_post_list_positions::table)
                .values(&_new_posts_list_position)
                .execute(&_connection)
                .expect("Error.");
            }
            return new_list;
        }
    }

    pub fn get_user_post_lists(user_id: i32, limit: i64, offset: i64) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::user_id.eq(user_id))
            .filter(schema::post_lists::community_id.is_null())
            .filter(schema::post_lists::types.lt(31))
            .order(schema::post_lists::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn count_user_post_lists(user_id: i32) -> i16 {
        use crate::utils::get_user;

        let _user = get_user(user_id).expect("E.");
        return _user.count_lists();
    }

    pub fn get_community_post_lists(community_id: i32, limit: i64, offset: i64) -> Vec<PostList> {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::community_id.eq(community_id))
            .filter(schema::post_lists::types.lt(31))
            .order(schema::post_lists::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<PostList>(&_connection)
            .expect("E.");
    }

    pub fn count_community_post_lists(community_id: i32) -> i16 {
        use crate::utils::get_community;

        let _community = get_community(community_id).expect("E.");
        return _community.count_lists();
    }

    pub fn get_user_post_lists_new_position(user_id: i32) -> i16 {
        let _user = get_user(user_id).expect("E.");
        let count = _user.count_lists() + 1;
        _user.plus_lists(1);
        return count;
    }
    pub fn get_community_post_lists_new_position(community_id: i32) -> i16 {
        let _community = get_community(community_id).expect("E.");
        let count = _community.count_lists() + 1;
        _community.plus_lists(1);
        return count;
    }

    pub fn create_list (data: Json<DataListJson>) -> RespListJson {
        use crate::models::{
            NewCommunityPostListPosition,
            NewUserPostListPosition,
        };

        let _connection = establish_connection();
        let _name: String;
        let c_name = data.name.as_deref().unwrap().to_string();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }

        let new_post_list = NewPostList {
            name:           _name.clone(),
            community_id:   data.community_id,
            user_id:        data.user_id.unwrap(),
            types:          2,
            description:    data.description.clone(),
            image:          data.image.clone(),
            created:        chrono::Local::now().naive_utc(),
            count:          0,
            repost:         0,
            copy:           0,
            see_el:         data.see_el.unwrap(),
            see_comment:    data.see_comment.unwrap(),
            create_el:      data.create_el.unwrap(),
            create_comment: data.create_comment.unwrap(),
            copy_el:        data.copy_el.unwrap(),
            reactions:      data.reactions.clone(),
        };
        let new_list = diesel::insert_into(schema::post_lists::table)
            .values(&new_post_list)
            .get_result::<PostList>(&_connection)
            .expect("Error.");

        // тут мелкий фокус: чтобы не получать владельца триста раз
        // мы в одной проге получаем его, добавляем 1 к списку
        // его записей и возвращаем порядковый номер для создания
        // такой таблицы.
        if data.community_id.is_some() {
            let community_pk = data.community_id.unwrap();
            let _new_posts_list_position = NewCommunityPostListPosition {
                community_id: community_pk,
                list_id:      new_list.id,
                position:     PostList::get_community_post_lists_new_position(community_pk),
                types:        1,
            };
            let _posts_list_position = diesel::insert_into(schema::community_post_list_positions::table)
                .values(&_new_posts_list_position)
                .get_result::<CommunityPostListPosition>(&_connection)
                .expect("Error.");
        }
        else {
            let _new_posts_list_position = NewUserPostListPosition {
                user_id:  data.user_id.unwrap(),
                list_id:  new_list.id,
                position: PostList::get_user_post_lists_new_position(data.user_id.unwrap()),
                types:    1,
            };
            let _posts_list_position = diesel::insert_into(schema::user_post_list_positions::table)
                .values(&_new_posts_list_position)
                .get_result::<UserPostListPosition>(&_connection)
                .expect("Error.");
        }
        let exclude_vec = vec![3, 5, 8, 10, 18];
        let include_vec = vec![4, 6, 9, 11, 19];

        if exclude_vec.iter().any(|&i| i==data.see_el.unwrap()) {
            if data.see_el_users.is_some() {
                for user_id in data.see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        11,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.see_el.unwrap()) {
            if data.see_el_users.is_some() {
                for user_id in data.see_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        1,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.see_comment.unwrap()) {
            if data.see_comment_users.is_some() {
                for user_id in data.see_comment_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        12,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.see_comment.unwrap()) {
            if data.see_comment_users.is_some() {
                for user_id in data.see_comment_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        2,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.create_el.unwrap()) {
            if data.create_el_users.is_some() {
                for user_id in data.create_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        13,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.create_el.unwrap()) {
            if data.create_el_users.is_some() {
                for user_id in data.create_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        3,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.create_comment.unwrap()) {
            if data.create_comment_users.is_some() {
                for user_id in data.create_comment_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        14,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.create_comment.unwrap()) {
            if data.create_comment_users.is_some() {
                for user_id in data.create_comment_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        4,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.copy_el.unwrap()) {
            if data.copy_el_users.is_some() {
                for user_id in data.copy_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        15,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.copy_el.unwrap()) {
            if data.copy_el_users.is_some() {
                for user_id in data.copy_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: new_list.id,
                        types:        5,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        return RespListJson {
            id:             new_list.id,
            community_id:   data.community_id,
            user_id:        data.user_id.unwrap(),
            name:           _name,
            description:    data.description.clone(),
            image:          data.image.clone(),
            see_el:         data.see_el.unwrap(),
            see_comment:    data.see_comment.unwrap(),
            create_el:      data.create_el.unwrap(),
            create_comment: data.create_comment.unwrap(),
            copy_el:        data.copy_el.unwrap(),
            reactions:      data.reactions.clone(),
        };
    }
    pub fn edit_list(data: Json<DataListJson>) -> RespListJson {
        use crate::schema::post_list_perms::dsl::post_list_perms;

        let _connection = establish_connection();
        let _name: String;
        let c_name = data.name.as_deref().unwrap().to_string();
        if c_name.len() > 99 {
            _name = c_name[..100].to_string();
        }
        else {
            _name = c_name;
        }
        let mut descr: Option<String> = Some("".to_string());
        let mut react: Option<String> = Some("".to_string());
        if data.description.is_some() {
            descr = data.description.clone();
        }
        if data.reactions.is_some() {
            react = data.reactions.clone();
        }
        let _id = data.id.unwrap();
        let list = get_post_list(_id).expect("E.");
        let list_see_el = list.see_el;

        let edit_post_list = EditPostList {
            name:           _name.clone(),
            description:    descr.clone(),
            image:          data.image.clone(),
            see_el:         data.see_el.unwrap(),
            see_comment:    data.see_comment.unwrap(),
            create_el:      data.create_el.unwrap(),
            create_comment: data.create_comment.unwrap(),
            copy_el:        data.copy_el.unwrap(),
            reactions:      react.clone(),
        };
        diesel::update(&list)
            .set(edit_post_list)
            .get_result::<PostList>(&_connection)
            .expect("Error.");
        let exclude_vec = vec![3, 5, 8, 10, 18];
        let include_vec = vec![4, 6, 9, 11, 19];

        // если отредактированный список стал видимым всем,
        // мы всем записям списка, которые опубликованы,
        // присвоим тип "приватные", чтобы они не участвовали
        // в публичных мероприятиях типа поиска
        // Ну и наоборот, в противном случае.
        let see_el_unwrap = data.see_el.unwrap();
        if see_el_unwrap != list_see_el {
            if see_el_unwrap == 1 || see_el_unwrap == 14 {
                for item in list.get_items().iter() {
                    if item.types == 5 {
                        diesel::update(&item)
                            .set(types.eq(types - 5))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            }
            else {
                for item in list.get_items().iter() {
                    if item.types == 0 {
                        diesel::update(&item)
                            .set(types.eq(types + 5))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            }
        }

        diesel::delete (
          post_list_perms
            .filter(schema::post_list_perms::post_list_id.eq(_id))
            .filter(schema::post_list_perms::types.ne(20))
        )
        .execute(&_connection)
        .expect("E");

        if exclude_vec.iter().any(|&i| i==see_el_unwrap) {
            if data.see_el_users.is_some() {
                for user_id in data.see_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        11,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==see_el_unwrap) {
            if data.see_el_users.is_some() {
                for user_id in data.see_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        1,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.see_comment.unwrap()) {
            if data.see_comment_users.is_some() {
                for user_id in data.see_comment_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        12,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error saving post_list_position.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.see_comment.unwrap()) {
            if data.see_comment_users.is_some() {
                for user_id in data.see_comment_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        2,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.create_el.unwrap()) {
            if data.create_el_users.is_some() {
                for user_id in data.create_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        13,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.create_el.unwrap()) {
            if data.create_el_users.is_some() {
                for user_id in data.create_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        3,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.create_comment.unwrap()) {
            if data.create_comment_users.is_some() {
                for user_id in data.create_comment_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        14,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.create_comment.unwrap()) {
            if data.create_comment_users.is_some() {
                for user_id in data.create_comment_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        4,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }

        if exclude_vec.iter().any(|&i| i==data.copy_el.unwrap()) {
            if data.copy_el_users.is_some() {
                for user_id in data.copy_el_users.as_deref().unwrap() {
                    let _new_exclude = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        15,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_exclude)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        else if include_vec.iter().any(|&i| i==data.copy_el.unwrap()) {
            if data.copy_el_users.is_some() {
                for user_id in data.copy_el_users.as_deref().unwrap() {
                    let _new_include = NewPostListPerm {
                        user_id:      *user_id,
                        post_list_id: _id,
                        types:        5,
                    };
                    diesel::insert_into(schema::post_list_perms::table)
                        .values(&_new_include)
                        .get_result::<PostListPerm>(&_connection)
                        .expect("Error.");
                }
            }
        }
        return RespListJson {
            id:             list.id,
            community_id:   list.community_id,
            user_id:        list.user_id,
            name:           _name,
            description:    descr,
            image:          data.image.clone(),
            see_el:         data.see_el.unwrap(),
            see_comment:    data.see_comment.unwrap(),
            create_el:      data.create_el.unwrap(),
            create_comment: data.create_comment.unwrap(),
            copy_el:        data.copy_el.unwrap(),
            reactions:      react,
        };
    }
    pub fn get_order(&self) -> UserPostListPosition {
        use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

        let _connection = establish_connection();
        return user_post_list_positions
            .filter(schema::user_post_list_positions::list_id.eq(self.id))
            .filter(schema::user_post_list_positions::types.eq(1))
            .first::<UserPostListPosition>(&_connection)
            .expect("E");
    }
    pub fn add_in_community_collections(&self, community_id: i32) -> i16 {
        use crate::models::NewCommunityPostListPosition;

        if !self.get_communities_ids().iter().any(|&i| i==community_id) && self.community_id.is_some() && self.community_id.unwrap() == community_id {
            return 0;
        }
        let _connection = establish_connection();
        let new_item = NewCommunityPostListCollection {
            community_id: community_id,
            post_list_id: self.id,
        };
        diesel::insert_into(schema::community_post_list_collections::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");

        let new_pos = NewCommunityPostListPosition {
            community_id: community_id,
            list_id:      self.id,
            position:     PostList::get_community_post_lists_new_position(community_id),
            types:        1,
        };
        diesel::insert_into(schema::community_post_list_positions::table)
            .values(&new_pos)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn remove_in_community_collections(&self, community_id: i32) -> i16 {
        use crate::schema::{
            community_post_list_collections::dsl::community_post_list_collections,
            community_post_list_positions::dsl::community_post_list_positions
        };

        if self.get_communities_ids().iter().any(|&i| i==community_id) {
            return 0;
        }
        let _connection = establish_connection();
        diesel::delete (
            community_post_list_collections
                .filter(schema::community_post_list_collections::community_id.eq(community_id))
                .filter(schema::community_post_list_collections::post_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete (
            community_post_list_positions
                .filter(schema::community_post_list_positions::community_id.eq(community_id))
                .filter(schema::community_post_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return 1;
    }

    pub fn add_in_user_collections(&self, user_id: i32) -> i16 {
        use crate::models::NewUserPostListPosition;

        if !self.get_users_ids().iter().any(|&i| i==user_id) && self.user_id == user_id {
            return 0;
        }
        let _connection = establish_connection();
        let new_item = NewUserPostListCollection {
            user_id:      user_id,
            post_list_id: self.id,
        };
        diesel::insert_into(schema::user_post_list_collections::table)
            .values(&new_item)
            .execute(&_connection)
            .expect("Error.");

        let new_pos = NewUserPostListPosition {
            user_id:  user_id,
            list_id:  self.id,
            position: PostList::get_user_post_lists_new_position(user_id),
            types:    1,
        };
        diesel::insert_into(schema::user_post_list_positions::table)
            .values(&new_pos)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn remove_in_user_collections(&self, user_id: i32) -> i16 {
        use crate::schema::{
            user_post_list_collections::dsl::user_post_list_collections,
            user_post_list_positions::dsl::user_post_list_positions
        };

        if self.get_users_ids().iter().any(|&i| i==user_id) {
            return 0;
        }
        let _connection = establish_connection();
        diesel::delete (
            user_post_list_collections
                .filter(schema::user_post_list_collections::user_id.eq(user_id))
                .filter(schema::user_post_list_collections::post_list_id.eq(self.id))
            )
          .execute(&_connection)
          .expect("E");
        diesel::delete (
            user_post_list_positions
                .filter(schema::user_post_list_positions::user_id.eq(user_id))
                .filter(schema::user_post_list_positions::list_id.eq(self.id))
         )
         .execute(&_connection)
         .expect("E");
        return 0;
    }

    pub fn copy_list (
        &self,
        user_id: i32,
        owners: Vec<String>
    ) -> i16 {
        //user_or_communities - список владельцев (c16, u8),
        // в коллекции которыхъ копируется список
        use crate::utils::{
            get_community,
            get_user,
        };

        for item in owners.iter() {
            let first = item.chars().nth(0).unwrap();
            if first == 'c' {
                let c_id: i32 = item[..1].parse().unwrap();
                let community = get_community(c_id).expect("E.");
                if community.get_administrators_ids().iter().any(|&i| i==user_id) {
                    self.add_in_community_collections(c_id);
                }
            }
            else if first == 'u' {
                let u_id: i32 = item[..1].parse().unwrap();
                let owner = get_user(u_id).expect("E.");
                if owner.user_id == user_id {
                    self.add_in_user_collections(u_id);
                }
            }
        }
        return 1;
    }
    pub fn get_posts_ids(&self) -> Vec<i32> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        let fix_list_ids = posts
            .filter(schema::posts::post_list_id.eq(self.id))
            .filter(schema::posts::types.lt(35))
            .select(schema::posts::id)
            .load::<i32>(&_connection)
            .expect("E.");
        return fix_list_ids;
    }
    pub fn get_user_lists(user_pk: i32) -> Vec<PostList> {
        use crate::schema::{
            user_post_list_collections::dsl::user_post_list_collections,
            user_post_list_positions::dsl::user_post_list_positions,
            post_lists::dsl::post_lists,
        };
        let _connection = establish_connection();
        let position_lists = user_post_list_positions
            .filter(schema::user_post_list_positions::user_id.eq(user_pk))
            .filter(schema::user_post_list_positions::types.eq(1))
            .select(schema::user_post_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            return post_lists
                .filter(schema::post_lists::id.eq_any(position_lists))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let user_lists = post_lists
            .filter(schema::post_lists::user_id.eq(user_pk))
            .filter(schema::post_lists::types.lt(10))
            .select(schema::post_lists::id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in user_lists.iter() {
            stack.push(_item);
        };
        let user_collections = user_post_list_collections
            .filter(schema::user_post_list_collections::user_id.eq(user_pk))
            .select(schema::user_post_list_collections::post_list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in user_collections.iter() {
            stack.push(_item);
        };
        return post_lists
            .filter(schema::post_lists::id.eq_any(stack))
            .filter(schema::post_lists::types.lt(31))
            .load::<PostList>(&_connection)
            .expect("E.");
    }
    pub fn get_community_lists(community_pk: i32) -> Vec<PostList> {
        use crate::schema::{
            community_post_list_collections::dsl::community_post_list_collections,
            community_post_list_positions::dsl::community_post_list_positions,
            post_lists::dsl::post_lists,
        };

        let _connection = establish_connection();
        let position_lists = community_post_list_positions
            .filter(schema::community_post_list_positions::community_id.eq(community_pk))
            .filter(schema::community_post_list_positions::types.eq(1))
            .select(schema::community_post_list_positions::list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        if position_lists.len() > 0 {
            return post_lists
                .filter(schema::post_lists::id.eq_any(position_lists))
                .filter(schema::post_lists::types.lt(31))
                .load::<PostList>(&_connection)
                .expect("E.");
        }

        let mut stack = Vec::new();
        let community_lists = post_lists
            .filter(schema::post_lists::community_id.eq(community_pk))
            .filter(schema::post_lists::types.lt(31))
            .select(schema::post_lists::id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in community_lists.iter() {
            stack.push(_item);
        };
        let community_collections = community_post_list_collections
            .filter(schema::community_post_list_collections::community_id.eq(community_pk))
            .select(schema::community_post_list_collections::post_list_id)
            .load::<i32>(&_connection)
            .expect("E.");
        for _item in community_collections.iter() {
            stack.push(_item);
        };
        return post_lists
            .filter(schema::post_lists::id.eq_any(stack))
            .filter(schema::post_lists::types.lt(31))
            .load::<PostList>(&_connection)
            .expect("E.");

    }
    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::post_lists::types.eq(self.types + 80))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_lists(1);
        }
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
            .set(schema::post_lists::types.eq(self.types - 80))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_lists(1);
        }

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_post_list_positions::dsl::community_post_list_positions;

            let list_positions = community_post_list_positions
                .filter(schema::community_post_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_post_list_positions::list_id.eq(self.id))
                .first::<CommunityPostListPosition>(&_connection);
            if list_positions.is_ok() {
                let list_position = list_positions.expect("Error.");
                diesel::update(&list_position)
                  .set(schema::community_post_list_positions::types.eq(2))
                  .execute(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

            let list_positions = user_post_list_positions
                .filter(schema::user_post_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_post_list_positions::list_id.eq(self.id))
                .first::<UserPostListPosition>(&_connection);
            if list_positions.is_ok() {
                let list_position = list_positions.expect("E.");
                diesel::update(&list_position)
                  .set(schema::user_post_list_positions::types.eq(2))
                  .execute(&_connection)
                  .expect("Error.");
            }
        }
        let o_1 = diesel::update(self)
            .set(schema::post_lists::types.eq(self.types + 40))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_lists(1);
        }

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();
        if self.community_id.is_some() {
            use crate::schema::community_post_list_positions::dsl::community_post_list_positions;

            let list_positions = community_post_list_positions
                .filter(schema::community_post_list_positions::community_id.eq(self.community_id.unwrap()))
                .filter(schema::community_post_list_positions::list_id.eq(self.id))
                .load::<CommunityPostListPosition>(&_connection);
            if list_positions.is_ok() {
                let list_position = list_positions.expect("Error.");
                diesel::update(&list_position)
                  .set(schema::community_post_list_positions::types.eq(1))
                  .execute(&_connection)
                  .expect("Error.");
            }
        } else {
            use crate::schema::user_post_list_positions::dsl::user_post_list_positions;

            let list_positions = user_post_list_positions
                .filter(schema::user_post_list_positions::user_id.eq(self.user_id))
                .filter(schema::user_post_list_positions::list_id.eq(self.id))
                .load::<UserPostListPosition>(&_connection);
            if list_positions.is_ok() {
                let list_position = list_positions.expect("E.");
                diesel::update(&list_position)
                  .set(schema::user_post_list_positions::types.eq(1))
                  .execute(&_connection)
                  .expect("Error.");
            }
        }
        let o_1 = diesel::update(self)
            .set(schema::post_lists::types.eq(self.types - 40))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_lists(1);
        }

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
            .set(schema::post_lists::types.eq(self.types + 120))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_lists(1);
        }

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
            .set(schema::post_lists::types.eq(self.types - 120))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_lists(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_lists(1);
        }

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn create_post (
        &self,
        creator: Option<User>,
        community: Option<Community>,
        data: Json<DataNewPost>
    ) -> RespPost {
        use crate::models::NewPost;

        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::post_lists::count.eq(self.count + 1))
            .execute(&_connection)
            .expect("Error.");

        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}
        let _types: i16;
        if community.is_some() {
            community.unwrap().plus_posts(1);
        }
        else if creator.is_some() {
            creator.unwrap().plus_posts(1);
        }
        if self.community_id.is_some() {
            let _community = self.get_community().expect("E.");
            if self.is_anon_user_see_el() && _community.is_anon_user_see_el() {
                _types = 0;
            }
            else {
                _types = 5;
            }
        }
        else {
            let _creator = self.get_creator().expect("E.");;
            if self.is_anon_user_see_el() && _creator.is_anon_user_see_el() {
                _types = 0;
            }
            else {
                _types = 5;
            }
        }

        let new_post_form = NewPost {
          content:      data.content.clone(),
          community_id: self.community_id,
          user_id:      data.user_id.unwrap(),
          post_list_id: self.id,
          types:        _types,
          attach:       data.attachments.clone(),
          comments_on:  data.comments_on,
          created:      chrono::Local::now().naive_utc(),
          comment:      0,
          view:         0,
          repost:       0,
          copy:         0,
          position:     (self.count).try_into().unwrap(),
          is_signature: data.is_signature,
          parent_id:    data.parent_id,
          reactions:    0,
        };
        let new_post = diesel::insert_into(schema::posts::table)
            .values(&new_post_form)
            .get_result::<Post>(&_connection)
            .expect("Error.");

        if data.attachments.is_some() {
            //use crate::models::NewAttachItem;
            //use crate::schema::attach_items::dsl::attach_items;
            //let _attach = data.attachments.as_deref().unwrap().to_string();
            //let v: Vec<&str> = _attach.split(",").collect();
            //for item in v.iter() {
            //    if item.len() > 3 {
            //        let pk: i32 = item[3..].parse().unwrap();
            //        let code = &item[..3];
            //    }
            //}
        }

        return RespPost {
            id:           new_post.id,
            list_id:      self.id,
            user_id:      data.user_id.unwrap(),
            community_id: self.community_id,
            content:      data.content.clone(),
            attach:       data.attachments.clone(),
            comments_on:  data.comments_on,
            is_signature: data.is_signature,
            parent_id:    data.parent_id,
            attachments:  None,
        };
    }
}
