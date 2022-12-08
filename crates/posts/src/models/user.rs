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
    PgConnection,
};

use crate::schema;
use crate::schema::{
    users,
    friends,
    follows,
    user_visible_perms,
};

use crate::models::{Post, PostList, SearchAllComments,};

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
    pub id:             i32,
    pub user_id:        i32,
    pub first_name:     String,
    pub last_name:      String,
    pub types:          i16,
    pub is_man:         bool,
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
    pub see_all:    Option<i16>,
    pub is_man:     Option<i16>,
    pub link:       Option<String>,
    pub s_avatar:   Option<String>,
}

impl User {
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

    pub fn edit_private (
        &self, 
        field:  &str, 
        value:  i16, 
        _users: Option<Vec<AttachOwner>>
    ) -> i16 {
        use crate::schema::item_users::dsl::item_users;
        use crate::models::ItemUser;

        let is_ie_mode = vec![3,4,5,6,9,10,11,12].iter().any(|&i| i==value);
        if value < 1 || value > 13 || (is_ie_mode && _users.is_none()) {
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

            for _user in _users.unwrap().iter() {
                let _new_perm = NewUserVisiblePerm {
                    user_id:   self.user_id,
                    target_id: _user.id,
                    types:     value,
                };
                diesel::insert_into(schema::user_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
                
                ItemUser::check_or_create(_user);
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
        user_id:     i32,
        first_name:  String,
        last_name:   String,
        types:       i16,
        is_man:      bool,
        link:        String,
        s_avatar:    Option<String>,
        see_all:     i16,
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
            link:           link.clone(),
            s_avatar:       s_avatar.clone(),
            last_activity:  chrono::Local::now().naive_utc(),
            see_all:        see_all,
            see_el:         1,
            see_comment:    1,
            create_el:      13,
            create_comment: 12,
            copy_el:        1,
            lists:          0,
            posts:          0,
            comments:       0,
        };
        diesel::insert_into(schema::users::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
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
        let user_types = self.types;
        let _case = match user_types {
            1 => 31,
            6 => 36,
            7 => 37,
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
        let user_types = self.types;
        let _case = match user_types {
            31 => 1,
            36 => 6,
            37 => 7,
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
        let user_types = self.types;
        let _case = match user_types {
            1 => 41,
            6 => 46,
            7 => 47,
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
        let user_types = self.types;
        let _case = match user_types {
            41 => 1,
            46 => 6,
            47 => 7,
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
        let user_types = self.types;
        let _case = match user_types {
            1 => 51,
            6 => 56,
            7 => 57,
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
        let user_types = self.types;
        let _case = match user_types {
            51 => 1,
            56 => 6,
            57 => 7,
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
    pub fn is_suspended(&self) -> bool {
        return self.types < 60 && self.types > 50;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types < 70 && self.types > 60;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types < 40 && self.types > 30;
    }
    pub fn is_closed(&self) -> bool {
        return self.types < 50 && self.types > 40;
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
20 пользователь заблокирован у владельца записей
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
