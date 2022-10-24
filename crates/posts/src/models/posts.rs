use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
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
    PostDetailJson,
    CardUserJson,
    CardOwnerJson,
    CommentsSmallJson,
    CardCommentJson,
};
use actix_web::web::Json;
use crate::models::{
    PostComment, NewPostComment,
    PostList, PostRepost,
    PostCounterReaction, User, Community,
};
use crate::schema::posts;

/////// Post //////

//////////// тип
// 1 Опубликовано
// 2 Закрепленый
// 3 Черновик владельца
// 4 Черновик предложки
// 5 Предложка сообщества
// 6 Предложка пользователя
// 7 Родительский пост

// 11 Удаленый Опубликовано
// 12 Удаленый Закрепленый
// 13 Удаленый Черновик владельца
// 14 Удаленый Черновик предложки
// 15 Удаленый Предложка сообщества
// 16 Удаленый Предложка пользователя
// 17 Удаленый Родительский пост

// 21 Закрытый Опубликовано
// 22 Удаленый Закрепленый
// 23 Удаленый Черновик владельца
// 24 Удаленый Черновик предложки
// 25 Удаленый Предложка сообщества
// 26 Удаленый Предложка пользователя
// 27 Удаленый Родительский пост


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Post {
    pub id:              i32,
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub post_list_id:    i32,
    pub types:           i16,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,
    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub is_signature:    bool,
    pub parent_id:       Option<i32>,
    pub reactions:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub post_list_id:    i32,
    pub types:           i16,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,
    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub is_signature:    bool,
    pub parent_id:       Option<i32>,
    pub reactions:       i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPost {
    pub content:         Option<String>,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub is_signature:    bool,
}
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostPosition {
    pub position: i16,
}

impl Post {
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
    pub fn get_comments (
        &self,
        limit: i64,
        offset: i64,
        user_id: i32,
        reactions_list: Vec<i32>,
    ) -> Vec<CardCommentJson> {
        use crate::schema::post_comments::dsl::post_comments;

        let _connection = establish_connection();
        let mut json = Vec::new();
        let items = post_comments
            .filter(schema::post_comments::post_id.eq(self.id))
            .filter(schema::post_comments::types.lt(10))
            .filter(schema::post_comments::parent_id.is_null())
            .limit(limit)
            .offset(offset)
            .load::<PostComment>(&_connection)
            .expect("E.");

        for c in items.iter() {
            let creator = c.get_owner_meta();
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
                items:          None,
            });
        }
        return json;
    }
    pub fn get_comments_post_json (
        &self,
        user_id: i32,
        reactions_list: Vec<i32>,
        page: i32,
        limit: i32
    ) -> CommentsSmallJson {
        let mut next_page_number = 0;
        let have_next: i32;
        let comments: Vec<CardCommentJson>;

        if page > 1 {
            have_next = page * limit + 1;
            comments = self.get_comments (
                limit.into(),
                ((page - 1) * limit).into(),
                user_id,
                reactions_list,
            );
        }
        else {
            have_next = limit + 1;
            comments = self.get_comments (
                limit.into(),
                0,
                user_id,
                reactions_list,
            );
        }
        if self.get_comments(1, have_next.into(), user_id, Vec::new()).len() > 0 {
            next_page_number = page + 1;
        }

        return CommentsSmallJson {
            comments:  comments,
            next_page: next_page_number,
        };
    }

    pub fn get_parent_post_json (&self) -> Option<CardParentPostJson> {
        // получаем родительский пост
        let parent: Option<CardParentPostJson>;
        if self.parent_id.is_some() {
            let _parent = self.get_parent();
            let creator = _parent.get_owner_meta();
            parent = Some(CardParentPostJson {
                id:          _parent.id,
                content:     _parent.content.clone(),
                owner_name:  creator.name.clone(),
                owner_link:  creator.link.clone(),
                owner_image: creator.image.clone(),
                created:     _parent.created.format("%d-%m-%Y в %H:%M").to_string(),
                items:       None,
            })
        }
        else {
            parent = None;
        }
        return parent;
    }
    pub fn get_6_reposts_post_json (&self) -> Option<RepostsPostJson> {
        // получаем репосты записи, если есть
        let reposts_window: Option<RepostsPostJson>;
        if self.repost > 0 {
            let mut reposts_json = Vec::new();
            for r in self.window_reposts().iter() {
                let creator = r.get_owner_meta();
                reposts_json.push (
                    CardOwnerJson {
                        name:  creator.name.clone(),
                        link:  creator.link.clone(),
                        image: creator.image.clone(),
                    }
                );
            }

            reposts_window = Some(RepostsPostJson {
                message_reposts: self.message_reposts_count(),
                copy_count:      self.count_copy(),
                posts:           reposts_json,
            });
        }
        else {
            reposts_window = None;
        }
        return reposts_window;
    }
    pub fn get_reposts_post_json (&self, limit: i64, offset: i64) -> Option<RepostsPostJson> {
        // получаем репосты записи, если есть
        let reposts_window: Option<RepostsPostJson>;
        if self.repost > 0 {
            let mut reposts_json = Vec::new();
            for r in self.reposts(limit.into(), offset).iter() {
                let creator = r.get_owner_meta();
                reposts_json.push (
                    CardOwnerJson {
                        name:  creator.name.clone(),
                        link:  creator.link.clone(),
                        image: creator.image.clone(),
                    }
                );
            }

            reposts_window = Some(RepostsPostJson {
                message_reposts: self.message_reposts_count(),
                copy_count:      self.count_copy(),
                posts:           reposts_json,
            });
        }
        else {
            reposts_window = None;
        }
        return reposts_window;
    }

    pub fn get_reactions_json (&self, user_id: i32, reactions_list: Vec<i32>) -> Option<Vec<ReactionBlockJson>> {
        // получаем реакции и отреагировавших
        let reactions_blocks: Option<Vec<ReactionBlockJson>>;
        if reactions_list.len() == 0 {
            reactions_blocks = None;
        }
        else {
            let mut reactions_json: Vec<ReactionBlockJson> = Vec::new();
            let mut user_reaction = 0;

            if self.is_have_user_reaction(user_id) {
                user_reaction = self.get_user_reaction(user_id);
            }

            for reaction in reactions_list.iter() {
                let count = self.get_count_model_for_reaction(*reaction).count;
                if count > 0 {
                    reactions_json.push(self.get_6_user_of_reaction(reaction, Some(user_reaction)));
                }
            }
            reactions_blocks = Some(reactions_json);
        }
        return reactions_blocks;
    }

    pub fn get_detail_post_json (&self, user_id: i32, page: i32, limit: i32) -> PostDetailJson {
        let list = self.get_list();
        let creator = self.get_owner_meta();
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
        return PostDetailJson {
                content:              self.content.clone(),
                owner_name:           creator.name.clone(),
                owner_link:           creator.link.clone(),
                owner_image:          creator.image.clone(),
                comment_enabled:      self.comment_enabled,
                created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:              self.comment,
                view:                 self.view,
                repost:               self.repost,
                is_signature:         self.is_signature,
                reactions:            self.reactions,
                types:                self.get_code(),
                parent:               self.get_parent_post_json(),
                reposts:              self.get_6_reposts_post_json(),
                reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                prev:                 prev,
                next:                 next,
                is_user_see_comments: list.is_user_see_comment(user_id),
                is_user_create_el:    list.is_user_create_el(user_id),
                comments:             self.get_comments_post_json(user_id, reactions_list.clone(), page, limit.into()),
                items:                None,
            };
    }
    pub fn get_post_json (&self, user_id: i32, reactions_list: Vec<i32>,) -> CardPostJson {
        let creator = self.get_owner_meta();
        return CardPostJson {
                id:              self.id,
                content:         self.content.clone(),
                owner_name:      creator.name.clone(),
                owner_link:      creator.link.clone(),
                owner_image:     creator.image.clone(),
                comment_enabled: self.comment_enabled,
                created:         self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:         self.comment,
                view:            self.view,
                repost:          self.repost,
                is_signature:    self.is_signature,
                reactions:       self.reactions,
                types:           self.get_code(),
                parent:          self.get_parent_post_json(),
                reposts:         self.get_6_reposts_post_json(),
                reactions_list:  self.get_reactions_json(user_id, reactions_list),
                items:           None,
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
                    owner_name:        _item.first_name.clone() + &" ".to_string() + &_item.last_name.clone(),
                    owner_link:        _item.link.clone(),
                    owner_image:       _item.image.clone(),
                    is_user_reaction: &user_reaction.unwrap() == reaction_id,
                }
            );
        }
        return ReactionBlockJson {
                count:    self.get_count_model_for_reaction(*reaction_id).count,
                reaction: *reaction_id,
                users:    user_json,
            };
    }
    pub fn get_users_of_reaction (
        &self,
        reaction_id:   &i32,
        user_reaction: Option<i32>,
        limit:         i64,
        offset:        i64,
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
                    owner_name:       _item.first_name.clone() + &" ".to_string() + &_item.last_name.clone(),
                    owner_link:       _item.link.clone(),
                    owner_image:      _item.image.clone(),
                    is_user_reaction: &user_reaction.unwrap() == reaction_id,
                }
            );
        }
        return ReactionBlockJson {
                count:    self.get_count_model_for_reaction(*reaction_id).count,
                reaction: *reaction_id,
                users:    user_json,
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
            .load::<PostCounterReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
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
        user_id: i32,
        reaction_id: i32,
    ) -> Json<JsonItemReactions> {
        use crate::schema::post_reactions::dsl::post_reactions;
        use crate::models::{PostReaction, NewPostReaction};

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_count_model_for_reaction(reaction_id);

        if reactions_of_list.iter().any(|&i| i==reaction_id) && list.is_user_see_el(user_id) && list.is_user_see_comment(user_id) {
            let votes = post_reactions
                .filter(schema::post_reactions::user_id.eq(user_id))
                .filter(schema::post_reactions::post_id.eq(self.id))
                .load::<PostReaction>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction_id == reaction_id {
                    diesel::delete(post_reactions
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
                        .get_result::<PostReaction>(&_connection)
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

        return Json(JsonItemReactions {
            count:     react_model.count,
            reactions: self.reactions,
        });
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
    pub fn reactions_ids(&self) -> Vec<i32> {
        use crate::schema::post_reactions::dsl::post_reactions;

        let _connection = establish_connection();
        let votes = post_reactions
            .filter(schema::post_reactions::post_id.eq(self.id))
            .select(schema::post_reactions::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return votes;
    }
    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self.reactions_ids().iter().any(|&i| i==user_id);
    }
    pub fn get_user_reaction(&self, user_id: i32) -> i32 {
        use crate::schema::post_reactions::dsl::post_reactions;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = post_reactions
            .filter(schema::post_reactions::user_id.eq(user_id))
            .filter(schema::post_reactions::post_id.eq(self.id))
            .select(schema::post_reactions::reaction_id)
            .load::<i32>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
        return vote;
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

    pub fn message_reposts_count(&self) -> String {
        use crate::schema::post_reposts::dsl::post_reposts;

        let _connection = establish_connection();

        let count = post_reposts
            .filter(schema::post_reposts::post_id.eq(self.id))
            .load::<PostRepost>(&_connection)
            .expect("E.")
            .len();

        if count == 0 {
            return "".to_string();
        }
        else {
            return ", из них в сообщениях - ".to_string() + &count.to_string();
        }
    }



    pub fn get_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::id.eq(self.post_list_id))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
            content:         None,
            community_id:    community_id,
            user_id:         user_id,
            post_list_id:    0,
            types:           8,
            attach:          attach,
            comment_enabled: false,
            created:         chrono::Local::now().naive_utc(),
            comment:         0,
            view:            0,
            repost:          0,
            copy:            0,
            position:        0,
            is_signature:    false,
            parent_id:       None,
            reactions:       0,
        };
        let new_post = diesel::insert_into(schema::posts::table)
            .values(&new_post_form)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return new_post;
    }
    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::posts::dsl::posts;
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let item = posts
            .filter(schema::posts::id.eq(pk))
            .filter(schema::posts::types.eq_any(vec![1, 2]))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = post_lists
                .filter(schema::post_lists::id.eq(list_id))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            list.create_post (
                item.content.clone(),
                list.user_id,
                None,
                item.attach.clone(),
                item.comment_enabled.clone(),
                item.is_signature.clone(),
                item.parent_id.clone(),
            );
        }
        diesel::update(&item)
          .set(schema::posts::copy.eq(item.copy + count))
          .get_result::<Post>(&_connection)
          .expect("Error.");

        if item.community_id.is_some() {
            let community = item.get_community();
            community.plus_posts(count);
        }
        else {
            let creator = item.get_creator();
            creator.plus_posts(count);
         }
        return true;
    }

    pub fn edit_post (
        &self,
        content: Option<String>,
        attach: Option<String>,
        comment_enabled: bool,
        is_signature: bool
    ) -> &Post {
        let _connection = establish_connection();

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}

        let edit_post = EditPost {
            content:         content,
            attach:          attach,
            comment_enabled: comment_enabled,
            is_signature:    is_signature,
        };
        diesel::update(self)
            .set(edit_post)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return self;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::reactions.eq(self.reactions + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");

        //if self.community_id.is_some() {
        //    use crate::models::{create_community_wall, create_community_notify};

        //    let community = self.get_community();
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
            .get_result::<Post>(&_connection)
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
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn is_open(&self) -> bool {
        return self.types == 1 && self.types == 2;
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 10 || self.types < 20;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 20 || self.types < 30;
    }
    pub fn is_fixed(&self) -> bool {
        return self.types == 2;
    }
    pub fn is_repost(&self) -> bool {
        return self.types == 8;
    }

    pub fn delete_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let close_case = match self.types {
            1 => 11,
            2 => 12,
            3 => 13,
            4 => 14,
            5 => 15,
            6 => 16,
            7 => 17,
            8 => 18,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.minus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.minus_posts(1);
        //}

        //hide_wall_notify_items(51, self.id);
    }
    pub fn restore_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let close_case = match self.types {
            11 => 1,
            12 => 2,
            13 => 3,
            14 => 4,
            15 => 5,
            16 => 6,
            17 => 7,
            18 => 8,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.plus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.plus_posts(1);
        //}

        //show_wall_notify_items(51, self.id);
    }

    pub fn close_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let close_case = match self.types {
            1 => 21,
            2 => 22,
            3 => 23,
            4 => 24,
            5 => 25,
            6 => 26,
            7 => 27,
            8 => 28,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.minus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.minus_posts(1);
        //}

        //hide_wall_notify_items(51, self.id);
    }
    pub fn unclose_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let close_case = match self.types {
            21 => 1,
            22 => 2,
            23 => 3,
            24 => 4,
            25 => 5,
            26 => 6,
            27 => 7,
            28 => 8,
            _ => self.types,
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.plus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.plus_posts(1);
        //}

        //show_wall_notify_items(51, self.id);
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

    pub fn fixed_post(&self) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::types.eq(2))
            .get_result::<Post>(&_connection)
            .expect("E");
    }
    pub fn unfixed_post(&self) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::types.eq(1))
            .get_result::<Post>(&_connection)
            .expect("E");
        return true;
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let self_attach = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>();
            return "files_".to_string() + &self_attach.len().to_string();
        }
        return "files_0".to_string();
    }

    pub fn reposts(&self, limit: i64, offset: i64) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.eq_any(vec![1, 2]))
            .limit(limit)
            .offset(offset)
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn window_reposts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.eq_any(vec![1, 2]))
            .limit(6)
            .load::<Post>(&_connection)
            .expect("E");
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> () {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = posts
                .filter(schema::posts::id.eq(i.key))
                .filter(schema::posts::types.eq(1))
                .limit(1)
                .load::<Post>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::posts::position.eq(i.value))
                .get_result::<Post>(&_connection)
                .expect("Error.");
        }
    }

    pub fn create_comment (
        &self,
        user_id:      i32,
        community_id: Option<i32>,
        attach:       Option<String>,
        parent_id:    Option<i32>,
        content:      Option<String>,
        sticker_id:   Option<i32>
    ) -> PostComment {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + 1))
            .get_result::<Post>(&_connection)
            .expect("Error.");

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}

        let new_comment_form = NewPostComment {
            post_id:      self.id,
            user_id:      user_id,
            community_id: community_id,
            sticker_id:   sticker_id,
            parent_id:    parent_id,
            content:      content,
            attach:       attach,
            types:        1,
            created:      chrono::Local::now().naive_utc(),
            repost:       0,
            reactions:    0,
            replies:      0,
        };
        let new_comment = diesel::insert_into(schema::post_comments::table)
            .values(&new_comment_form)
            .get_result::<PostComment>(&_connection)
            .expect("Error.");

        return new_comment;
    }
    pub fn get_parent(&self) -> Post {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::id.eq(self.parent_id.unwrap()))
            .filter(schema::posts::types.lt(10))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
}
