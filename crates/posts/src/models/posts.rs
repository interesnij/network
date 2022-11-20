use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    PgTextExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    JsonPosition,
    JsonItemReactions,
    CardParentPostJson,
    RepostsPostJson,
    CardPostJson,
    ReactionBlockJson,
    SmallReactionBlockJson,
    PostDetailJson,
    CardUserJson,
    CardOwnerJson,
    CardCommentJson,
    EditPostJson,
    DataEditPost,
    RespPost,
    RespComment,
    AttachPostResp,
};
use actix_web::web::Json;
use crate::models::{
    PostComment, NewPostComment, PostList,
    PostCounterReaction, User, Community,
};
use crate::schema::posts;
use crate::errors::Error;


/////// Post //////
//////////// тип
// 0 Опубликовано
// 5 Опубликовано приватно
// 10 Закрепленый
// 15 Черновик владельца
// 20 Черновик предложки
// 25 Предложка сообщества
// 30 Предложка пользователя
// 35 Родительский пост

// 40 Удаленый Опубликовано
// 45 Удаленый Опубликовано приватно
// 50 Удаленый Закрепленый
// 55 Удаленый Черновик владельца
// 60 Удаленый Черновик предложки
// 65 Удаленый Предложка сообщества
// 70 Удаленый Предложка пользователя
// 75 Удаленый Родительский пост

// 80 Закрытый Опубликовано
// 85 Закрытый Опубликовано приватно
// 90 Закрытый Закрепленый
// 95 Закрытый Черновик владельца
// 100 Закрытый Черновик предложки
// 105 Закрытый Предложка сообщества
// 110 Закрытый Предложка пользователя
// 115 Закрытый Родительский пост

