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
    JsonItemReactions,
    CardUserJson,
    CardOwnerJson,
    CardCommentJson,
    CardReplyJson,
    ReactionBlockJson,
    SmallReactionBlockJson,
    RespComment,
    AttachmentsJson,
    AttachPostComment,
    AttachPostCommentResp,
};

use crate::models::{
    Post, User, Community, PostList,
    PostCommentCounterReaction,
};
use crate::schema::post_comments;
use crate::errors::Error;


/////// PostComment //////

/////// PostComment //////
// 0 Опубликованный
// 2 Изменённый
// 5 Опубликованный приватный
// 10 Удаленый
// 12 Изменённый Удаленый
// 15 Удаленый приватный
// 21 Закрытый
// 22 Изменённый Закрытый
// 26 приватный Закрытый

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostComment {
    pub id:           i32,
    pub post_id:      i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub parent_id:    Option<i32>,
    pub content:      Option<String>,
    pub attach:       Option<String>,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
    pub repost:       i32,
    pub reactions:    i32,
    pub replies:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comments"]
pub struct NewPostComment {
    pub post_id:      i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub parent_id:    Option<i32>,
    pub content:      Option<String>,
    pub attach:       Option<String>,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
    pub repost:       i32,
    pub reactions:    i32,
    pub replies:      i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="post_comments"]
pub struct EditComment {
    pub content: Option<String>,
    pub attach:  Option<String>,
}

pub struct SearchAllComments {
    pub comments: Vec<CardCommentJson>,
    pub offset:   i64,
}
impl PostComment {
    pub fn get_comments_for_attach(&self, ids: Vec<i32>) -> Vec<AttachCommentResp> {
        // выдача инфы для прикрепления комментов
        // по запросу API
        use crate::schema::post_comments::dsl::post_comments;
        use crate::utils::{
            AttachOwner,
            AttachCommunity,
            AttachList,
            AttachPermList
        };

        let stack: Vec<AttachCommentResp> = Vec::new();
        let _connection = establish_connection();
        let comments = post_comments
            .filter(schema::post_comments::id.eq_any(ids))
            .filter(schema::post_comments::types.lt(10))
            .load::<PostComment>(&_connection)
            .expect("E.");

        for i in comments.iter() {
            let mut c_resp: Option<AttachCommunity> = None;
            let mut u_resp: Option<AttachOwner> = None;

            let list = i.get_list();
            if list.community_id.is_some() {
                let community = list.get_community().expect("E.");
                c_resp = Some(AttachCommunity {
                    id:         community.id,
                    name:       community.name,
                    types:      community.types,
                    link:       community.link,
                    s_avatar:   community.s_avatar,
                    see_member: community.see_member,
                })
            }
            else {
                let creator = list.get_creator().expect("E.");
                u_resp = Some(AttachOwner {
                    id:         creator.id,
                    first_name: creator.first_name,
                    last_name:  creator.last_name,
                    types:      creator.types,
                    link:       creator.link,
                    s_avatar:   creator.s_avatar,
                    see_all:    creator.see_all,
                    see_friend: creator.see_friend,
                })
            }

            let list_data = Some(AttachPermList {
                user_id:      list.user_id,
                community_id: list.community_id,
                list_id:      list.id,
                list_types:   20,
                types:        list.types,
                see_el:       list.see_el,
                copy_el:      list.copy_el,
            });
            let data = AttachPostComment {
                id:             i.id,
                content:        i.content.clone(),
                attachments:    i.get_attach(),
                types:          i.types,
                created:        i.created,
                repost:         i.repost,
                reactions:      i.reactions,
            };
            stack.push (AttachCommentResp{
                owner:     u_resp,
                community: c_resp,
                list:      list_data,
                data:      data,
            })
        }
        return stack;
    }

    pub fn get_attach(&self) -> Option<AttachmentsJson> {
        return None;
    }

    pub fn search_comments (
        q:       &String,
        user_id: i32,
        limit:   i64,
        offset:  i64,
    ) -> SearchAllComments {
        if limit > 100 {
            use crate::schema::post_comments::dsl::post_comments;

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
            let mut list_json = Vec::new();

            while _count < limit {
                _step += limit;

                let items = post_comments
                    .filter(schema::post_comments::content.ilike(&q))
                    .filter(schema::post_comments::types.lt(10))
                    .limit(_step)
                    .offset(_offset)
                    .order(schema::post_comments::created.desc())
                    .load::<PostComment>(&_connection)
                    .expect("E.");

                for i in items.iter() {
                    if _count == limit {
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
                _offset += limit;
            }
            return SearchAllComments {
                comments: list_json,
                offset:   offset,
            };
        }
        else {
            return SearchAllComments {
                comments: Vec::new(),
                offset:   0,
            };
        }
    }

    pub fn get_6_user_of_reaction (
        &self,
        reaction_id:   &i32,
        user_reaction: Option<i32>,
    ) -> ReactionBlockJson {
        use crate::schema::{
            post_comment_reactions::dsl::post_comment_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPostJson;

        let _connection = establish_connection();
        let user_ids = post_comment_reactions
            .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
            .filter(schema::post_comment_reactions::reaction_id.eq(reaction_id))
            .limit(6)
            .select(schema::post_comment_reactions::user_id)
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
                    owner_name:       _item.first_name.clone() + &" ".to_string() + &_item.last_name.clone(),
                    owner_link:       _item.link.clone(),
                    owner_image:      _item.image.clone(),
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
            post_comment_reactions::dsl::post_comment_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPostJson;

        let mut user_reaction: Option<i32> = None;
        if self.is_have_user_reaction(user_id) {
            user_reaction = Some(self.get_user_reaction(user_id).expect("E."));
        }

        let _connection = establish_connection();
        let user_ids = post_comment_reactions
            .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
            .filter(schema::post_comment_reactions::reaction_id.eq(reaction_id))
            .limit(limit)
            .offset(offset)
            .select(schema::post_comment_reactions::user_id)
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
            count:         self.get_count_model_for_reaction(reaction_id).count,
            reaction:      reaction_id,
            users:         user_json,
            user_react_id: user_reaction,
        };
    }

    pub fn get_reactions_json (&self, user_id: i32, reactions_list: Vec<i32>) -> Option<Vec<SmallReactionBlockJson>> {
        // получаем реакции
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

    pub fn get_comment_json (&self, user_id: i32, reactions_list: Vec<i32>) -> CardCommentJson {
        let creator = self.get_owner_meta().expect("E");
        let card = CardCommentJson {
            content:        self.content.clone(),
            owner_name:     creator.name,
            owner_link:     creator.link.clone(),
            owner_image:    creator.image.clone(),
            created:        self.created.format("%d-%m-%Y в %H:%M").to_string(),
            reactions:      self.reactions,
            types:          self.get_code(),
            replies:        self.replies,
            reactions_list: self.get_reactions_json(user_id, reactions_list.clone()),
            attachments:    None,
        };
        return card;
    }
    pub fn get_reply_json (&self, user_id: i32, reactions_list: Vec<i32>) -> CardReplyJson {
        let creator = self.get_owner_meta().expect("E");
        let card = CardReplyJson {
            content:        self.content.clone(),
            owner_name:     creator.name.clone(),
            owner_link:     creator.link.clone(),
            owner_image:    creator.image.clone(),
            created:        self.created.format("%d-%m-%Y в %H:%M").to_string(),
            reactions:      self.reactions,
            types:          self.get_code(),
            reactions_list: self.get_reactions_json(user_id, reactions_list.clone()),
            attachments:    None,
        };
        return card;
    }

    pub fn get_replies(&self, limit: i64, offset: i64) -> Result<Vec<PostComment>,Error>  {
        use crate::schema::post_comments::dsl::post_comments;

        let _limit: i64;
        if limit > 100 {
            _limit = 20;
        }
        else {
            _limit = limit;
        }
        let _connection = establish_connection();
        return Ok(post_comments
            .filter(schema::post_comments::parent_id.eq(self.id))
            .filter(schema::post_comments::types.lt(10))
            .limit(_limit)
            .offset(offset)
            .load::<PostComment>(&_connection)?);
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 39 && self.types < 80;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 79 && self.types < 120;
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post_comment(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "cpo".to_string() + &self.get_str_id();
    }

    pub fn get_item(&self) -> Result<Post, Error> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return Ok(posts
            .filter(schema::posts::id.eq(self.post_id))
            .filter(schema::posts::types.lt(31))
            .first::<Post>(&_connection)?);
    }
    pub fn get_list(&self) -> PostList {
        return self
            .get_item()
            .expect("E")
            .get_list()
            .expect("E");
    }
    pub fn get_parent(&self) -> Result<PostComment, Error> {
        use crate::schema::post_comments::dsl::post_comments;

        let _connection = establish_connection();
        return Ok(post_comments
            .filter(schema::post_comments::id.eq(self.parent_id.unwrap()))
            .filter(schema::post_comments::types.lt(10))
            .first::<PostComment>(&_connection)?);
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

    pub fn get_manager_type(&self) -> i16 {
        if self.parent_id.is_some() {
            return 87;
        }
        else {
            return 81;
        }
    }

    pub fn count_replies(&self) -> i32 {
        return self.replies;
    }
    pub fn count_replies_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.replies,
            " ответ".to_string(),
            " ответа".to_string(),
            " ответов".to_string(),
        );
    }
    pub fn close_item(&self) -> i16 {
        let _connection = establish_connection();
        let item = self.get_item().expect("E");
        let o_1 = diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment - 1))
            .execute(&_connection);

        let o_2 = diesel::update(self)
            .set(schema::post_comments::types.eq(self.types + 20))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_comments(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_comments(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn unclose_item(&self) -> i16 {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let item = self.get_item().expect("E");
        let o_1 = diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment + 1))
            .execute(&_connection);

        let o_2 = diesel::update(self)
            .set(schema::post_comments::types.eq(self.types - 20))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_comments(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_comments(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }

    pub fn delete_item(&self) -> i16 {
        let _connection = establish_connection();
        let item = self.get_item().expect("E");
        let o_1 = diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment - 1))
            .execute(&_connection);

        let o_2 = diesel::update(self)
            .set(schema::post_comments::types.eq(self.types + 10))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_comments(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_comments(1);
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
        let item = self.get_item().expect("E");
        let o_1 = diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment + 1))
            .execute(&_connection);

        let o_2 = diesel::update(self)
            .set(schema::post_comments::types.eq(self.types - 10))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_comments(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_comments(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let length = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>().len();
            if length == 1 {
                return "files_one".to_string();
            }
            else if length == 2 {
                return "files_two".to_string();
            }
        }
        return "files_null".to_string();
    }
    pub fn get_format_text(&self) -> String {
        if self.content.is_some() {
            let unwrap = self.content.as_ref().unwrap();
            if unwrap.len() <= 101 {
                return self.content.as_ref().unwrap().to_string();
            }
            else {
                let new_str = unwrap[..100].to_owned() + &"<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>" + &unwrap[101..] + &"</span>";
                return new_str;
            }
        } else { return "".to_string(); }
    }

    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
    }

    pub fn get_count_model_for_reaction(&self, reaction_id: i32) -> PostCommentCounterReaction {
        use crate::schema::post_comment_counter_reactions::dsl::post_comment_counter_reactions;
        use crate::models::NewPostCommentCounterReaction;

        let _connection = establish_connection();
        let _react_model = post_comment_counter_reactions
            .filter(schema::post_comment_counter_reactions::post_comment_id.eq(self.id))
            .first::<PostCommentCounterReaction>(&_connection);
        if _react_model.is_ok() {
            return _react_model.expect("Error.");
        }
        else {
            let new_react_model = NewPostCommentCounterReaction {
                post_comment_id: self.id,
                reaction_id:  reaction_id,
                count:  0,
            };
            let _react_model = diesel::insert_into(schema::post_comment_counter_reactions::table)
                .values(&new_react_model)
                .get_result::<PostCommentCounterReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction (
        &self,
        user_id:     i32,
        reaction_id: i32,
    ) -> JsonItemReactions {
        use crate::schema::post_comment_reactions::dsl::post_comment_reactions;
        use crate::models::{PostCommentReaction, NewPostCommentReaction};

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_count_model_for_reaction(reaction_id);

        if reactions_of_list.iter().any(|&i| i==reaction_id) && list.is_user_see_el(user_id) && list.is_user_see_comment(user_id) {
            let vote_ok = post_comment_reactions
                .filter(schema::post_comment_reactions::user_id.eq(user_id))
                .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
                .first::<PostCommentReaction>(&_connection);

            // если пользователь уже реагировал на товар
            if vote_ok.is_ok() {
                let vote = vote_ok.expect("E");

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction_id == reaction_id {
                    diesel::delete(post_comment_reactions
                        .filter(schema::post_comment_reactions::user_id.eq(user_id))
                        .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_count(self.id, user_id, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот товар
                else {
                    diesel::update(&vote)
                        .set(schema::post_comment_reactions::reaction_id.eq(reaction_id))
                        .execute(&_connection)
                        .expect("Error.");

                    react_model.update_count(self.id, user_id, false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewPostCommentReaction {
                    user_id:         user_id,
                    post_comment_id: self.id,
                    reaction_id:     reaction_id,
                };
                diesel::insert_into(schema::post_comment_reactions::table)
                    .values(&new_vote)
                    .execute(&_connection)
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
        use crate::schema::post_comment_reactions::dsl::post_comment_reactions;

        let _connection = establish_connection();
        let votes = post_comment_reactions
            .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
            .select(schema::post_comment_reactions::user_id)
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
        use crate::schema::post_comment_reactions::dsl::post_comment_reactions;
        let _connection = establish_connection();
        let vote = post_comment_reactions
            .filter(schema::post_comment_reactions::user_id.eq(user_id))
            .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
            .select(schema::post_comment_reactions::reaction_id)
            .first::<i32>(&_connection)?;
        return Ok(vote);
    }

    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::post_comments::reactions.eq(self.reactions + count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus_reactions(&self, count: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::post_comments::reactions.eq(self.reactions - count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn get_small_content(&self) -> String {
        if self.content.is_some() {
            let _content = self.content.as_deref().unwrap();
            if _content.len() > 50 {
                return _content[..50].to_string();
            }
            else {
                return _content.to_string();
            }
        }
        else {
            return "".to_string();
        }
    }
    pub fn edit_comment (
        &self,
        content: Option<String>,
        attachments: Option<String>
    ) -> RespComment {
        let _connection = establish_connection();
        let edit_post = EditComment {
            content: content.clone(),
            attach:  attachments.clone(),
        };
        diesel::update(self)
            .set(edit_post)
            .execute(&_connection)
            .expect("Error.");

        return RespComment {
            id:           self.id,
            post_id:      self.post_id,
            user_id:      self.user_id,
            community_id: self.community_id,
            content:      content.clone(),
            parent_id:    self.parent_id,
            attachments:  None,
        };
    }
}
