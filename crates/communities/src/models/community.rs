use crate::schema;
use crate::schema::{
    community_categorys,
    community_subcategorys,
    communitys,
    communities_memberships,
    community_infos,
    community_privates,
    community_notifications,
    community_visible_perms,
    community_banned_users,
    community_follows,
    featured_communities,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;
use crate::utils::{
    //CommunitySubcategoryJson,
    CommunityCategoryJson,
    UsersJson,
    CardUserJson,
    //CommunityInfoJson,
    //CommunityDetailJson,
    CommunityPrivateJson,
    NewCommunityJson,
};
use crate::errors::Error;


/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityCategory {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

impl CommunityCategory {
    pub fn get_categories_json() -> Result<Vec<CommunityCategoryJson>, Error> {
        use crate::schema::community_categorys::dsl::community_categorys;

        let _connection = establish_connection();
        let cats = community_categorys
            .order(schema::community_categorys::position)
            .select((
                schema::community_categorys::id,
                schema::community_categorys::name,
                schema::community_categorys::avatar,
            ))
            .load::<CommunityCategoryJson>(&_connection)?;
        return Ok(cats);
    }
    pub fn create_category(name: String, avatar: Option<String>,
        position: i16) -> Result<CommunityCategory, Error> {

        let _connection = establish_connection();
        let new_form = NewCommunityCategory {
            name:     name,
            avatar:   avatar,
            position: position,
        };
        let new_cat = diesel::insert_into(schema::community_categorys::table)
            .values(&new_form)
            .get_result::<CommunityCategory>(&_connection)?;
        return Ok(new_cat);
    }
    pub fn create_subcategory(&self, name: String, avatar: Option<String>,
        position: i16) -> Result<CommunitySubcategory, Error> {

        let _connection = establish_connection();
        let new_form = NewCommunitySubcategory {
            name:        name,
            category_id: self.id,
            avatar:      avatar,
            position:    position,
        };
        let new_cat = diesel::insert_into(schema::community_subcategorys::table)
            .values(&new_form)
            .get_result::<CommunitySubcategory>(&_connection)?;
        return Ok(new_cat);
    }
    pub fn edit_category(&self, name: String, avatar: Option<String>,
        position: i16) -> Result<CommunityCategory, Error> {
        let _connection = establish_connection();
        let new_form = NewCommunityCategory {
            name:     name,
            avatar:   avatar,
            position: position,
        };
        let updated = diesel::update(self)
            .set(new_form)
            .get_result::<CommunityCategory>(&_connection)?;
        return Ok(updated);
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="community_categorys"]
pub struct NewCommunityCategory {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

/////// CommunitySubCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunitySubcategory {
    pub id:          i32,
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

impl CommunitySubcategory {
    pub fn get_categories_json() -> Result<Vec<CommunityCategoryJson>, Error> {
        use crate::schema::community_categorys::dsl::community_categorys;

        let _connection = establish_connection();
        let cats = community_categorys
            .order(schema::community_categorys::position)
            .select((
                schema::community_categorys::id,
                schema::community_categorys::name,
                schema::community_categorys::avatar,
            ))
            .load::<CommunityCategoryJson>(&_connection)?;

        return Ok(cats);
    }
    pub fn edit_subcategory(&self, name: String, category_id: i32,
        avatar: Option<String>, position: i16) -> Result<&CommunitySubcategory, Error> {
        let _connection = establish_connection();
        let new_form = NewCommunitySubcategory {
            name:        name,
            category_id: category_id,
            avatar:      avatar,
            position:    position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<CommunitySubcategory>(&_connection)?;
        return Ok(self);
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="community_subcategorys"]
pub struct NewCommunitySubcategory {
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

/////// Community //////

/////// Тип сообщества //////
    // 1 публичное сообщество
    // 2 закрытое сообщество
    // 3 публичное сообщество

    // 7 публичное сообщество подало заявку
    // 8 закрытое сообщество подало заявку
    // 9 публичное сообщество подало заявку

    // 13 публичное сообщество идентификацированное
    // 14 закрытое сообщество идентификацированное
    // 15 публичное сообщество идентификацированное

    // 21 удалено публичное сообщество
    // 22 удалено закрытое сообщество
    // 23 удалено публичное сообщество

    // 27 удалено публичное сообщество подало заявку
    // 28 удалено закрытое сообщество подало заявку
    // 29 удалено публичное сообщество подало заявку

    // 33 удалено публичное сообщество идентификацированное
    // 34 удалено закрытое сообщество идентификацированное
    // 35 удалено публичное сообщество идентификацированное

    // 41 баннер публичное сообщество
    // 42 баннер закрытое сообщество
    // 43 баннер публичное сообщество

    // 47 баннер публичное сообщество подало заявку
    // 48 баннер закрытое сообщество подало заявку
    // 49 баннер публичное сообщество подало заявку

    // 53 баннер публичное сообщество идентификацированное
    // 54 баннер закрытое сообщество идентификацированное
    // 55 баннер публичное сообщество идентификацированное

    // 61 закрыто публичное сообщество
    // 62 закрыто закрытое сообщество
    // 63 закрыто публичное сообщество

    // 67 закрыто публичное сообщество подало заявку
    // 68 закрыто закрытое сообщество подало заявку
    // 69 закрыто публичное сообщество подало заявку

    // 73 закрыто публичное сообщество идентификацированное
    // 74 закрыто закрытое сообщество идентификацированное
    // 75 закрыто публичное сообщество идентификацированное

    // 81 закрыто публичное сообщество
    // 82 закрыто закрытое сообщество
    // 83 закрыто публичное сообщество

    // 87 закрыто публичное сообщество подало заявку
    // 88 закрыто закрытое сообщество подало заявку
    // 89 закрыто публичное сообщество подало заявку

    // 93 закрыто публичное сообщество идентификацированное
    // 94 закрыто закрытое сообщество идентификацированное
    // 95 закрыто публичное сообщество идентификацированное

/////// Community //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Community {
    pub id:          i32,
    pub name:        String,
    pub status:      Option<String>,
    pub types:       i16,
    pub link:        String,
    pub s_avatar:    Option<String>,
    pub category_id: i32,
    pub user_id:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="communitys"]
pub struct NewCommunity {
    pub name:        String,
    pub types:       i16,
    pub link:        String,
    pub category_id: i32,
    pub user_id:     i32,
}

impl Community {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn count_communities() -> usize {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .select(schema::communitys::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }

    pub fn is_community(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "com".to_string() + &self.get_str_id();
    }

    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }

    pub fn get_info_model(&self) -> Result<CommunityInfo, Error> {
        let profile = self.find_info_model();
        if profile.is_ok() {
            return profile;
        }
        else {
            return self.create_info_model();
        }
    }
    pub fn find_info_model(&self) -> Result<CommunityInfo, Error> {
        use crate::schema::community_infos::dsl::community_infos;

        let _connection = establish_connection();
        let info = community_infos
            .filter(schema::community_infos::community_id.eq(self.id))
            .first(&_connection)?;
        return Ok(info);
    }
    pub fn create_info_model(&self) -> Result<CommunityInfo, Error> {
        use crate::schema::community_infos::dsl::community_infos;

        let _connection = establish_connection();

        let _new_community_info = NewCommunityInfo {
            community_id: self.id,
            avatar_id:    None,
            b_avatar:     None,
            status:       None,
            level:        100,
            cover:        None,
            created:      chrono::Local::now().naive_utc(),
            description:  None,
            members:      0,
        };
        let _community_info = diesel::insert_into(schema::community_infos::table)
            .values(&_new_community_info)
            .get_result::<CommunityInfo>(&_connection)?;

        return Ok(_community_info);
    }

    pub fn plus_members(&self, count: i32) -> () {
        let _connection = establish_connection();
        let profile = self.get_info_model();
        match profile {
          Ok(_ok) => diesel::update(&_ok)
              .set(schema::community_infos::members.eq(_ok.members + count))
              .execute(&_connection)
              .expect("Error."),
          Err(_error) => 0,
        };
    }
    pub fn minus_members(&self, count: i32) -> () {
        let _connection = establish_connection();
        let profile = self.get_info_model();
        match profile {
          Ok(_ok) => diesel::update(&_ok)
              .set(schema::community_infos::members.eq(_ok.members - count))
              .execute(&_connection)
              .expect("Error."),
          Err(_error) => 0,
        };
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

    pub fn create_banned_user(&self, user_id: i32) -> bool {
        let _connection = establish_connection();
        let new_banned_user = NewCommunityBannedUser {
            community_id: self.id,
            user_id:      user_id,
        };
        let banned_user = diesel::insert_into(schema::community_banned_users::table)
            .values(&new_banned_user)
            .execute(&_connection);

        if banned_user.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn delete_banned_user(&self, user_id: i32) -> bool {
        use crate::schema::community_banned_users::dsl::community_banned_users;

        let _connection = establish_connection();
        let banned_user = diesel::delete (
            community_banned_users
                .filter(schema::community_banned_users::community_id.eq(self.id))
                .filter(schema::community_banned_users::user_id.eq(user_id))
            )
            .execute(&_connection);

        if banned_user.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }

    // придется усложнить работу создания сообщества, в частности
    // создание подписчика:
    // 1. Сначала создаётся сообщество
    // 2. Из формы ссылка нового сообщества присылается
    // обратно для создания объекта стены и уведомлений, ведь
    // создатель должен получать их как админ
    pub fn create_community (
        name: String,
        category_id: i32,
        user_id: i32,
        types: i16
    ) -> NewCommunityJson {
        let _connection = establish_connection();
        let count = Community::count_communities() + 1;
        let link = "/public".to_string() + &count.to_string() + &"/".to_string();
        let new_community_form = NewCommunity {
                name:        name,
                types:       types,
                link:        link,
                category_id: category_id,
                user_id:     user_id,
            };
        let new_community = diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .get_result::<Community>(&_connection)
            .expect("Error.");

        let community_id = new_community.id;

        // создаем приватность нового сообщества
        let _private = NewCommunityPrivate {
            community_id: community_id,
            see_member:   1,
            see_info:     1,
            see_settings: 4,
            see_log:      4,
            see_stat:     4,
        };
        diesel::insert_into(schema::community_privates::table)
            .values(&_private)
            .execute(&_connection)
            .expect("Error saving community_private.");

        // создаем информацию нового сообщества
        let _info = NewCommunityInfo {
            community_id: community_id,
            avatar_id:    None,
            b_avatar:     None,
            status:       None,
            level:        100,
            cover:        None,
            created:      chrono::Local::now().naive_utc(),
            description:  None,
            members:      0,
        };
        diesel::insert_into(schema::community_infos::table)
            .values(&_info)
            .execute(&_connection)
            .expect("E.");

        // создаем уведомления нового сообщества
        let _community_notification = NewCommunityNotification {
            community_id:         community_id,
            connection_request:   true,
            connection_confirmed: true,
            community_invite:     true,
        };
        diesel::insert_into(schema::community_notifications::table)
            .values(&_community_notification)
            .execute(&_connection)
            .expect("Error saving community_notification.");

        CommunitiesMembership::create_membership (
            user_id,
            &new_community,
            5,
        );
        return NewCommunityJson {
            name:  new_community.name.clone(),
            types: new_community.types,
            link:  new_community.link.clone(),
        };
    }

    pub fn count_members(&self) -> i32 {
        let profile = self.get_info_model();
        return match profile {
          Ok(_ok) => _ok.members,
          Err(_error) => 0,
        };
    }
    pub fn count_members_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_members(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }
    pub fn count_members_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_members(),
            " подписчик".to_string(),
            " подписчика".to_string(),
            " подписчиков".to_string(),
        );
    }

    pub fn create_administrator(&self, user_id: i32) -> bool {
        // нужно создавать объект уведомлений для сообщества для нового админа
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member: Result<CommunitiesMembership, diesel::result::Error> = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first(&_connection);

        return match member {
             Ok(_ok) => {
                 diesel::update(&_ok)
                 .set(schema::communities_memberships::level.eq(5))
                 .execute(&_connection)
                 .expect("Error.");
                 return true;
             },
             Err(_error) => false,
        };
    }
    pub fn create_editor(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member: Result<CommunitiesMembership, diesel::result::Error> = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first(&_connection);

        return match member {
             Ok(_ok) => {
                 diesel::update(&_ok)
                 .set(schema::communities_memberships::level.eq(3))
                 .execute(&_connection)
                 .expect("Error.");
                 return true;
             },
             Err(_error) => false,
        };
    }
    pub fn create_moderator(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member: Result<CommunitiesMembership, diesel::result::Error> = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first(&_connection);

        return match member {
             Ok(_ok) => {
                 diesel::update(&_ok)
                 .set(schema::communities_memberships::level.eq(2))
                 .execute(&_connection)
                 .expect("Error.");
                 return true;
             },
             Err(_error) => false,
        };
    }
    pub fn create_advertisor(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member: Result<CommunitiesMembership, diesel::result::Error> = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first(&_connection);

        return match member {
             Ok(_ok) => {
                 diesel::update(&_ok)
                 .set(schema::communities_memberships::level.eq(4))
                 .execute(&_connection)
                 .expect("Error.");
                 return true;
             },
             Err(_error) => false,
        };
    }
    pub fn delete_staff_member(&self, user_id: i32) -> bool {
        // нужно удалять объект уведомлений для сообщества
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member: Result<CommunitiesMembership, diesel::result::Error> = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first(&_connection);

        return match member {
             Ok(_ok) => {
                 diesel::update(&_ok)
                 .set(schema::communities_memberships::level.eq(1))
                 .execute(&_connection)
                 .expect("Error.");
                 return true;
             },
             Err(_error) => false,
        };
    }

    pub fn get_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_6_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
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
            .filter(schema::communities_memberships::community_id.eq(self.id))
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
            .filter(schema::communities_memberships::community_id.eq(self.id))
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
            .filter(schema::communities_memberships::community_id.eq(self.id))
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
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(3))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_advertisers_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(4))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_info_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(12))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_see_info_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(2))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_info_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(12))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return _users;
    }
    pub fn get_see_info_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(2))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return _users;
    }

    pub fn get_see_member_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(11))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_see_member_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(1))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_member_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

          let _connection = establish_connection();
          let items = community_visible_perms
              .filter(schema::community_visible_perms::community_id.eq(self.id))
              .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
              .filter(schema::community_visible_perms::types.eq(11))
              .limit(limit)
              .offset(offset)
              .select(schema::community_visible_perms::target_id)
              .load::<i32>(&_connection)
              .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
      }
      pub fn get_see_member_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
          use crate::schema::{
              community_visible_perms::dsl::community_visible_perms,
              users::dsl::users,
          };

          let _connection = establish_connection();
          let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(1))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
    }

    pub fn get_see_settings_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(13))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_see_settings_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(3))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_settings_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

          let _connection = establish_connection();
          let items = community_visible_perms
              .filter(schema::community_visible_perms::community_id.eq(self.id))
              .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
              .filter(schema::community_visible_perms::types.eq(13))
              .limit(limit)
              .offset(offset)
              .select(schema::community_visible_perms::target_id)
              .load::<i32>(&_connection)
              .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
      }
      pub fn get_see_settings_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
          use crate::schema::{
              community_visible_perms::dsl::community_visible_perms,
              users::dsl::users,
          };

          let _connection = establish_connection();
          let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(3))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
    }

    pub fn get_see_log_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(14))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_see_log_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(4))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_log_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

          let _connection = establish_connection();
          let items = community_visible_perms
              .filter(schema::community_visible_perms::community_id.eq(self.id))
              .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
              .filter(schema::community_visible_perms::types.eq(14))
              .limit(limit)
              .offset(offset)
              .select(schema::community_visible_perms::target_id)
              .load::<i32>(&_connection)
              .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
      }
      pub fn get_see_log_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
          use crate::schema::{
              community_visible_perms::dsl::community_visible_perms,
              users::dsl::users,
          };

          let _connection = establish_connection();
          let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(4))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
    }

    pub fn get_see_stat_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(15))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }
    pub fn get_see_stat_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(5))
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");
        return items_ids;
    }

    pub fn get_see_stat_exclude(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

          let _connection = establish_connection();
          let items = community_visible_perms
              .filter(schema::community_visible_perms::community_id.eq(self.id))
              .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
              .filter(schema::community_visible_perms::types.eq(15))
              .limit(limit)
              .offset(offset)
              .select(schema::community_visible_perms::target_id)
              .load::<i32>(&_connection)
              .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
      }
      pub fn get_see_stat_include(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
          use crate::schema::{
              community_visible_perms::dsl::community_visible_perms,
              users::dsl::users,
          };

          let _connection = establish_connection();
          let items = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::target_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(5))
            .limit(limit)
            .offset(offset)
            .select(schema::community_visible_perms::target_id)
            .load::<i32>(&_connection)
            .expect("E");

          let _users = users
              .filter(schema::users::id.eq_any(items))
              .select((
                  schema::users::user_id,
                  schema::users::first_name,
                  schema::users::last_name,
                  schema::users::link,
                  schema::users::s_avatar.nullable(),
              ))
              .load::<CardUserJson>(&_connection)
              .expect("E");
          return _users;
    }

    pub fn get_members(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .limit(limit)
            .offset(offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }
    pub fn get_6_members(&self) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .limit(6)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_administrators(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(5))
            .limit(limit)
            .offset(offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_editors(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(3))
            .limit(limit)
            .offset(offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_moderators(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(2))
            .limit(limit)
            .offset(offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_advertisers(&self, limit: i64, offset: i64) -> Result<Vec<CardUserJson>, Error> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(4))
            .limit(limit)
            .offset(offset)
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)?;
        return Ok(_users);
    }

    pub fn get_private_model(&self) -> Result<CommunityPrivate, Error> {
        let private = self.find_private_model();
        if private.is_ok() {
            return private;
        }
        else {
            return self.create_private_model();
        }
    }
    pub fn create_private_model(&self) -> Result<CommunityPrivate, Error> {
        use crate::schema::community_privates::dsl::community_privates;

        let _connection = establish_connection();

        let _new_community_private = CommunityPrivate {
            community_id: self.id,
            see_member:   1,
            see_info:     1,
            see_settings: 4,
            see_log:      4,
            see_stat:     4,
        };
        let _community_private = diesel::insert_into(schema::community_privates::table)
            .values(&_new_community_private)
            .get_result::<CommunityPrivate>(&_connection)?;

        return Ok(_community_private);
    }
    pub fn find_private_model(&self) -> Result<CommunityPrivate, Error> {
        use crate::schema::community_privates::dsl::community_privates;

        let _connection = establish_connection();
        let private = community_privates
            .filter(schema::community_privates::community_id.eq(self.id))
            .first(&_connection)?;
        return Ok(private);
    }

    pub fn get_private_model_json(&self) -> Json<CommunityPrivateJson> {
        let private = Ok(self.get_private_model());
        let json = CommunityPrivateJson {
            see_member:   private.see_member,
            see_info:     private.see_info,
            see_settings: private.see_settings,
            see_log:      private.see_log,
            see_stat:     private.see_stat,
        };
        return Json(json);
    }

    pub fn is_user_see_info(&self, user_id: i32) -> bool {
        let private = Ok(self.get_private_model());
        return match private.see_info {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_info_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_info_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_see_member(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private.see_member {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_member_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_member_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_see_settings(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private.see_settings {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_settings_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_settings_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_see_log(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private.see_log {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_log_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_log_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_see_stat(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private.see_stat {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_anon_user_see_info(&self) -> bool {
        let private = self.get_private_model();
        return private.see_info == 1;
    }
    pub fn is_anon_user_see_member(&self) -> bool {
        let private = self.get_private_model();
        return private.see_member == 1;
    }
    pub fn is_anon_user_see_settings(&self) -> bool {
        let private = self.get_private_model();
        return private.see_settings == 1;
    }
    pub fn is_anon_user_see_log(&self) -> bool {
        let private = self.get_private_model();
        return private.see_log == 1;
    }
    pub fn is_anon_user_see_stat(&self) -> bool {
        let private = self.get_private_model();
        return private.see_stat == 1;
    }

    pub fn get_community_all_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == self.user_id {
            return vec![true, true, true, true, true];
        }
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();

        let bool_see_info = match private.see_info {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_see_info);

        let bool_see_member = match private.see_member {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_see_member);

        let bool_see_settings = match private.see_settings {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_see_settings);

        let bool_see_log = match private.see_log {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_see_log);

        let bool_see_stat = match private.see_stat {
            1 => true,
            2 => self.get_members_ids().iter().any(|&i| i==user_id),
            3 => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            4 => self.get_administrators_ids().iter().any(|&i| i==user_id),
            5 => self.user_id == user_id,
            6 => !self.get_see_stat_exclude_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            7 => self.get_see_stat_include_users_ids().iter().any(|&i| i==user_id) && self.get_members_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_see_stat);

        return bool_stack;
    }
    pub fn get_anon_community_all_see(&self) -> Vec<bool> {
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();

        bool_stack.push(private.see_info == 1);
        bool_stack.push(private.see_member == 1);
        bool_stack.push(private.see_stat == 1);

        return bool_stack;
    }

    pub fn get_follows_json(&self, page: i32, limit: i32) -> Json<UsersJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_follows_users(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_follows_users(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_follows_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        return Json(UsersJson {
            users: users,
            next_page: next_page_number,
        });
    }
    pub fn get_follows_users(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_follows::dsl::community_follows,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = community_follows
            .filter(schema::community_follows::community_id.eq(self.id))
            .limit(limit)
            .offset(offset)
            .select(schema::community_follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return _users;
    }

    pub fn get_banned_user_json(&self, page: i32, limit: i32) -> Json<UsersJson> {
        let mut next_page_number = 0;
        let users: Vec<CardUserJson>;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            users = self.get_banned_user(limit.into(), ((page - 1) * limit).into());
        }
        else {
            users = self.get_banned_user(limit.into(), 0);
            have_next = limit + 1;
        }
        if self.get_banned_user(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        return Json(UsersJson {
            users: users,
            next_page: next_page_number,
        });
    }
    pub fn get_banned_user(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
        use crate::schema::{
            community_banned_users::dsl::community_banned_users,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let items = community_banned_users
            .filter(schema::community_banned_users::community_id.eq(self.id))
            .limit(limit)
            .offset(offset)
            .select(schema::community_banned_users::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
        return _users;
    }


    pub fn set_members_visible_perms(&self, users: String, types: i16) -> bool {
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

        match types {
            1 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(11))
                )
                .execute(&_connection)
                .expect("E"),
            11 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(1))
                )
                .execute(&_connection)
                .expect("E"),
            2 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(12))
                )
                .execute(&_connection)
                .expect("E"),
            12 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(2))
                )
                .execute(&_connection)
                .expect("E"),
            3 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(13))
                )
                .execute(&_connection)
                .expect("E"),
            13 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(3))
                )
                .execute(&_connection)
                .expect("E"),
            4 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(14))
                )
                .execute(&_connection)
                .expect("E"),
            14 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(4))
                )
                .execute(&_connection)
                .expect("E"),
            5 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(15))
                )
                .execute(&_connection)
                .expect("E"),
            15 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(5))
                )
                .execute(&_connection)
                .expect("E"),
            _ => 0,
        };

        for user_id in users_ids.iter() {
            let _new_perm = NewCommunityVisiblePerm {
                community_id: self.id,
                target_id:    *user_id,
                types:        types,
            };
            diesel::insert_into(schema::community_visible_perms::table)
                .values(&_new_perm)
                .get_result::<CommunityVisiblePerm>(&_connection)
                .expect("Error.");
        }
        return true;
    }

}