// 120 Удаленый полностью Опубликовано
// 125 Удаленый полностью Опубликовано приватно
// 130 Удаленый полностью Закрепленый

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Post {
    pub id:           i32,
    pub content:      Option<String>,
    pub community_id: Option<i32>,
    pub user_id:      i32,
    pub post_list_id: i32,
    pub types:        i16,
    pub attach:       Option<String>,
    pub comments_on:  bool,
    pub created:      chrono::NaiveDateTime,
    pub comment:      i32,
    pub view:         i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,
    pub is_signature: bool,
    pub parent_id:    Option<i32>,
    pub reactions:    i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub content:      Option<String>,
    pub community_id: Option<i32>,
    pub user_id:      i32,
    pub post_list_id: i32,
    pub types:        i16,
    pub attach:       Option<String>,
    pub comments_on:  bool,
    pub created:      chrono::NaiveDateTime,
    pub comment:      i32,
    pub view:         i32,
    pub repost:       i32,
    pub copy:         i32,
    pub position:     i16,
    pub is_signature: bool,
    pub parent_id:    Option<i32>,
    pub reactions:    i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPost {
    pub content:      Option<String>,
    pub attach:       Option<String>,
    pub comments_on:  bool,
    pub is_signature: bool,
}
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostPosition {
    pub position: i16,
}

pub struct SearchAllPosts {
    pub posts:  Vec<CardPostJson>,
    pub offset: i64,
}

impl Post {
    pub fn get_posts_for_attach(&self, ids: Vec<i32>) -> Vec<AttachPostResp> {
        // выдача инфы для прикрепления записей
        // по запросу API
        use crate::schema::posts::dsl::posts;
        use crate::utils::{
            AttachOwner,
            AttachCommunity,
            AttachList,
            AttachPermList
        };

        let stack: Vec<AttachPostResp> = Vec::new();
        let _connection = establish_connection();
        let post_list = posts
            .filter(schema::posts::id.eq_any(ids))
            .filter(schema::posts::types.lt(10))
            .load::<Post>(&_connection)
            .expect("E.");

        for i in post_list.iter() {
            let mut c_resp: Option<AttachCommunity> = None;
            let mut u_resp: Option<AttachOwner> = None;
            if i.community_id.is_some() {
                let community = i.get_community().expect("E.");
                c_resp = AttachCommunity {
                    id:         community.id,
                    name:       community.name,
                    types:      community.types,
                    link:       community.link,
                    s_avatar:   community.s_avatar,
                    see_member: community.see_member,
                }
            }
            else {
                let creator = i.get_creator().expect("E.");
                c_resp = AttachOwner {
                    id:         creator.id,
                    first_name: creator.first_name,
                    last_name:  creator.last_name,
                    types:      creator.types,
                    link:       creator.link,
                    s_avatar:   creator.s_avatar,
                    see_all:    creator.see_all,
                    see_friend: creator.see_friend,
                }
            }
            let list = i.get_list().expect("E.");
            let list_data = AttachPermList {
                user_id:      list.user_id,
                community_id: list.community_id,
                list_id:      list.id,
                list_types:   20,
                types:        list.types,
                see_el:       list.see_el,
                copy_el:      list.copy_el,
            }
            let data = AttachPost {
                id:             i.id,
                content:        i.content.clone(),
                comments_on:    i.comments_on,
                created:        c.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:        i.comment,
                view:           i.view,
                repost:         i.repost,
                is_signature:   i.is_signature,
                reactions:      i.reactions,
                types:          i.get_code(),                         // например pos1
                parent:         i.get_parent_post_json(),     // пост родитель
                reactions_list: self.get_reactions_json(user_id, list.get_reactions_list()),
                attachments:    i.get_attach(),
            }
            stack.push {
                owner:     u_resp,
                community: c_resp,
                list:      list_data,
                data:      data,
            }
        }
        return stack;
    }
    pub fn get_attach(&self) -> Option<AttachmentsJson> {
        return None;
    }
    pub fn search_posts (
        q:       &String,
        user_id: i32,
        limit:   i64,
        offset:  i64,
    ) -> SearchAllPosts {
        if limit > 100 {
            use crate::schema::posts::dsl::posts;

            let _connection = establish_connection();
            let mut _count = 0;
            let mut _step = 0;
            let mut _offset = offset;

            let mut creator_include: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых можно смотреть посты
            let mut community_include: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых можно смотреть посты
            let mut list_include: Vec<i32> = Vec::new();
            let mut creator_exclude: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых нельзя смотреть посты
            let mut community_exclude: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых можно нельзя посты
            let mut list_exclude: Vec<i32> = Vec::new();      // запишем ids списков, у которых можно нельзя посты
            let mut posts_json = Vec::new();

            while _count < limit {
                _step += limit;

                let items = posts
                    .filter(schema::posts::content.ilike(&q))
                    .filter(schema::posts::types.lt(11))
                    .limit(_step)
                    .offset(_offset)
                    .order(schema::posts::created.desc())
                    .load::<Post>(&_connection)
                    .expect("E.");

                for i in items.iter() {
                    if _count == limit {
                        break;
                    }

                    // проверяем, запрещено ли запрашивающему смотреть
                    // посты пользователя или сообщества или списка
                    if creator_exclude.iter().any(|&a| a==i.user_id)
                        ||
                        (i.community_id.is_some() && community_exclude.iter().any(|&a| a==i.community_id.unwrap()))
                        ||
                        list_exclude.iter().any(|&a| a==i.post_list_id)
                    {
                        continue;
                    }
                    else if list_include.iter().any(|&a| a==i.post_list_id) {
                        _count += 1;
                        let list = i.get_list().expect("E.");
                        posts_json.push ( i.get_post_json(user_id, list.get_reactions_list()) );
                        continue;

                    }

                    let list = i.get_list().expect("E.");

                    if i.community_id.is_some() {
                        // если пост сообщества
                        if community_include.iter().any(|&a| a==i.community_id.unwrap()) {
                            // если id сообщества в разрешенных community_include,
                            if (user_id > 0 && list.is_user_see_el(user_id))
                                ||
                                (user_id == 0 && list.is_anon_user_see_el())
                            {
                                posts_json.push ( i.get_post_json(user_id, list.get_reactions_list()) );
                                _count += 1;
                                list_include.push(i.post_list_id);
                                continue;
                            }
                            else {
                                list_exclude.push(i.post_list_id);
                                continue;
                            }
                        }
                        else {
                            // если id сообщества нет в разрешенных community_include,
                            let community = i.get_community().expect("E.");
                            if (user_id > 0 && community.is_user_see_el(user_id))
                                ||
                                (user_id == 0 && community.is_anon_user_see_el())
                            {
                                community_include.push(community.id);
                                if (user_id > 0 && list.is_user_see_el(user_id))
                                    ||
                                    (user_id == 0 && list.is_anon_user_see_el())
                                {
                                    posts_json.push ( i.get_post_json(user_id, list.get_reactions_list()) );
                                    _count += 1;
                                    list_include.push(i.post_list_id);
                                    continue;
                                }
                                else {
                                    list_exclude.push(i.post_list_id);
                                    continue;
                                }
                            }
                            else {
                                community_exclude.push(i.community_id.unwrap());
                                continue;
                            }
                        }
                    }
                    // если пост пользователя
                    if creator_include.iter().any(|&a| a==i.user_id) {
                        // если id пользователя в разрешенных creator_include,
                        if (user_id > 0 && list.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && list.is_anon_user_see_el())
                        {
                            posts_json.push ( i.get_post_json(user_id, list.get_reactions_list()) );
                            _count += 1;
                            list_include.push(i.post_list_id);
                            continue;
                        }
                        else {
                            list_exclude.push(i.post_list_id);
                            continue;
                        }
                    }
                    else {
                        // если id пользователя нет в разрешенных creator_include,
                        let creator = i.get_creator().expect("E.");
                        if (user_id > 0 && creator.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && creator.is_anon_user_see_el())
                        {
                            creator_include.push(creator.id);
                            if (user_id > 0 && list.is_user_see_el(user_id))
                                ||
                                (user_id == 0 && list.is_anon_user_see_el())
                            {
                                posts_json.push ( i.get_post_json(user_id, list.get_reactions_list()) );
                                _count += 1;
                                list_include.push(i.post_list_id);
                                continue;
                            }
                            else {
                                list_exclude.push(i.post_list_id);
                                continue;
                            }
                        }
                        else {
                            creator_exclude.push(i.user_id);
                            continue;
                        }
                    }
                }
                _offset += limit;
            }
            return SearchAllPosts {
                posts:  posts_json,
                offset: offset,
            };
        }
        else {
            return SearchAllPosts {
                posts:  Vec::new(),
                offset: 0,
            };
        }
    }

    pub fn item_message_reposts_count (
        item_id: i32,
        types: i16
    ) -> usize {
        // получаем кол-во репостов элемента заданного типа
        // Тип объекта types задается единый для всех аналогичных задач .
        use crate::schema::item_reposts::dsl::item_reposts;

        let _connection = establish_connection();
        return item_reposts
            .filter(schema::item_reposts::item_id.eq(item_id))
            .filter(schema::item_reposts::item_types.eq(types))
            .filter(schema::item_reposts::message_id.is_not_null())
            .select(schema::item_reposts::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_item_reposts (
        item_id: i32,
        types: i16,
        limit: i64,
        offset: i64
    ) -> Vec<Post> {
        use crate::schema::{
            item_reposts::dsl::item_reposts,
            posts::dsl::posts,
        };

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }
        let _connection = establish_connection();
        let item_reposts_ids = item_reposts
            .filter(schema::item_reposts::item_id.eq(item_id))
            .filter(schema::item_reposts::item_types.eq(types))
            .order(schema::item_reposts::id.desc())
            .limit(_limit)
            .offset(offset)
            .select(schema::item_reposts::post_id)
            .load::<Option<i32>>(&_connection)
            .expect("E.");
        let mut stack: Vec<i32> = Vec::new();
        for id in item_reposts_ids.iter() {
            stack.push(id.unwrap());
        }
        return posts
            .filter(schema::posts::id.eq_any(stack))
            .filter(schema::posts::types.lt(36))
            .load::<Post>(&_connection)
            .expect("E");
    }

    pub fn get_item_reposts_with_limit (
        item_id: i32,
        types: i16,
        limit: i64,
    ) -> Vec<Post> {
        use crate::schema::{
            item_reposts::dsl::item_reposts,
            posts::dsl::posts,
        };

        let _connection = establish_connection();
        let item_reposts_ids = item_reposts
            .filter(schema::item_reposts::item_id.eq(item_id))
            .filter(schema::item_reposts::item_types.eq(types))
            .order(schema::item_reposts::id.desc())
            .limit(limit)
            .select(schema::item_reposts::post_id)
            .load::<Option<i32>>(&_connection)
            .expect("E.");
        let mut stack: Vec<i32> = Vec::new();
        for id in item_reposts_ids.iter() {
            stack.push(id.unwrap());
        }
        return posts
            .filter(schema::posts::id.eq_any(stack))
            .filter(schema::posts::types.lt(36))
            .load::<Post>(&_connection)
            .expect("E");
    }

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
    }
    pub fn get_comments (
        &self,
        user_id: i32,
        reactions_list: Vec<i32>,
        limit: i64,
        offset: i64,
    ) -> Vec<CardCommentJson> {
        use crate::schema::post_comments::dsl::post_comments;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }
        let _connection = establish_connection();
        let mut json = Vec::new();
        let items = post_comments
            .filter(schema::post_comments::post_id.eq(self.id))
            .filter(schema::post_comments::types.lt(10))
            .filter(schema::post_comments::parent_id.is_null())
            .limit(_limit)
            .offset(offset)
            .load::<PostComment>(&_connection)
            .expect("E.");

        for c in items.iter() {
            let creator = c.get_owner_meta().expect("E");
            json.push (CardCommentJson {
                content:        c.content.clone(),
                owner_name:     creator.name.clone(),
                owner_link:     creator.link.clone(),
                owner_image:    creator.image.clone(),
                created:        c.created.format("%d-%m-%Y в %H:%M").to_string(),
                reactions:      c.reactions,
                types:          c.get_code(),       // например cpo1
                replies:        c.replies,    // кол-во ответов
                reactions_list: c.get_reactions_json(user_id, reactions_list.clone()),
                attachments:    None,
            });
        }
        return json;
    }

    pub fn get_parent_post_json (&self) -> Option<CardParentPostJson> {
        // получаем родительский пост
        let parent: Option<CardParentPostJson>;
        if self.parent_id.is_some() {
            let _parent = self.get_parent().expect("E");
            let creator = _parent.get_owner_meta().expect("E");
            parent = Some(CardParentPostJson {
                id:          _parent.id,
                content:     _parent.content.clone(),
                owner_name:  creator.name.clone(),
                owner_link:  creator.link.clone(),
                owner_image: creator.image.clone(),
                created:     _parent.created.format("%d-%m-%Y в %H:%M").to_string(),
                attachments: None,
            })
        }
        else {
            parent = None;
        }
        return parent;
    }
    pub fn get_item_reposts_with_limit_json (
        item_id: i32,
        types: i16,
        limit: i64,
    ) -> RepostsPostJson {
        // получаем репосты объекта, если есть

        let mut reposts_json = Vec::new();
        for r in Post::get_item_reposts_with_limit(item_id, types, limit).iter() {
            let creator = r.get_owner_meta().expect("E");
            reposts_json.push (
                CardOwnerJson {
                    name:  creator.name.clone(),
                    link:  creator.link.clone(),
                    image: creator.image.clone(),
                }
            );
        }

        let reposts_window = RepostsPostJson {
            message_reposts: Post::item_message_reposts_count(item_id, types),
            creators:        reposts_json,
        };
        return reposts_window;
    }
    pub fn get_reposts_with_limit_json (
        &self,
        limit: i64,
    ) -> RepostsPostJson {
        // получаем репосты записи, если есть

        let mut reposts_json = Vec::new();
        for r in self.get_reposts_with_limit(limit).iter() {
            let creator = r.get_owner_meta().expect("E");
            reposts_json.push (
                CardOwnerJson {
                    name:  creator.name.clone(),
                    link:  creator.link.clone(),
                    image: creator.image.clone(),
                }
            );
        }

        let reposts_window = RepostsPostJson {
            message_reposts: Post::item_message_reposts_count(self.id, 51),
            creators:        reposts_json,
        };
        return reposts_window;
    }

    pub fn get_item_reposts_json (
        item_id: i32,
        types: i16,
        limit: i64,
        offset: i64
    ) -> RepostsPostJson {
        // получаем репосты записи, если есть
        let mut reposts_json = Vec::new();
        for r in Post::get_item_reposts(item_id, types, limit, offset).iter() {
            let creator = r.get_owner_meta().expect("E");
            reposts_json.push (
                CardOwnerJson {
                    name:  creator.name.clone(),
                    link:  creator.link.clone(),
                    image: creator.image.clone(),
                }
            );
        }

        let reposts_window = RepostsPostJson {
            message_reposts: Post::item_message_reposts_count(item_id, types),
            creators:        reposts_json,
        };

        return reposts_window;
    }

    pub fn get_reposts_json (
        &self,
        limit: i64,
        offset: i64
    ) -> RepostsPostJson {
        // получаем репосты записи, если есть

        let mut reposts_json = Vec::new();
        for r in self.get_reposts(limit, offset).iter() {
            let creator = r.get_owner_meta().expect("E");
            reposts_json.push (
                CardOwnerJson {
                    name:  creator.name.clone(),
                    link:  creator.link.clone(),
                    image: creator.image.clone(),
                }
            );
        }

        let reposts_window = RepostsPostJson {
            message_reposts: Post::item_message_reposts_count(self.id, 51),
            creators:        reposts_json,
        };
        return reposts_window;
    }

    pub fn get_reactions_json (&self, user_id: i32, reactions_list: Vec<i32>) -> Option<Vec<SmallReactionBlockJson>> {
        // получаем реакции и отреагировавших
        let reactions_blocks: Option<Vec<SmallReactionBlockJson>>;
        if reactions_list.len() == 0 {
            reactions_blocks = None;
        }
        else {
            let mut reactions_json: Vec<SmallReactionBlockJson> = Vec::new();
            let mut user_reaction = 0;

            if user_id > 0 && self.is_have_user_reaction(user_id) {
                user_reaction = self.get_user_reaction(user_id).expect("E.");
            }

            for reaction in reactions_list.iter() {
                let count = self.get_count_model_for_reaction(*reaction).count;
                if count > 0 {
                    reactions_json.push (
                        SmallReactionBlockJson {
                            count:         count,         // кол-во отреагировавших
                            reaction:      *reaction,     // id реакции
                            user_react_id: user_reaction, // id реакции request_user'а, если он реагировал на этот коммент
                        }
                    );
                }
            }
            reactions_blocks = Some(reactions_json);
        }
        return reactions_blocks;
    }

    pub fn get_edit_data_json(&self) -> EditPostJson {
        return EditPostJson {
            content:      self.content.clone(),
            comments_on:  self.comments_on,
            is_signature: self.is_signature,
            attachments:  None,
        };
    }
    pub fn get_detail_post_json (
        &self,
        user_id: i32,
        limit: i64,
        offset: i64,
    ) -> PostDetailJson {
        let list = self.get_list().expect("E");
        let creator = self.get_owner_meta().expect("E");
        let reactions_list = list.get_reactions_list();

        let mut prev: Option<i32> = None;
        let mut next: Option<i32> = None;
        let _posts = list.get_items();
        for (i, item) in _posts.iter().enumerate().rev() {
            if item.position == self.position {
                if (i + 1) != _posts.len() {
                    prev = Some(_posts[i + 1].id);
                };
                if i != 0 {
                    next = Some(_posts[i - 1].id);
                };
                break;
            }
        };
        if user_id > 0 {
            return PostDetailJson {
                    content:              self.content.clone(),
                    owner_name:           creator.name.clone(),
                    owner_link:           creator.link.clone(),
                    owner_image:          creator.image.clone(),
                    comments_on:          self.comments_on,
                    created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                    comment:              self.comment,
                    view:                 self.view,
                    repost:               self.repost,
                    is_signature:         self.is_signature,
                    reactions:            self.reactions,
                    types:                self.get_code(),
                    parent:               self.get_parent_post_json(),
                    reposts:              self.get_reposts_with_limit_json(limit),
                    reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                    prev:                 prev,
                    next:                 next,
                    is_user_see_comments: list.is_user_see_comment(user_id),
                    is_user_create_comments: list.is_user_create_comment(user_id),
                    comments:             self.get_comments(user_id, reactions_list.clone(), limit, offset),
                    attachments:          None,
                };
        } else {
            return PostDetailJson {
                content:              self.content.clone(),
                owner_name:           creator.name.clone(),
                owner_link:           creator.link.clone(),
                owner_image:          creator.image.clone(),
                comments_on:          self.comments_on,
                created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:              self.comment,
                view:                 self.view,
                repost:               self.repost,
                is_signature:         self.is_signature,
                reactions:            self.reactions,
                types:                self.get_code(),
                parent:               self.get_parent_post_json(),
                reposts:              self.get_reposts_with_limit_json(limit),
                reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                prev:                 prev,
                next:                 next,
                is_user_see_comments: list.is_anon_user_see_comment(),
                is_user_create_comments: false,
                comments:             self.get_comments(user_id, reactions_list.clone(), limit, offset),
                attachments:          None,
            };
        }
    }
    pub fn get_post_json (&self, user_id: i32, reactions_list: Vec<i32>,) -> CardPostJson {
        let creator = self.get_owner_meta().expect("E");
        return CardPostJson {
                id:             self.id,
                content:        self.content.clone(),
                owner_name:     creator.name.clone(),
                owner_link:     creator.link.clone(),
                owner_image:    creator.image.clone(),
                comments_on:    self.comments_on,
                created:        self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:        self.comment,
                view:           self.view,
                repost:         self.repost,
                is_signature:   self.is_signature,
                reactions:      self.reactions,
                types:          self.get_code(),
                parent:         self.get_parent_post_json(),
                reactions_list: self.get_reactions_json(user_id, reactions_list.clone()),
                attachments:    None,
            };
    }

    pub fn get_6_user_of_reaction (
        &self,
        reaction_id: &i32,
        user_reaction: Option<i32>,
    ) -> ReactionBlockJson {
        use crate::schema::{
            post_reactions::dsl::post_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPostJson;

        let _connection = establish_connection();
        let user_ids = post_reactions
            .filter(schema::post_reactions::post_id.eq(self.id))
            .filter(schema::post_reactions::reaction_id.eq(reaction_id))
            .limit(6)
            .select(schema::post_reactions::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(user_ids))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");

        let mut user_json = Vec::new();
        for _item in _users.iter() {
            user_json.push (
                CardReactionPostJson {
                    owner_name:  _item.first_name.clone() + &" ".to_string() + &_item.last_name.clone(),
                    owner_link:  _item.link.clone(),
                    owner_image: _item.image.clone(),
                }
            );
        }
        return ReactionBlockJson {
            count:         self.get_count_model_for_reaction(*reaction_id).count,
            reaction:      *reaction_id,
            users:         user_json,
            user_react_id: user_reaction,
        };
    }
    pub fn get_users_of_reaction (
        &self,
        user_id:     i32,
        reaction_id: i32,
        limit:       i64,
        offset:      i64,
    ) -> ReactionBlockJson {
        use crate::schema::{
            post_reactions::dsl::post_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPostJson;

        let mut user_reaction: Option<i32> = None;
        if self.is_have_user_reaction(user_id) {
            user_reaction = Some(self.get_user_reaction(user_id).expect("E."));
        }

        let _connection = establish_connection();
        let user_ids = post_reactions
            .filter(schema::post_reactions::post_id.eq(self.id))
            .filter(schema::post_reactions::reaction_id.eq(reaction_id))
            .limit(limit)
            .offset(offset)
            .select(schema::post_reactions::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(user_ids))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");

        let mut user_json = Vec::new();
        for _item in _users.iter() {
            user_json.push (
                CardReactionPostJson {
                    owner_name:   _item.first_name.clone() + &" ".to_string() + &_item.last_name.clone(),
                    owner_link:  _item.link.clone(),
                    owner_image: _item.image.clone(),
                }
            );
        }
        return ReactionBlockJson {
                count:         self.get_count_model_for_reaction(reaction_id).count,
                reaction:      reaction_id,
                users:         user_json,
                user_react_id: user_reaction,
        };
    }


    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
    }
    pub fn get_count_model_for_reaction(&self, reaction_id: i32) -> PostCounterReaction {
        use crate::schema::post_counter_reactions::dsl::post_counter_reactions;
        use crate::models::NewPostCounterReaction;

        let _connection = establish_connection();
        let _react_model = post_counter_reactions
            .filter(schema::post_counter_reactions::post_id.eq(self.id))
            .first::<PostCounterReaction>(&_connection);
        if _react_model.is_ok() {
            return _react_model.expect("Error.");
        }
        else {
            let new_react_model = NewPostCounterReaction {
                post_id:     self.id,
                reaction_id: reaction_id,
                count:       0,
            };
            let _react_model = diesel::insert_into(schema::post_counter_reactions::table)
                .values(&new_react_model)
                .get_result::<PostCounterReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }
    pub fn send_reaction (
        &self,
        user_id:     i32,
        reaction_id: i32,
    ) -> JsonItemReactions {
        use crate::schema::post_reactions::dsl::post_reactions;
        use crate::models::{PostReaction, NewPostReaction};

        let _connection = establish_connection();
        let list = self.get_list().expect("E.");
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_count_model_for_reaction(reaction_id);

        if reactions_of_list.iter().any(|&i| i==reaction_id) && list.is_user_see_el(user_id) && list.is_user_see_el(user_id) {
            let vote_ok = post_reactions
                .filter(schema::post_reactions::user_id.eq(user_id))
                .filter(schema::post_reactions::post_id.eq(self.id))
                .first::<PostReaction>(&_connection);

            // если пользователь уже реагировал на товар
            if vote_ok.is_ok() {
                let vote = vote_ok.expect("E");

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction_id == reaction_id {
                    diesel::delete (
                        post_reactions
                            .filter(schema::post_reactions::user_id.eq(user_id))
                            .filter(schema::post_reactions::post_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_count(self.id, user_id, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот товар
                else {
                    diesel::update(&vote)
                        .set(schema::post_reactions::reaction_id.eq(reaction_id))
                        .execute(&_connection)
                        .expect("Error.");

                    react_model.update_count(self.id, user_id, false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewPostReaction {
                    user_id:     user_id,
                    post_id:     self.id,
                    reaction_id: reaction_id,
                };
                diesel::insert_into(schema::post_reactions::table)
                    .values(&new_vote)
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error.");

                react_model.update_count(self.id, user_id, true);
                self.plus_reactions(1, user_id);
            }
        }

        return JsonItemReactions {
            count:     react_model.count,
            reactions: self.reactions,
        };
    }
    pub fn count_reaction_ru(&self, reaction_id: i32) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.get_count_model_for_reaction(reaction_id).count,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn count_reactions_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.reactions,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn is_have_reactions(&self) -> bool {
        return self.reactions > 0;
    }
    pub fn reactions_ids(&self) -> Result<Vec<i32>, Error> {
        use crate::schema::post_reactions::dsl::post_reactions;

        let _connection = establish_connection();
        let votes = post_reactions
            .filter(schema::post_reactions::post_id.eq(self.id))
            .select(schema::post_reactions::user_id)
            .load::<i32>(&_connection)?;
        return Ok(votes);
    }
    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self
            .reactions_ids()
            .expect("E.")
            .iter()
            .any(|&i| i==user_id);
    }
    pub fn get_user_reaction(&self, user_id: i32) -> Result<i32, Error> {
        use crate::schema::post_reactions::dsl::post_reactions;
        let _connection = establish_connection();
        let vote = post_reactions
            .filter(schema::post_reactions::user_id.eq(user_id))
            .filter(schema::post_reactions::post_id.eq(self.id))
            .select(schema::post_reactions::reaction_id)
            .first::<i32>(&_connection)?;
        return Ok(vote);
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "pos".to_string() + &self.get_str_id();
    }
    pub fn get_folder(&self) -> String {
        return "posts".to_string();
    }

    pub fn message_reposts_count(&self) -> usize {
        use crate::schema::item_reposts::dsl::item_reposts;

        let _connection = establish_connection();

        return item_reposts
            .filter(schema::item_reposts::item_id.eq(self.id))
            .filter(schema::item_reposts::item_types.eq(51))
            .filter(schema::item_reposts::message_id.is_not_null())
            .select(schema::item_reposts::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }



    pub fn get_list(&self) -> Result<PostList, Error> {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return Ok(post_lists
            .filter(schema::post_lists::id.eq(self.post_list_id))
            .filter(schema::post_lists::types.lt(31))
            .first::<PostList>(&_connection)?);
    }

    pub fn get_playlist_image(&self) -> String {
        return "/static/images/news_small3.jpg".to_string();
    }

    pub fn create_parent_post (
        community_id: Option<i32>,
        user_id:      i32,
        attach:       Option<String>,
    ) -> Post {
        let _connection = establish_connection();

        let new_post_form = NewPost {
            content:      None,
            community_id: community_id,
            user_id:      user_id,
            post_list_id: 0,
            types:        8,
            attach:       attach,
            comments_on:  false,
            created:      chrono::Local::now().naive_utc(),
            comment:      0,
            view:         0,
            repost:       0,
            copy:         0,
            position:     0,
            is_signature: false,
            parent_id:    None,
            reactions:    0,
        };
        let new_post = diesel::insert_into(schema::posts::table)
            .values(&new_post_form)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return new_post;
    }
    pub fn copy_item (
        &self,
        lists: Vec<i32>,
    ) -> i16 {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = post_lists
                .filter(schema::post_lists::id.eq(list_id))
                .filter(schema::post_lists::types.lt(10))
                .first::<PostList>(&_connection)
                .expect("E");

            let new_post_form = NewPost {
                content:      self.content.clone(),
                community_id: self.community_id,
                user_id:      self.user_id,
                post_list_id: *list_id,
                types:        1,
                attach:       self.attach.clone(),
                comments_on:  self.comments_on,
                created:      chrono::Local::now().naive_utc(),
                comment:      0,
                view:         0,
                repost:       0,
                copy:         0,
                position:     (list.count).try_into().unwrap(),
                is_signature: self.is_signature,
                parent_id:    self.parent_id,
                reactions:    0,
            };
            diesel::insert_into(schema::posts::table)
                .values(&new_post_form)
                .execute(&_connection)
                .expect("Error.");

            diesel::update(&list)
              .set(schema::post_lists::copy.eq(list.count + 1))
              .execute(&_connection)
              .expect("Error.");
        }
        diesel::update(self)
          .set(schema::posts::copy.eq(self.copy + count))
          .execute(&_connection)
          .expect("Error.");
        return 1;
    }

    pub fn edit_post (
        &self,
        data: Json<DataEditPost>
    ) -> RespPost {
        let _connection = establish_connection();
        let edit_post = EditPost {
            content:      data.content.clone(),
            attach:       data.attachments.clone(),
            comments_on:  data.comments_on,
            is_signature: data.is_signature,
        };
        diesel::update(self)
            .set(edit_post)
            .execute(&_connection)
            .expect("Error.");

        return RespPost {
            id:           self.id,
            list_id:      self.post_list_id,
            user_id:      self.user_id,
            community_id: self.community_id,
            content:      data.content.clone(),
            attach:       data.attachments.clone(),
            comments_on:  data.comments_on,
            is_signature: data.is_signature,
            parent_id:    self.parent_id,
            attachments:  None,
        };
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::reactions.eq(self.reactions + count))
            .execute(&_connection)
            .expect("Error.");

        //if self.community_id.is_some() {
        //    use crate::models::{create_community_wall, create_community_notify};

        //    let community = self.get_community().expect("E");
        //    create_community_wall (
        //        &user,
        //        &community,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //    create_community_notify (
        //        &user,
        //        &community,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //}
        //else {
        //    use crate::models::{create_user_wall, create_user_notify};

        //    create_user_wall (
        //        &user,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //    create_user_notify (
        //        &user,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //}
    }
    pub fn minus_reactions(&self, count: i32) -> () {
        //use crate::schema::{
        //    notifications::dsl::notifications,
        //    wall_objects::dsl::wall_objects,
        //};

        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::reactions.eq(self.reactions - count))
            .execute(&_connection)
            .expect("Error.");

        //let _q_standalone = "%".to_owned() + &"отреагировал на запись".to_string() + &"%".to_string();
        //diesel::delete (
        //    notifications
        //        .filter(schema::notifications::types.eq(51))
        //        .filter(schema::notifications::object_id.eq(self.id))
        //        .filter(schema::notifications::verb.ilike(&_q_standalone))
        //    )
        //    .execute(&_connection)
        //    .expect("E");

        //diesel::delete (
        //    wall_objects
        //        .filter(schema::wall_objects::types.eq(51))
        //        .filter(schema::wall_objects::object_id.eq(self.id))
        //        .filter(schema::wall_objects::verb.ilike(&_q_standalone))
        //    )
        //    .execute(&_connection)
        //    .expect("E");
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment - count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn is_open(&self) -> bool {
        return self.types < 11;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 39 || self.types < 80;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 79 || self.types < 120;
    }
    pub fn is_fixed(&self) -> bool {
        return self.types == 10;
    }
    pub fn is_repost(&self) -> bool {
        return self.types == 35;
    }

    pub fn on_comments(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::posts::comments_on.eq(true))
            .execute(&_connection);

        if o_1.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn off_comments(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::posts::comments_on.eq(false))
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
        let o_1 = diesel::update(self)
            .set(schema::posts::types.eq(self.types + 40))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_posts(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_posts(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn restore_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::posts::types.eq(self.types - 40))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_posts(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_posts(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::posts::types.eq(self.types + 80))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_posts(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_posts(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::posts::types.eq(self.types - 80))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_posts(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_posts(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn get_format_text(&self) -> String {
        if self.content.is_some() {
            let unwrap = self.content.as_ref().unwrap();
            let split_unwrap: Vec<&str> = unwrap.split(" ").collect();
            if split_unwrap.len() <= 20 {
                return self.content.as_ref().unwrap().to_string();
            }
            else {
                let mut string = String::new();
                for (i, word) in split_unwrap.iter().enumerate() {
                    if i == 20 {
                        string.push_str("<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>");
                    }
                    string.push_str(word);
                    string.push_str(" ");
                }
                return string;
            }
        } else { return "".to_string(); }
    }

    pub fn count_comments(&self) -> String {
        if self.comment == 0 {
            return "".to_string();
        }
        else {
            return self.comment.to_string();
        }
    }

    pub fn count_reposts(&self) -> String {
        if self.repost == 0 {
            return "".to_string();
        }
        else {
            return self.repost.to_string();
        }
    }
    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }

    pub fn reposts_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.repost,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn is_have_reposts(&self) -> bool {
        return self.repost > 0;
    }

    pub fn user_fixed_post(&self, user: User) -> i16 {
        if user.is_can_fixed_post() {
            let _connection = establish_connection();
            let u = diesel::update(self)
                .set(schema::posts::types.eq(10))
                .execute(&_connection);
            if u.is_ok() {
                return 1;
            }
            return 0;
        }
        return 0;
    }
    pub fn community_fixed_post(&self, community: Community) -> i16 {
        if community.is_can_fixed_post() {
            let _connection = establish_connection();
            let u = diesel::update(self)
                .set(schema::posts::types.eq(10))
                .execute(&_connection);
            if u.is_ok() {
                return 1;
            }
            return 0;
        }
        return 0;
    }
    pub fn unfixed_post(&self) -> i16 {
        let _connection = establish_connection();
        let u = diesel::update(self)
            .set(schema::posts::types.eq(0))
            .execute(&_connection);
        if u.is_ok() {
            return 1;
        }
        return 0;
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let self_attach = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>();
            return "files_".to_string() + &self_attach.len().to_string();
        }
        return "files_0".to_string();
    }

    pub fn get_reposts(&self, limit: i64, offset: i64) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }
        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.lt(40))
            .limit(_limit)
            .offset(offset)
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn get_reposts_with_limit(&self, limit: i64) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.lt(40))
            .limit(limit)
            .load::<Post>(&_connection)
            .expect("E");
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> () {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = posts
                .filter(schema::posts::id.eq(i.key))
                .filter(schema::posts::types.lt(10))
                .first::<Post>(&_connection);
            if item.is_ok() {
                diesel::update(&item.expect("E."))
                    .set(schema::posts::position.eq(i.value))
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
    }

    pub fn create_comment (
        &self,
        user_id:      i32,
        community_id: Option<i32>,
        content:      Option<String>,
        parent_id:    Option<i32>,
        attachments:  Option<String>,
    ) -> RespComment {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + 1))
            .execute(&_connection)
            .expect("Error.");

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}
        let _types: i16;
        let list = self.get_list().expect("E");
        if list.community_id.is_some() {
            let _community = list.get_community().expect("E");
            if list.is_anon_user_see_el() && _community.is_anon_user_see_el() {
                _types = 0;
            }
            else {
                _types = 5;
            }
        }
        else {
            let _creator = list.get_creator().expect("E.");
            if list.is_anon_user_see_el() && _creator.is_anon_user_see_el() {
                _types = 0;
            }
            else {
                _types = 5;
            }
        }
        if community_id.is_some() {
            use crate::utils::get_community;
            get_community(community_id.unwrap())
                .expect("E.")
                .plus_posts(1);
        }
        else {
            use crate::utils::get_user;
            get_user(user_id)
                .expect("E.")
                .plus_posts(1);
        }
        let new_comment_form = NewPostComment {
            post_id:      self.id,
            user_id:      user_id,
            community_id: community_id,
            parent_id:    parent_id,
            content:      content.clone(),
            attach:       attachments.clone(),
            types:        _types,
            created:      chrono::Local::now().naive_utc(),
            repost:       0,
            reactions:    0,
            replies:      0,
        };
        let new_comment = diesel::insert_into(schema::post_comments::table)
            .values(&new_comment_form)
            .get_result::<PostComment>(&_connection)
            .expect("Error.");


        return RespComment {
            id:           new_comment.id,
            post_id:      self.id,
            user_id:      user_id,
            community_id: community_id,
            content:      content.clone(),
            parent_id:    parent_id,
            attachments:  None,
        };
    }
    pub fn get_parent(&self) -> Result<Post, Error> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return Ok(posts
            .filter(schema::posts::id.eq(self.parent_id.unwrap()))
            .filter(schema::posts::types.lt(31))
            .first::<Post>(&_connection)?);
    }
}
