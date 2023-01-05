use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema::{
    user_photo_list_collections,
    user_photo_list_positions,
    community_photo_list_collections,
    community_photo_list_positions,
    photo_list_perms,
    photo_counter_reactions,
    photo_comment_counter_reactions,
    photo_reactions,
    photo_comment_reactions,
    list_user_communities_keys,
    notify_user_communities,
    news_user_communities,
    featured_user_communities,
    user_photo_notifications,
    community_photo_notifications,
};
//use crate::errors::Error;
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
};
//use actix_web::web::Json;
use crate::models::PhotoComment;


/////// UserPhotoListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserPhotoListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    i16, // 1 - открыт, 0 - недоступен (например, удален)
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_list_positions"]
pub struct NewUserPhotoListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    i16,
}
/////// CommunityPhotoListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPhotoListPosition {
    pub id:           i32,
    pub community_id: i32,
    pub list_id:      i32,
    pub position:     i16,
    pub types:        i16, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_list_positions"]
pub struct NewCommunityPhotoListPosition {
    pub community_id:  i32,
    pub list_id:       i32,
    pub position:      i16,
    pub types:         i16,
}

/////// UserPhotoListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPhotoListCollection {
    pub id:           i32,
    pub user_id:      i32,
    pub photo_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_list_collections"]
pub struct NewUserPhotoListCollection {
    pub user_id:       i32,
    pub photo_list_id: i32,
}

/////// CommunityPhotoListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPhotoListCollection {
    pub id:            i32,
    pub community_id:  i32,
    pub photo_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_list_collections"]
pub struct NewCommunityPhotoListCollection {
    pub community_id:  i32,
    pub photo_list_id: i32,
}

