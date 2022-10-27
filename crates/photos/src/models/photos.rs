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
    CardParentPhotoJson,
    CardPhotoJson,
    ReactionBlockJson,
    PhotoDetailJson,
    CardUserJson,
    CardOwnerJson,
    CardCommentJson,
};
use actix_web::web::Json;
use crate::models::{
    PhotoComment, NewPhotoComment, PhotoList,
    PhotoCounterReaction, User, Community,
};
use crate::schema::photos;

/////// Photo //////

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
pub struct Photo {
    pub id:            i32,
    pub community_id:  Option<i32>,
    pub user_id:       i32,
    pub photo_list_id: i32,
    pub types:         i16,
    pub preview:       String,
    pub file:          String,
    pub description:   Option<String>,
    pub comments_on:   bool,
    pub created:       chrono::NaiveDateTime,
    pub comment:       i32,
    pub view:          i32,
    pub repost:        i32,
    pub copy:          i32,
    pub position:      i16,
    pub reactions:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="photos"]
pub struct NewPhoto {
    pub community_id:  Option<i32>,
    pub user_id:       i32,
    pub photo_list_id: i32,
    pub types:         i16,
    pub preview:       String,
    pub file:          String,
    pub description:   Option<String>,
    pub comments_on:   bool,
    pub created:       chrono::NaiveDateTime,
    pub comment:       i32,
    pub view:          i32,
    pub repost:        i32,
    pub copy:          i32,
    pub position:      i16,
    pub reactions:     i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="photos"]
pub struct EditPhoto {
    pub description:  Option<String>,
    pub comments_on:  bool,
}
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="photos"]
pub struct EditPhotoPosition {
    pub position: i16,
}

impl Photo {
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
        use crate::schema::photo_comments::dsl::photo_comments;

