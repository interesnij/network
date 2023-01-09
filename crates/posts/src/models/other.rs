use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema::{
    user_post_list_collections,
    user_post_list_positions,
    community_post_list_collections,
    community_post_list_positions,
    post_list_perms,
    item_reposts,
    post_counter_reactions,
    post_comment_counter_reactions,
    post_reactions,
    post_comment_reactions,
    list_user_communities_keys,
    notify_user_communities,
    news_user_communities,
    featured_user_communities,
    user_post_notifications,
    community_post_notifications,
};
//use crate::errors::Error;
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
};
//use actix_web::web::Json;
use crate::models::PostComment;


/////// UserPostListPosition //////
#[derive(Queryable, Serialize, Identifiable)]
pub struct UserPostListPosition {
    pub id:       i32,
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    i16, // 1 - открыт, 0 - недоступен (например, удален)
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_positions"]
pub struct NewUserPostListPosition {
    pub user_id:  i32,
    pub list_id:  i32,
    pub position: i16,
    pub types:    i16,
}
/////// CommunityPostListPosition //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityPostListPosition {
    pub id:           i32,
    pub community_id: i32,
    pub list_id:      i32,
    pub position:     i16,
    pub types:        i16, // 1 - open, 2 - close
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_positions"]
pub struct NewCommunityPostListPosition {
    pub community_id:  i32,
    pub list_id:       i32,
    pub position:      i16,
    pub types:         i16,
}

/////// UserPostListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPostListCollection {
    pub id:           i32,
    pub user_id:      i32,
    pub post_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_collections"]
pub struct NewUserPostListCollection {
    pub user_id:      i32,
    pub post_list_id: i32,
}

/////// CommunityPostListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPostListCollection {
    pub id:           i32,
    pub community_id: i32,
    pub post_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_collections"]
pub struct NewCommunityPostListCollection {
    pub community_id: i32,
    pub post_list_id: i32,
}

/*
PostListPerm
1 может видеть записи
2 может видеть комменты
3 может создавать записи
4 может создавать комменты
5 может копировать / репостить
11 не может видеть записи
12 не может видеть комменты
13 не может создавать записи
14 не может создавать комменты
15 не может копировать / репостить

101 список может видеть записи
102 список может видеть комменты к записи
103 список может создавать записи
104 список может создавать комменты к записи
105 список может копировать списки / записи

111 список не может видеть записи
112 список не может видеть комменты к записи
113 список не может создавать записи
114 список не может создавать комменты к записи
115 список не может копировать списки / записи

21 в черном списке пользователя
22 в черном списке сообщества
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostListPerm {
    pub id:           i32,
    pub user_id:      i32,
    pub post_list_id: i32,
    pub types:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_list_perms"]
pub struct NewPostListPerm {
    pub user_id:      i32,
    pub post_list_id: i32,
    pub types:        i16,
}


/////// PostReaction//////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostReaction {
    pub id:          i32,
    pub user_id:     i32,
    pub post_id:     i32,
    pub reaction_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="post_reactions"]
pub struct NewPostReaction {
    pub user_id:     i32,
    pub post_id:     i32,
    pub reaction_id: i32,
}
/////// PostCommentVote //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostCommentReaction {
    pub id:              i32,
    pub user_id:         i32,
    pub post_comment_id: i32,
    pub reaction_id:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comment_reactions"]
pub struct NewPostCommentReaction {
    pub user_id:         i32,
    pub post_comment_id: i32,
    pub reaction_id:     i32,
}

/////// ItemRepost //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct ItemRepost {
    pub id:         i32,
    pub item_id:    i32,
    pub item_types: i16,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="item_reposts"]
pub struct NewItemRepost {
    pub item_id:    i32,
    pub item_types: i16,
    pub post_id:    Option<i32>,
    pub message_id: Option<i32>,
}

/////// PostCounterReaction //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostCounterReaction {
    pub id:          i32,
    pub post_id:     i32,
    pub reaction_id: i32,
    pub count:       i32,
}

impl PostCounterReaction {
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
        post_id: i32,
        user_id: i32,
        plus:    bool,
    ) -> () {
        use crate::schema::{
            post_counter_reactions::dsl::post_counter_reactions,
            post_reactions::dsl::post_reactions,
        };

        let _connection = establish_connection();
        if plus {
            diesel::update(self)
                .set(schema::post_counter_reactions::count.eq(self.count + 1))
                .get_result::<PostCounterReaction>(&_connection)
                .expect("Error.");

            let prev_reactions = post_reactions
                .filter(schema::post_reactions::post_id.eq(post_id))
                .filter(schema::post_reactions::user_id.eq(user_id))
                .load::<PostReaction>(&_connection)
                .expect("E");
            if prev_reactions.len() > 0 {
                for react in prev_reactions.iter() {
                    let prev_react_count = post_counter_reactions
                        .filter(schema::post_counter_reactions::post_id.eq(post_id))
                        .filter(schema::post_counter_reactions::reaction_id.eq(react.reaction_id))
                        .limit(1)
                        .load::<PostCounterReaction>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0)
                        .unwrap();
                    diesel::update(&prev_react_count)
                        .set(schema::post_counter_reactions::count.eq(prev_react_count.count - 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            diesel::delete(post_reactions
                .filter(schema::post_reactions::post_id.eq(post_id))
                .filter(schema::post_reactions::user_id.eq(user_id))
            )
            .execute(&_connection)
            .expect("E");
        }
        else {
            diesel::update(self)
                .set(schema::post_counter_reactions::count.eq(self.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="post_counter_reactions"]
pub struct NewPostCounterReaction {
    pub post_id:     i32,
    pub reaction_id: i32,
    pub count:       i32,
}


/////// PostCommentCounterReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(PostComment)]
pub struct PostCommentCounterReaction {
    pub id:              i32,
    pub post_comment_id: i32,
    pub reaction_id:     i32,
    pub count:           i32,
}
impl PostCommentCounterReaction {
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
        post_comment_id: i32,
        user_id:         i32,
        plus:            bool,
    ) -> () {
        use crate::schema::{
            post_comment_counter_reactions::dsl::post_comment_counter_reactions,
            post_comment_reactions::dsl::post_comment_reactions,
        };

        let _connection = establish_connection();
        if plus {
            diesel::update(self)
                .set(schema::post_comment_counter_reactions::count.eq(self.count + 1))
                .get_result::<PostCommentCounterReaction>(&_connection)
                .expect("Error.");

            let prev_reactions = post_comment_reactions
                .filter(schema::post_comment_reactions::post_comment_id.eq(post_comment_id))
                .filter(schema::post_comment_reactions::user_id.eq(user_id))
                .load::<PostCommentReaction>(&_connection)
                .expect("E");
            if prev_reactions.len() > 0 {
                for react in prev_reactions.iter() {
                    let prev_react_count = post_comment_counter_reactions
                        .filter(schema::post_comment_counter_reactions::post_comment_id.eq(post_comment_id))
                        .filter(schema::post_comment_counter_reactions::reaction_id.eq(react.reaction_id))
                        .limit(1)
                        .load::<PostCommentCounterReaction>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0)
                        .unwrap();
                    diesel::update(&prev_react_count)
                        .set(schema::post_comment_counter_reactions::count.eq(prev_react_count.count - 1))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            diesel::delete( post_comment_reactions
                    .filter(schema::post_comment_reactions::post_comment_id.eq(post_comment_id))
                    .filter(schema::post_comment_reactions::user_id.eq(user_id))
                )
                .execute(&_connection)
                .expect("E");
        }
        else {
            diesel::update(self)
                .set(schema::post_comment_counter_reactions::count.eq(self.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="post_comment_counter_reactions"]
pub struct NewPostCommentCounterReaction {
    pub post_comment_id: i32,
    pub reaction_id:     i32,
    pub count:           i32,
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


/////// UserPostNotification //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPostNotification {
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
#[table_name="user_post_notifications"]
pub struct NewUserPostNotification {
    pub user_id:         i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}

/////// CommunityPostNotification //////
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPostNotification {
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
#[table_name="community_post_notifications"]
pub struct NewCommunityPostNotification {
    pub community_id:    i32,
    pub comment:         i16,
    pub comment_reply:   i16,
    pub mention:         i16,
    pub comment_mention: i16,
    pub repost:          i16,
    pub reactions:       i16
}