/*
PhotoListPerm
1 может видеть фото
2 может видеть комменты
3 может создавать фото
4 может создавать комменты
5 может копировать / репостить
11 не может видеть фото
12 не может видеть комменты
13 не может создавать фото
14 не может создавать комменты
15 не может копировать / репостить
21 в черном списке пользователя
22 в черном списке сообщества
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PhotoListPerm {
    pub id:            i32,
    pub user_id:       i32,
    pub photo_list_id: i32,
    pub types:         i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_list_perms"]
pub struct NewPhotoListPerm {
    pub user_id:       i32,
    pub photo_list_id:  i32,
    pub types:         i16,
}


/////// PhotoReaction//////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PhotoReaction {
    pub id:           i32,
    pub user_id:      i32,
    pub photo_id:     i32,
    pub reaction_id:  i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="photo_reactions"]
pub struct NewPhotoReaction {
    pub user_id:      i32,
    pub photo_id:     i32,
    pub reaction_id:  i32,
}
/////// PhotoCommentVote //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PhotoCommentReaction {
    pub id:              i32,
    pub user_id:         i32,
    pub photo_comment_id: i32,
    pub reaction_id:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="photo_comment_reactions"]
pub struct NewPhotoCommentReaction {
    pub user_id:         i32,
    pub photo_comment_id: i32,
    pub reaction_id:     i32,
}

/////// PhotoCounterReaction //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PhotoCounterReaction {
    pub id:           i32,
    pub photo_id:     i32,
    pub reaction_id:  i32,
    pub count:        i32,
}

impl PhotoCounterReaction {
    pub fn count_reactions_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn update_count (
        &self,
        photo_id: i32,
        user_id:  i32,
        plus:     bool,
    ) -> () {
        use crate::schema::{
            photo_counter_reactions::dsl::photo_counter_reactions,
            photo_reactions::dsl::photo_reactions,
        };

        let _connection = establish_connection();
        if plus {
            diesel::update(self)
                .set(schema::photo_counter_reactions::count.eq(self.count + 1))
                .execute(&_connection)
                .expect("Error.");

            let prev_reactions = photo_reactions
                .filter(schema::photo_reactions::photo_id.eq(photo_id))
                .filter(schema::photo_reactions::user_id.eq(user_id))
                .load::<PhotoReaction>(&_connection)
                .expect("E");
            if prev_reactions.len() > 0 {
                for react in prev_reactions.iter() {
                    let prev_react_count = photo_counter_reactions
                        .filter(schema::photo_counter_reactions::photo_id.eq(photo_id))
                        .filter(schema::photo_counter_reactions::reaction_id.eq(react.reaction_id))
                        .first::<PhotoCounterReaction>(&_connection)
                        .expect("E");
                    diesel::update(&prev_react_count)
                        .set(schema::photo_counter_reactions::count.eq(prev_react_count.count - 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            diesel::delete(photo_reactions
                .filter(schema::photo_reactions::photo_id.eq(photo_id))
                .filter(schema::photo_reactions::user_id.eq(user_id))
            )
            .execute(&_connection)
            .expect("E");
        }
        else {
            diesel::update(self)
                .set(schema::photo_counter_reactions::count.eq(self.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="photo_counter_reactions"]
pub struct NewPhotoCounterReaction {
    pub photo_id:    i32,
    pub reaction_id: i32,
    pub count:       i32,
}


/////// PhotoCommentCounterReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(PhotoComment)]
pub struct PhotoCommentCounterReaction {
    pub id:               i32,
    pub photo_comment_id: i32,
    pub reaction_id:      i32,
    pub count:            i32,
}
impl PhotoCommentCounterReaction {
    pub fn count_reactions_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn update_count (
        &self,
        photo_comment_id: i32,
        user_id:          i32,
        plus:             bool,
    ) -> () {
        use crate::schema::{
            photo_comment_counter_reactions::dsl::photo_comment_counter_reactions,
            photo_comment_reactions::dsl::photo_comment_reactions,
        };

        let _connection = establish_connection();
        if plus {
            diesel::update(self)
                .set(schema::photo_comment_counter_reactions::count.eq(self.count + 1))
                .execute(&_connection)
                .expect("Error.");

            let prev_reactions = photo_comment_reactions
                .filter(schema::photo_comment_reactions::photo_comment_id.eq(photo_comment_id))
                .filter(schema::photo_comment_reactions::user_id.eq(user_id))
                .load::<PhotoCommentReaction>(&_connection)
                .expect("E");
            if prev_reactions.len() > 0 {
                for react in prev_reactions.iter() {
                    let prev_react_count = photo_comment_counter_reactions
                        .filter(schema::photo_comment_counter_reactions::photo_comment_id.eq(photo_comment_id))
                        .filter(schema::photo_comment_counter_reactions::reaction_id.eq(react.reaction_id))
                        .first::<PhotoCommentCounterReaction>(&_connection)
                        .expect("E");
                    diesel::update(&prev_react_count)
                        .set(schema::photo_comment_counter_reactions::count.eq(prev_react_count.count - 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            diesel::delete( photo_comment_reactions
                    .filter(schema::photo_comment_reactions::photo_comment_id.eq(photo_comment_id))
                    .filter(schema::photo_comment_reactions::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else {
            diesel::update(self)
                .set(schema::photo_comment_counter_reactions::count.eq(self.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="photo_comment_counter_reactions"]
pub struct NewPhotoCommentCounterReaction {
    pub photo_comment_id: i32,
    pub reaction_id:      i32,
    pub count:            i32,
}


/////// ListUserCommunitiesKey //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct ListUserCommunitiesKey {
    pub id:    i32,
    pub types: i16,
    pub name:  String,
    pub owner: i32,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="list_user_communities_keys"]
pub struct NewListUserCommunitiesKey {
    pub types: i16,
    pub name:  String,
    pub owner: i32,
}

/////// NewsUserCommunitie //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct NewsUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="news_user_communities"]
pub struct NewNewsUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>
}

/////// NotifyUserCommunitie //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct NotifyUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="notify_user_communities"]
pub struct NewNotifyUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// FeaturedUC //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct FeaturedUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>
}
#[derive(Deserialize, Insertable)]
#[table_name="featured_user_communities"]
pub struct NewFeaturedUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>
}

/////// UserPhotoNotification //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPhotoNotification {
    pub id:              i32,
    pub user_id:         i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}
#[derive(Deserialize, Insertable)]
#[table_name="user_photo_notifications"]
pub struct NewUserPhotoNotification {
    pub user_id:         i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}

/////// CommunityPhotoNotification //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPhotoNotification {
    pub id:              i32,
    pub community_id:    i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}
#[derive(Deserialize, Insertable)]
#[table_name="community_photo_notifications"]
pub struct NewCommunityPhotoNotification {
    pub community_id:    i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}