        let _connection = establish_connection();
        let mut json = Vec::new();
        let items = photo_comments
            .filter(schema::photo_comments::photo_id.eq(self.id))
            .filter(schema::photo_comments::types.lt(10))
            .filter(schema::photo_comments::parent_id.is_null())
            .limit(limit)
            .offset(offset)
            .load::<PhotoComment>(&_connection)
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
                attachments:    None,
            });
        }
        return json;
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

    pub fn get_detail_photo_json (&self, user_id: i32, page: i32, limit: i32) -> PhotoDetailJson {
        let list = self.get_list();
        let creator = self.get_owner_meta();
        let reactions_list = list.get_reactions_list();
        let mut prev: Option<i32> = None;
        let mut next: Option<i32> = None;
        let _photos = list.get_items();
        for (i, item) in _photos.iter().enumerate().rev() {
            if item.position == self.position {
                if (i + 1) != _photos.len() {
                    prev = Some(_photos[i + 1].id);
                };
                if i != 0 {
                    next = Some(_photos[i - 1].id);
                };
                break;
            }
        };
        return PhotoDetailJson {
                owner_name:           creator.name.clone(),
                owner_link:           creator.link.clone(),
                owner_image:          creator.image.clone(),
                preview:              self.preview,
                file:                 self.file,
                description:          self.description,
                comments_on:          self.comments_on,
                created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:              self.comment,
                view:                 self.view,
                repost:               self.repost,
                reactions:            self.reactions,
                types:                self.get_code(),
                reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                prev:                 prev,
                next:                 next,
                is_user_see_comments: list.is_user_see_comment(user_id),
                is_user_create_el:    list.is_user_create_el(user_id),
                comments:             self.get_comments(limit, 0, user_id, reactions_list.clone()),
            };
    }
    pub fn get_photo_json (&self) -> CardPhotoJson {
        return CardPhotoJson {
                id:      self.id,
                preview: self.preview,
                file:    self.file,
            };
    }

    pub fn get_6_user_of_reaction (
        &self,
        reaction_id: &i32,
        user_reaction: Option<i32>,
    ) -> ReactionBlockJson {
        use crate::schema::{
            photo_reactions::dsl::photo_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPhotoJson;

        let _connection = establish_connection();
        let user_ids = photo_reactions
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .filter(schema::photo_reactions::reaction_id.eq(reaction_id))
            .limit(6)
            .select(schema::photo_reactions::user_id)
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
                CardReactionPhotoJson {
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
            photo_reactions::dsl::photo_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPhotoJson;

        let _connection = establish_connection();
        let user_ids = photo_reactions
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .filter(schema::photo_reactions::reaction_id.eq(reaction_id))
            .limit(limit)
            .offset(offset)
            .select(schema::photo_reactions::user_id)
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
                CardReactionPhotoJson {
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
    pub fn get_count_model_for_reaction(&self, reaction_id: i32) -> PhotoCounterReaction {
        use crate::schema::photo_counter_reactions::dsl::photo_counter_reactions;
        use crate::models::NewPhotoCounterReaction;

        let _connection = establish_connection();
        let _react_model = photo_counter_reactions
            .filter(schema::photo_counter_reactions::photo_id.eq(self.id))
            .load::<PhotoCounterReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewPhotoCounterReaction {
                photo_id:    self.id,
                reaction_id: reaction_id,
                count:       0,
            };
            let _react_model = diesel::insert_into(schema::photo_counter_reactions::table)
                .values(&new_react_model)
                .get_result::<PhotoCounterReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }
    pub fn send_reaction (
        &self,
        user_id: i32,
        reaction_id: i32,
    ) -> Json<JsonItemReactions> {
        use crate::schema::photo_reactions::dsl::photo_reactions;
        use crate::models::{PhotoReaction, NewPhotoReaction};

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_count_model_for_reaction(reaction_id);

        if reactions_of_list.iter().any(|&i| i==reaction_id) && list.is_user_see_el(user_id) && list.is_user_see_comment(user_id) {
            let votes = photo_reactions
                .filter(schema::photo_reactions::user_id.eq(user_id))
                .filter(schema::photo_reactions::photo_id.eq(self.id))
                .load::<PhotoReaction>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction_id == reaction_id {
                    diesel::delete(photo_reactions
                        .filter(schema::photo_reactions::user_id.eq(user_id))
                        .filter(schema::photo_reactions::photo_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_count(self.id, user_id, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот товар
                else {
                    diesel::update(&vote)
                        .set(schema::photo_reactions::reaction_id.eq(reaction_id))
                        .get_result::<PhotoReaction>(&_connection)
                        .expect("Error.");

                    react_model.update_count(self.id, user_id, false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewPhotoReaction {
                    user_id:     user_id,
                    photo_id:    self.id,
                    reaction_id: reaction_id,
                };
                diesel::insert_into(schema::photo_reactions::table)
                    .values(&new_vote)
                    .get_result::<PhotoReaction>(&_connection)
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
        use crate::schema::photo_reactions::dsl::photo_reactions;

        let _connection = establish_connection();
        let votes = photo_reactions
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .select(schema::photo_reactions::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return votes;
    }
    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self.reactions_ids().iter().any(|&i| i==user_id);
    }
    pub fn get_user_reaction(&self, user_id: i32) -> i32 {
        use crate::schema::photo_reactions::dsl::photo_reactions;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = photo_reactions
            .filter(schema::photo_reactions::user_id.eq(user_id))
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .select(schema::photo_reactions::reaction_id)
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
    pub fn is_photo(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "pos".to_string() + &self.get_str_id();
    }
    pub fn get_folder(&self) -> String {
        return "photos".to_string();
    }

    pub fn get_list(&self) -> PhotoList {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return photo_lists
            .filter(schema::photo_lists::id.eq(self.photo_list_id))
            .filter(schema::photo_lists::types.lt(10))
            .load::<PhotoList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_playlist_image(&self) -> String {
        return "/static/images/news_small3.jpg".to_string();
    }
    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::photos::dsl::photos;
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let item = photos
            .filter(schema::photos::id.eq(pk))
            .filter(schema::photos::types.eq_any(vec![1, 2]))
            .load::<Photo>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = photo_lists
                .filter(schema::photo_lists::id.eq(list_id))
                .filter(schema::photo_lists::types.lt(10))
                .load::<PhotoList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            list.create_photo (
                item.community_id,
                item.user_id,
                item.preview.clone(),
                item.file.clone(),
            );
        }
        diesel::update(&item)
          .set(schema::photos::copy.eq(item.copy + count))
          .get_result::<Photo>(&_connection)
          .expect("Error.");

        if item.community_id.is_some() {
            let community = item.get_community();
            community.plus_photos(count);
        }
        else {
            let creator = item.get_creator();
            creator.plus_photos(count);
         }
        return true;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::comment.eq(self.comment + count))
            .get_result::<Photo>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::reactions.eq(self.reactions + count))
            .get_result::<Photo>(&_connection)
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
            .set(schema::photos::reactions.eq(self.reactions - count))
            .get_result::<Photo>(&_connection)
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
            .set(schema::photos::comment.eq(self.comment - count))
            .get_result::<Photo>(&_connection)
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
            .set(schema::photos::types.eq(close_case))
            .get_result::<Photo>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count - 1))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.minus_photos(1);
        }
        else {
            let creator = self.get_creator();
            creator.minus_photos(1);
        }

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
            .set(schema::photos::types.eq(close_case))
            .get_result::<Photo>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count + 1))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_photos(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_photos(1);
        }

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
            .set(schema::photos::types.eq(close_case))
            .get_result::<Photo>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count - 1))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_photos(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_photos(1);
        }

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
            .set(schema::photos::types.eq(close_case))
            .get_result::<Photo>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count + 1))
            .get_result::<PhotoList>(&_connection)
            .expect("E");

        if self.community_id.is_some() {
            let community = self.get_community();
            community.plus_photos(1);
        }
        else {
            let creator = self.get_creator();
            creator.plus_photos(1);
        }

        //show_wall_notify_items(51, self.id);
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

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> () {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = photos
                .filter(schema::photos::id.eq(i.key))
                .filter(schema::photos::types.eq(1))
                .limit(1)
                .load::<Photo>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::photos::position.eq(i.value))
                .get_result::<Photo>(&_connection)
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
    ) -> PhotoComment {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::comment.eq(self.comment + 1))
            .get_result::<Photo>(&_connection)
            .expect("Error.");

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}

        let new_comment_form = NewPhotoComment {
            photo_id:     self.id,
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
        let new_comment = diesel::insert_into(schema::photo_comments::table)
            .values(&new_comment_form)
            .get_result::<PhotoComment>(&_connection)
            .expect("Error.");

        return new_comment;
    }
}