/////// CommunityMembership //////
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
    pub created:      chrono::NaiveDateTime,
    pub visited:      i16,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="communities_memberships"]
pub struct NewCommunitiesMembership {
    pub user_id:      i32,
    pub community_id: i32,
    pub level:        i16,
    pub created:      chrono::NaiveDateTime,
    pub visited:      i16,
}
impl CommunitiesMembership {
    // придется усложнить работу создания подписчика:
    // 1. Сначала создаётся подписчик
    // 2. Затем на сервере пользователей создаются объекты стены и
    // уведомлений, таблицы то источников находятся там.

    //user.plus_communities(1);
    //user.plus_community_visited(community.id);
    //if is_administrator || is_editor || is_moderator {
    //    add_notification_community_subscriber(user.id);
    //}
    //add_new_community_subscriber(user.id);

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
            created:      chrono::Local::now().naive_utc(),
            visited:      0,
        };
        let new_member = diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member_form)
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("E.");

        community.plus_members(1);
        return new_member;
    }
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityInfo {
    pub id:           i32,
    pub community_id: i32,
    pub avatar_id:    Option<i32>,
    pub b_avatar:     Option<String>,
    pub status:       Option<String>,
    pub level:        i16,
    pub cover:        Option<String>,
    pub created:      chrono::NaiveDateTime,
    pub description:  Option<String>,
    pub members:      i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_infos"]
pub struct NewCommunityInfo {
    pub community_id: i32,
    pub avatar_id:    Option<i32>,
    pub b_avatar:     Option<String>,
    pub status:       Option<String>,
    pub level:        i16,
    pub cover:        Option<String>,
    pub created:      chrono::NaiveDateTime,
    pub description:  Option<String>,
    pub members:      i32,
}

/////// CommunityPrivate //////
    // 1 Все пользователи
    // 2 Подписчики
    // 3 Персонал
    // 4 Администраторы
    // 5 Владелец сообщества
    // 6 Подписчики, кроме
    // 7 Некоторые подписчики

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPrivate {
    pub id:           i32,
    pub community_id: i32,
    pub see_member:   i16,
    pub see_info:     i16,
    pub see_settings: i16,
    pub see_log:      i16,
    pub see_stat:     i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_privates"]
pub struct NewCommunityPrivate {
    pub community_id: i32,
    pub see_member:   i16,
    pub see_info:     i16,
    pub see_settings: i16,
    pub see_log:      i16,
    pub see_stat:     i16,
}

/////// CommunityNotifications //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityNotification {
    pub id:                   i32,
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_notifications"]
pub struct NewCommunityNotification {
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}


// включения и исключения для пользователей касательно конкретного сообщества
// 1 может видеть подписчиков
// 2 может видеть информацию
// 3 может видеть настройки
// 4 может видеть логи
// 5 может видеть статистику
// 11 не может видеть подписчиков
// 12 не может видеть информацию
// 13 не может видеть настройки
// 14 не может видеть логи
// 15 не может видеть статистику

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

/////// CommunityBannedUser //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityBannedUser {
    pub id:           i32,
    pub community_id: i32,
    pub user_id:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_banned_users"]
pub struct NewCommunityBannedUser {
    pub community_id: i32,
    pub user_id:      i32,
}

/////// CommunityFollow //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityFollow {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_follows"]
pub struct NewCommunityFollow {
    pub user_id:      i32,
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i16,
}

/////// FeaturedCommunitie //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct FeaturedCommunitie {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: i32,
    pub hidden:       bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="featured_communities"]
pub struct NewFeaturedCommunitie {
    pub user_id:      i32,
    pub community_id: i32,
    pub hidden:       bool,
}
