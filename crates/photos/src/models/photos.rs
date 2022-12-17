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
    get_limit_offset,
    get_limit,
    JsonPosition,
    JsonItemReactions,
    CardPhotoJson,
    ReactionBlockJson,
    SmallReactionBlockJson,
    PhotoDetailJson,
    CardUserJson,
    CardOwnerJson,
    CardCommentJson,
    EditPhotoJson,
    DataEditPhoto,
    RespPhoto,
    RespComment,
    AttachPhotoResp,
    AttachmentsJson,
};
use actix_web::web::Json;
use crate::models::{
    PhotoComment, NewPhotoComment, PhotoList,
    PhotoCounterReaction, User, Community,
};
use crate::schema::photos;
use crate::errors::Error;


/*
Photo
тип
0 Опубликовано
5 Опубликовано приватно
10 Закрепленый
15 Черновик владельца
20 Черновик предложки
25 Предложка сообщества
30 Предложка пользователя
35 Родительский пост

40 Удаленый Опубликовано
45 Удаленый Опубликовано приватно
50 Удаленый Закрепленый
55 Удаленый Черновик владельца
60 Удаленый Черновик предложки
65 Удаленый Предложка сообщества
70 Удаленый Предложка пользователя
75 Удаленый Родительский пост

80 Закрытый Опубликовано
85 Закрытый Опубликовано приватно
90 Закрытый Закрепленый
95 Закрытый Черновик владельца
100 Закрытый Черновик предложки
105 Закрытый Предложка сообщества
110 Закрытый Предложка пользователя
115 Закрытый Родительский пост

120 Удаленый полностью Опубликовано
125 Удаленый полностью Опубликовано приватно
130 Удаленый полностью Закрепленый
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Photo {
    pub id:            i32,
    pub community_id:  Option<i32>,
    pub user_id:       i32,
    pub photo_list_id: i32,
    pub types:         i16,
    pub server_id:     i16,
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
    pub community_id: Option<i32>,
    pub user_id:       i32,
    pub photo_list_id: i32, 
    pub types:         i16,
    pub server_id:     i16,
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
}
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="photos"]
pub struct EditPhotoPosition {
    pub position: i16,
}

#[derive(Serialize)]
pub struct SearchAllPhotos {
    pub photos: Vec<CardPhotoJson>,
    pub offset: i64,
}

impl Photo {
    pub fn get_photos_for_attach(ids: Vec<i32>) -> Vec<AttachPhotoResp> {
        // выдача инфы для прикрепления записей по запросу API
        use crate::schema::photos::dsl::photos;
        use crate::utils::{
            AttachOwner,
            AttachCommunity,
            AttachPermList,
            AttachPhoto,
        };

        let mut stack: Vec<AttachPhotoResp> = Vec::new();
        let _connection = establish_connection();
        let photo_list = photos
            .filter(schema::photos::id.eq_any(ids))
            .filter(schema::photos::types.lt(10))
            .load::<Photo>(&_connection)
            .expect("E.");

        for i in photo_list.iter() {
            let mut c_resp: Option<AttachCommunity> = None;
            let mut u_resp: Option<AttachOwner> = None;
            if i.community_id.is_some() {
                let community = i.get_community().expect("E.");
                c_resp = Some(AttachCommunity {
                    id:       community.id,
                    name:     community.name,
                    types:    community.types,
                    link:     community.link,
                    s_avatar: community.s_avatar,
                })
            }
            else {
                let creator = i.get_creator().expect("E.");
                u_resp = Some(AttachOwner {
                    id:         creator.id,
                    first_name: creator.first_name,
                    last_name:  creator.last_name,
                    types:      creator.types,
                    link:       creator.link,
                    s_avatar:   creator.s_avatar,
                    see_all:    creator.see_all,
                })
            }
            let list = i.get_list().expect("E.");
            let list_data = AttachPermList {
                user_id:      list.user_id,
                community_id: list.community_id,
                list_id:      list.id,
                list_types:   24,
                types:        list.types,
                see_el:       list.see_el,
                copy_el:      list.copy_el,
            };
            let data = AttachPhoto {
                id:        i.id,
                server_id: i.server_id,
                file:      i.file.clone(),
            };
            stack.push (AttachPhotoResp {
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
    pub fn search_photos (
        q:       &String,
        user_id: i32,
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> SearchAllPhotos {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        let mut _count = 0;
        let mut _step = 0;
        let (_limit, mut _offset) = get_limit_offset(limit, offset, 20);

        let mut creator_include: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых можно смотреть фото
        let mut community_include: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых можно смотреть фото
        let mut list_include: Vec<i32> = Vec::new();      // запишем ids списков, у которых можно смотреть фото
        let mut creator_exclude: Vec<i32> = Vec::new();   // запишем ids пользователей, у которых нельзя смотреть фото
        let mut community_exclude: Vec<i32> = Vec::new(); // запишем ids сообществ, у которых нельзя смотреть фото
        let mut list_exclude: Vec<i32> = Vec::new();      // запишем ids списков, у которых нельзя смотреть фото
        let mut photos_json = Vec::new();

        while _count < _limit {
            _step += _limit;

            let items = photos
                .filter(schema::photos::description.ilike(&q))
                .filter(schema::photos::types.lt(11))
                .limit(_step)
                .offset(_offset)
                .order(schema::photos::created.desc())
                .load::<Photo>(&_connection)
                .expect("E.");

            for i in items.iter() {
                if _count == _limit {
                    break;
                }

                // проверяем, запрещено ли запрашивающему смотреть
                // фото пользователя или сообщества или списка
                if creator_exclude.iter().any(|&a| a==i.user_id)
                    ||
                    (i.community_id.is_some() && community_exclude.iter().any(|&a| a==i.community_id.unwrap()))
                    ||
                    list_exclude.iter().any(|&a| a==i.photo_list_id)
                {
                    continue;
                }
                else if list_include.iter().any(|&a| a==i.photo_list_id) {
                    _count += 1;
                    let list = i.get_list().expect("E.");
                    photos_json.push(i.get_photo_json());
                    continue;
                }

                let list = i.get_list().expect("E.");

                if i.community_id.is_some() {
                    // если фото сообщества
                    if community_include.iter().any(|&a| a==i.community_id.unwrap()) {
                        // если id сообщества в разрешенных community_include,
                        if (user_id > 0 && list.is_user_see_el(user_id))
                            ||
                            (user_id == 0 && list.is_anon_user_see_el())
                        {
                            photos_json.push(i.get_photo_json());
                            _count += 1;
                            list_include.push(i.photo_list_id);
                            continue;
                        }
                        else {
                            list_exclude.push(i.photo_list_id);
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
                                photos_json.push(i.get_photo_json());
                                _count += 1;
                                list_include.push(i.photo_list_id);
                                continue;
                            }
                            else {
                                list_exclude.push(i.photo_list_id);
                                continue;
                            }
                        }
                        else {
                            community_exclude.push(i.community_id.unwrap());
                            continue;
                        }
                    }
                }
                // если фото пользователя
                if creator_include.iter().any(|&a| a==i.user_id) {
                    // если id пользователя в разрешенных creator_include,
                    if (user_id > 0 && list.is_user_see_el(user_id))
                        ||
                        (user_id == 0 && list.is_anon_user_see_el())
                    {
                        photos_json.push(i.get_photo_json());
                        _count += 1;
                        list_include.push(i.photo_list_id);
                        continue;
                    }
                    else {
                        list_exclude.push(i.photo_list_id);
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
                            photos_json.push(i.get_photo_json());
                            _count += 1;
                            list_include.push(i.photo_list_id);
                            continue;
                        }
                        else {
                            list_exclude.push(i.photo_list_id);
                            continue;
                        }
                    }
                    else {
                        creator_exclude.push(i.user_id);
                        continue;
                    }
                }
            }
            _offset += _limit;
        }
        return SearchAllPhotos {
            photos: photos_json,
            offset: _offset,
        };
    }

    pub fn search_comments (
        &self,
        user_id:        i32,
        reactions_list: Vec<i32>,
        q:              &String,
        limit:          Option<i64>,
        offset:         Option<i64>,
    ) -> Vec<CardCommentJson> { 
        use crate::schema::photo_comments::dsl::photo_comments;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let mut comments_json = Vec::new();
        let items = photo_comments
            .filter(schema::photo_comments::photo_id.eq(self.id))
            .filter(schema::photo_comments::content.ilike(&q))
            .filter(schema::photo_comments::types.lt(5))
            .order(schema::photo_comments::created.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<PhotoComment>(&_connection)
            .expect("E.");

        for c in items.iter() {
            let creator = c.get_owner_meta().expect("E");
            comments_json.push (CardCommentJson {
                content:        c.content.clone(),
                owner_name:     creator.name.clone(),
                owner_link:     creator.link.clone(),
                owner_image:    creator.image.clone(),
                created:        c.created.format("%d-%m-%Y в %H:%M").to_string(),
                reactions:      c.reactions,
                types:          c.get_code(),       // например cpo1
                replies:        c.replies,          // кол-во ответов
                reactions_list: c.get_reactions_json(user_id, reactions_list.clone()),
                attachments:    None,
            });
        }
        return comments_json;
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
        user_id:        i32,
        reactions_list: Vec<i32>,
        limit:          Option<i64>,
        offset:         Option<i64>
    ) -> Vec<CardCommentJson> {
        use crate::schema::photo_comments::dsl::photo_comments;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let mut json = Vec::new();
        let items = photo_comments
            .filter(schema::photo_comments::photo_id.eq(self.id))
            .filter(schema::photo_comments::types.lt(10))
            .filter(schema::photo_comments::parent_id.is_null())
            .limit(_limit)
            .offset(_offset)
            .load::<PhotoComment>(&_connection)
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

    pub fn get_file(&self) -> String {
        let _path = self.photo_list_id.to_string() + &"/cur-".to_string() + &self.file;

        return match self.server_id {
            1 => "http://194.58.90.123:9050/photo_files/media/".to_owned() + &_path,
            2 => "http://194.58.90.123:9051/photo_files_2/media/".to_owned() + &_path,
            _ => "".to_string(),
        };
    }
    pub fn get_detail_photo_json (
        &self,
        user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> PhotoDetailJson {
        let list = self.get_list().expect("E");
        let creator = self.get_owner_meta().expect("E");
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
        if user_id > 0 {
            return PhotoDetailJson {
                    file:                 self.get_file(),
                    description:          self.description.clone(),
                    owner_name:           creator.name.clone(),
                    owner_link:           creator.link.clone(),
                    owner_image:          creator.image.clone(),
                    comments_on:          self.comments_on,
                    created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                    comment:              self.comment,
                    view:                 self.view,
                    repost:               self.repost,
                    reactions:            self.reactions,
                    reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                    prev:                 prev,
                    next:                 next,
                    is_user_see_comments: list.is_user_see_comment(user_id),
                    is_user_create_comments: list.is_user_create_comment(user_id),
                    comments:             self.get_comments(user_id, reactions_list.clone(), limit, offset),
                };
        } else {
            return PhotoDetailJson {
                file:                 self.get_file(),
                description:          self.description.clone(),
                owner_name:           creator.name.clone(),
                owner_link:           creator.link.clone(),
                owner_image:          creator.image.clone(),
                comments_on:          self.comments_on,
                created:              self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:              self.comment,
                view:                 self.view,
                repost:               self.repost,
                reactions:            self.reactions,
                reactions_list:       self.get_reactions_json(user_id, reactions_list.clone()),
                prev:                 prev,
                next:                 next,
                is_user_see_comments: list.is_anon_user_see_comment(),
                is_user_create_comments: false,
                comments:             self.get_comments(user_id, reactions_list.clone(), limit, offset),
            };
        }
    }
    pub fn get_photo_json (&self) -> CardPhotoJson {
        return CardPhotoJson {
                id:        self.id,
                server_id: self.server_id,
                file:      self.file.clone(),
                position:  self.position,
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
        limit:       Option<i64>,
        offset:      Option<i64>,
    ) -> ReactionBlockJson {
        use crate::schema::{
            photo_reactions::dsl::photo_reactions,
            users::dsl::users,
        };
        use crate::utils::CardReactionPhotoJson;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let mut user_reaction: Option<i32> = None;
        if self.is_have_user_reaction(user_id) {
            user_reaction = Some(self.get_user_reaction(user_id).expect("E."));
        }

        let _connection = establish_connection();
        let user_ids = photo_reactions
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .filter(schema::photo_reactions::reaction_id.eq(reaction_id))
            .limit(_limit)
            .offset(_offset)
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
    pub fn get_count_model_for_reaction(&self, reaction_id: i32) -> PhotoCounterReaction {
        use crate::schema::photo_counter_reactions::dsl::photo_counter_reactions;
        use crate::models::NewPhotoCounterReaction;

        let _connection = establish_connection();
        let _react_model = photo_counter_reactions
            .filter(schema::photo_counter_reactions::photo_id.eq(self.id))
            .first::<PhotoCounterReaction>(&_connection);
        if _react_model.is_ok() {
            return _react_model.expect("Error.");
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
        user_id:     i32,
        reaction_id: i32,
    ) -> JsonItemReactions {
        use crate::schema::photo_reactions::dsl::photo_reactions;
        use crate::models::{PhotoReaction, NewPhotoReaction};

        let _connection = establish_connection();
        let list = self.get_list().expect("E.");
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_count_model_for_reaction(reaction_id);

        if reactions_of_list.iter().any(|&i| i==reaction_id) && list.is_user_see_el(user_id) && list.is_user_see_el(user_id) {
            let vote_ok = photo_reactions
                .filter(schema::photo_reactions::user_id.eq(user_id))
                .filter(schema::photo_reactions::photo_id.eq(self.id))
                .first::<PhotoReaction>(&_connection);

            // если пользователь уже реагировал на фото
            if vote_ok.is_ok() {
                let vote = vote_ok.expect("E");

                // если пользователь уже реагировал этой реакцией на этот фото
                if vote.reaction_id == reaction_id {
                    diesel::delete (
                        photo_reactions
                            .filter(schema::photo_reactions::user_id.eq(user_id))
                            .filter(schema::photo_reactions::photo_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_count(self.id, user_id, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот фото
                else {
                    diesel::update(&vote)
                        .set(schema::photo_reactions::reaction_id.eq(reaction_id))
                        .execute(&_connection)
                        .expect("Error.");

                    react_model.update_count(self.id, user_id, false);
                }
            }

            // если пользователь не реагировал на этот фото
            else {
                let new_vote = NewPhotoReaction {
                    user_id:     user_id,
                    photo_id:     self.id,
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
        use crate::schema::photo_reactions::dsl::photo_reactions;

        let _connection = establish_connection();
        let votes = photo_reactions
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .select(schema::photo_reactions::user_id)
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
        use crate::schema::photo_reactions::dsl::photo_reactions;
        let _connection = establish_connection();
        let vote = photo_reactions
            .filter(schema::photo_reactions::user_id.eq(user_id))
            .filter(schema::photo_reactions::photo_id.eq(self.id))
            .select(schema::photo_reactions::reaction_id)
            .first::<i32>(&_connection)?;
        return Ok(vote);
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

    pub fn get_list(&self) -> Result<PhotoList, Error> {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        return Ok(photo_lists
            .filter(schema::photo_lists::id.eq(self.photo_list_id))
            .filter(schema::photo_lists::types.lt(31))
            .first::<PhotoList>(&_connection)?);
    }

    pub fn get_playlist_image(&self) -> String {
        return "/static/images/news_small3.jpg".to_string();
    }

    pub fn copy_item (
        &self,
        lists: Vec<i32>,
    ) -> i16 {
        use crate::schema::photo_lists::dsl::photo_lists;

        let _connection = establish_connection();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = photo_lists
                .filter(schema::photo_lists::id.eq(list_id))
                .filter(schema::photo_lists::types.lt(31))
                .first::<PhotoList>(&_connection)
                .expect("E");

            let new_photo_form = NewPhoto {
                community_id: self.community_id,
                user_id: self.user_id,
                photo_list_id: self.photo_list_id,
                types: self.types,
                server_id: self.server_id,
                file: self.file.clone(),
                description: self.description.clone(),
                comments_on: self.comments_on,

                created: self.created,
                comment: self.comment,
                view: 0,
                repost: 0,
                copy: 0,
                position: count,
                reactions: 0,
            };
            diesel::insert_into(schema::photos::table)
                .values(&new_photo_form)
                .execute(&_connection)
                .expect("Error.");

            diesel::update(&list)
              .set(schema::photo_lists::copy.eq(list.count + 1))
              .execute(&_connection)
              .expect("Error.");
        }
        diesel::update(self)
          .set(schema::photos::copy.eq(self.copy + count as i32))
          .execute(&_connection)
          .expect("Error."); 
        return 1;
    }

    pub fn edit_photo (&self, description: Option<String>) -> i16 {
        let _connection = establish_connection();
        let edit_photo = EditPhoto {
            description: description.clone(),
        };
        diesel::update(self)
            .set(edit_photo)
            .execute(&_connection)
            .expect("Error.");

        return 1;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::comment.eq(self.comment + count))
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::reactions.eq(self.reactions + count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus_reactions(&self, count: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::reactions.eq(self.reactions - count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::photos::comment.eq(self.comment - count))
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

    pub fn on_comments(&self) -> i16 {
        let _connection = establish_connection();
        let o_1 = diesel::update(self)
            .set(schema::photos::comments_on.eq(true))
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
            .set(schema::photos::comments_on.eq(false))
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
            .set(schema::photos::types.eq(self.types + 40))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count - 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_photos(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_photos(1);
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
            .set(schema::photos::types.eq(self.types - 40))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count + 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_photos(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_photos(1);
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
            .set(schema::photos::types.eq(self.types + 80))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count - 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.minus_photos(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.minus_photos(1);
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
            .set(schema::photos::types.eq(self.types - 80))
            .execute(&_connection);
        let list = self.get_list().expect("E");
        let o_2 = diesel::update(&list)
            .set(schema::photo_lists::count.eq(list.count + 1))
            .execute(&_connection);

        if self.community_id.is_some() {
            let community = self.get_community().expect("E");
            community.plus_photos(1);
        }
        else {
            let creator = self.get_creator().expect("E");
            creator.plus_photos(1);
        }

        if o_1.is_ok() && o_2.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn get_format_text(&self) -> String {
        if self.description.is_some() {
            let unwrap = self.description.as_ref().unwrap();
            let split_unwrap: Vec<&str> = unwrap.split(" ").collect();
            if split_unwrap.len() <= 20 {
                return self.description.as_ref().unwrap().to_string();
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

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> () {
        use crate::schema::photos::dsl::photos;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = photos
                .filter(schema::photos::id.eq(i.key))
                .filter(schema::photos::types.lt(10))
                .first::<Photo>(&_connection);
            if item.is_ok() {
                diesel::update(&item.expect("E."))
                    .set(schema::photos::position.eq(i.value))
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
            .set(schema::photos::comment.eq(self.comment + 1))
            .execute(&_connection)
            .expect("Error.");

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
                .plus_comments(1);
        }
        else {
            use crate::utils::get_user;
            get_user(user_id)
                .expect("E.")
                .plus_comments(1);
        }
        let new_comment_form = NewPhotoComment {
            photo_id:     self.id,
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
        let new_comment = diesel::insert_into(schema::photo_comments::table)
            .values(&new_comment_form)
            .get_result::<PhotoComment>(&_connection)
            .expect("Error.");


        return RespComment {
            id:           new_comment.id,
            photo_id:     self.id,
            user_id:      user_id,
            community_id: community_id,
            content:      content.clone(),
            parent_id:    parent_id,
            attachments:  None,
        };
    }
}
