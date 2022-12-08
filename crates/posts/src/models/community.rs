use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_limit_offset,
    CardPostJson,
    CardUserJson,
    CardPostListJson,
    AttachOwner,
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

use crate::models::{Post, PostList, SearchAllComments};

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
    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _users: Option<Vec<AttachOwner>>
    ) -> i16 { 
        let is_ie_mode = vec![6,7].iter().any(|&i| i==value);
        if value < 1 || value > 7 || (is_ie_mode && _users.is_none()) {
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
        if _users.is_some() && is_ie_mode {
            /*
            это сервис не пользователей, потому мы добавим всех 
            включенных / исключенных пользователей для приватности в таблицу 
            пользователей item_users, чтобы выводить сведения при изменении приватности
            и в других подобных случаях.
            */
            use crate::models::ItemUser;
            for _user in _users.unwrap().iter() {
                let _new_perm = NewCommunityVisiblePerm {
                    community_id: self.community_id,
                    target_id:    _user.id,
                    types:        value,
                };
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
                
                ItemUser::check_or_create(_user);
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
        diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .execute(&_connection)
            .expect("Error.");
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
        return self.types == 3;
    }
    pub fn is_close(&self) -> bool {
        return self.types == 2;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 10;
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
