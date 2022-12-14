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

    featured_communities,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection, get_limit_offset, get_limit,
    CommunityCategoryJson, CardUserJson, KeyValue,
    CommunityPrivateJson, NewCommunityJson, PermList,
    AttachCommunityResp, CardCommunityJson,
    CommunityDetailJson, EditNotifyResp, EditCommunityPrivateResp,
};
use crate::errors::Error;
use crate::models::{
    TokenDetailJson, TokenJson, User, MembershipsList,
};

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

/*
Community

?????? ????????????????????
1 ?????????????????? ????????????????????
2 ???????????????? ????????????????????
3 ?????????????????? ????????????????????

7 ?????????????????? ???????????????????? ???????????? ????????????
8 ???????????????? ???????????????????? ???????????? ????????????
9 ?????????????????? ???????????????????? ???????????? ????????????

13 ?????????????????? ???????????????????? ????????????????????????????????????????
14 ???????????????? ???????????????????? ????????????????????????????????????????
15 ?????????????????? ???????????????????? ????????????????????????????????????????

21 ?????????????? ?????????????????? ????????????????????
22 ?????????????? ???????????????? ????????????????????
23 ?????????????? ?????????????????? ????????????????????

27 ?????????????? ?????????????????? ???????????????????? ???????????? ????????????
28 ?????????????? ???????????????? ???????????????????? ???????????? ????????????
29 ?????????????? ?????????????????? ???????????????????? ???????????? ????????????

33 ?????????????? ?????????????????? ???????????????????? ????????????????????????????????????????
34 ?????????????? ???????????????? ???????????????????? ????????????????????????????????????????
35 ?????????????? ?????????????????? ???????????????????? ????????????????????????????????????????

41 ???????????? ?????????????????? ????????????????????
42 ???????????? ???????????????? ????????????????????
43 ???????????? ?????????????????? ????????????????????

47 ???????????? ?????????????????? ???????????????????? ???????????? ????????????
48 ???????????? ???????????????? ???????????????????? ???????????? ????????????
49 ???????????? ?????????????????? ???????????????????? ???????????? ????????????

53 ???????????? ?????????????????? ???????????????????? ????????????????????????????????????????
54 ???????????? ???????????????? ???????????????????? ????????????????????????????????????????
55 ???????????? ?????????????????? ???????????????????? ????????????????????????????????????????

61 ?????????????? ?????????????????? ????????????????????
62 ?????????????? ???????????????? ????????????????????
63 ?????????????? ?????????????????? ????????????????????

67 ?????????????? ?????????????????? ???????????????????? ???????????? ????????????
68 ?????????????? ???????????????? ???????????????????? ???????????? ????????????
69 ?????????????? ?????????????????? ???????????????????? ???????????? ????????????

73 ?????????????? ?????????????????? ???????????????????? ????????????????????????????????????????
74 ?????????????? ???????????????? ???????????????????? ????????????????????????????????????????
75 ?????????????? ?????????????????? ???????????????????? ????????????????????????????????????????

81 ???????????????????????????? ?????????????????? ????????????????????
82 ???????????????????????????? ???????????????? ????????????????????
83 ???????????????????????????? ?????????????????? ????????????????????

87 ???????????????????????????? ?????????????????? ???????????????????? ???????????? ????????????
88 ???????????????????????????? ???????????????? ???????????????????? ???????????? ????????????
89 ???????????????????????????? ?????????????????? ???????????????????? ???????????? ????????????

93 ???????????????????????????? ?????????????????? ???????????????????? ????????????????????????????????????????
94 ???????????????????????????? ???????????????? ???????????????????? ????????????????????????????????????????
95 ???????????????????????????? ?????????????????? ???????????????????? ????????????????????????????????????????
*/

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
    pub lists:       i32,
    pub members:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="communitys"]
pub struct NewCommunity {
    pub name:        String,
    pub types:       i16,
    pub link:        String,
    pub category_id: i32,
    pub user_id:     i32,
    pub lists:       i32,
    pub members:     i32,
}

impl Community {
    pub fn get_memberships_lists (
        &self,
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> Vec<PermList> {
        use crate::schema::memberships_lists::dsl::memberships_lists;
  
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .order(schema::memberships_lists::position.desc())
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::memberships_lists::id,
                schema::memberships_lists::name,
                schema::memberships_lists::position,
                schema::memberships_lists::count
            ))
            .load::<PermList>(&_connection)
            .expect("E.");
    }
    pub fn is_have_memberships_lists(&self) -> bool {
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        return memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .filter(schema::memberships_lists::types.eq(5))
            .select(schema::memberships_lists::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_in_memberships_lists(&self, list_ids: Vec<i32>) -> bool {
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        return memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .filter(schema::memberships_lists::id.eq_any(list_ids))
            .select(schema::memberships_lists::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn get_memberships_lists_obj(&self) -> Vec<MembershipsList> {
        use crate::schema::memberships_lists::dsl::memberships_lists;
  
        let _connection = establish_connection();
        return memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .load::<MembershipsList>(&_connection)
            .expect("E.");
    }
    pub fn get_memberships_of_list (
        &self,
        list_id: Option<i32>, 
        limit:   Option<i64>,
        offset:  Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            memberships_list_items::dsl::memberships_list_items,
            users::dsl::users,
        };
        let current_list_id: i32;
        if list_id.is_some() {
            current_list_id = list_id.unwrap();
        }
        else {
            current_list_id = self.get_selected_list_id();
        }
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let users_ids = memberships_list_items
            .filter(schema::memberships_list_items::list_id.eq(current_list_id))
            .order(schema::memberships_list_items::visited.desc())
            .select(schema::memberships_list_items::user_id)
            .limit(_limit)
            .offset(_offset)
            .load::<i32>(&_connection)
            .expect("E.");
        return users
            .filter(schema::users::id.eq_any(users_ids))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
    }
    pub fn get_limit_members(&self, limit: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let _limit = get_limit(limit, 6);
        let _connection = establish_connection();
        let users_ids = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .order(schema::communities_memberships::visited.desc())
            .select(schema::communities_memberships::user_id)
            .limit(_limit)
            .load::<i32>(&_connection)
            .expect("E.");
        return users
            .filter(schema::users::id.eq_any(users_ids))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            )) 
            .load::<CardUserJson>(&_connection)
            .expect("E.");
    }

    pub fn get_selected_list_id(&self) -> i32 {
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        let list_id = memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .filter(schema::memberships_lists::types.eq(0))
            .order(schema::memberships_lists::position.desc())
            .select(schema::memberships_lists::id)
            .first::<i32>(&_connection);
        
        if list_id.is_ok() {
            return list_id.expect("E.");
        }
        else {
            return self.get_main_memberships_list().id;
        }
    }
    pub fn get_selected_list(&self) -> MembershipsList {
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        let list = memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .filter(schema::memberships_lists::types.eq(0))
            .order(schema::memberships_lists::position.desc())
            .first::<MembershipsList>(&_connection);
        
        if list.is_ok() {
            return list.expect("E.");
        }
        else {
            return self.get_main_memberships_list();
        }
    }

    pub fn get_main_memberships_list(&self) -> MembershipsList {
        use crate::schema::memberships_lists::dsl::memberships_lists;

        let _connection = establish_connection();
        let list = memberships_lists
            .filter(schema::memberships_lists::community_id.eq(self.id))
            .filter(schema::memberships_lists::types.eq(0))
            .first::<MembershipsList>(&_connection);
        
        if list.is_ok() {
            return list.expect("E.");
        }
        else {
            use crate::models::NewMembershipsList;

            let new_list_f = NewMembershipsList { 
                name:         "????????????????????".to_string(),
                community_id: self.id,
                types:        0,
                position:     1,
                count:        0,
                repost:       0, 
                see_el:       1
            };
            let new_list = diesel::insert_into(schema::memberships_lists::table)
                .values(&new_list_f)
                .get_result::<MembershipsList>(&_connection)
                .expect("Error.");
            
            self.plus_lists(1);
            return new_list;
        }
    }

    pub fn edit_name(&self, name: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::communitys::name.eq(name),
            ))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_status(&self, status: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set((  
                schema::communitys::status.eq(status),
            ))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit_link(&self, link: &str) -> i16 {
        let _connection = establish_connection();
        let _o = diesel::update(self)
            .set(schema::communitys::link.eq(link))
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn get_notify_model(&self) -> Result<CommunityNotification, Error> {
        let notify = self.find_notify_model();
        if notify.is_ok() {
            return notify;
        }
        else {
            return self.create_notify_model();
        }
    }
    pub fn create_notify_model(&self) -> Result<CommunityNotification, Error> {
        let _connection = establish_connection();
        let _new_notify = NewCommunityNotification {
            community_id:         self.id,
            connection_request:   1,
            connection_confirmed: 1,
            community_invite:     1
        };
        let _notify = diesel::insert_into(schema::community_notifications::table)
            .values(&_new_notify)
            .get_result::<CommunityNotification>(&_connection)?;

        return Ok(_notify);
    }
    pub fn find_notify_model(&self) -> Result<CommunityNotification, Error> {
        use crate::schema::community_notifications::dsl::community_notifications;

        let _connection = establish_connection();
        let notify = community_notifications
            .filter(schema::community_notifications::community_id.eq(self.id))
            .first(&_connection)?;
        return Ok(notify);
    }

    pub fn get_notify_json(&self) -> EditNotifyResp {
        let notify = self.get_notify_model().expect("E.");
        return EditNotifyResp {
            community_id:       self.id,
            connection_request: notify.connection_request,
            new_member:         notify.connection_confirmed,
        } 
    }
    pub fn get_private_field(value: i16) -> KeyValue {
        let info = match value {
            1 => "?????? ????????????????????????",
            2 => "????????????????????",
            3 => "????????????????",
            4 => "????????????????????????????",
            5 => "???????????????? ????????????????????",
            6 => "????????????????????, ??????????",
            7 => "?????????????????? ????????????????????",
            _ => "????????????",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    }
    pub fn get_notify_field(value: i16) -> KeyValue {
        let info = match value {
            1 => "?????? ????????????????????????",
            2 => "????????????????????",
            3 => "????????????????",
            4 => "????????????????????????????",
            5 => "??????????????????",
            6 => "????????????????????, ??????????",
            7 => "?????????????????? ????????????????????",
            _ => "????????????",
        };
        return KeyValue {
            value: value,
            info:  info.to_string(),
        }
    }
    pub fn get_private_json(&self) -> EditCommunityPrivateResp {
        let see_member_members:   Option<Vec<CardUserJson>>;
        let see_info_members:     Option<Vec<CardUserJson>>;
        let see_settings_members: Option<Vec<CardUserJson>>;
        let see_log_members:      Option<Vec<CardUserJson>>;
        let see_stat_members:     Option<Vec<CardUserJson>>;

        let private = self.get_private_model().expect("E.");
        
        if private.see_member == 6 {
            see_member_members = Some(self.get_limit_see_member_exclude_members(Some(20), Some(0)));
        }
        else if private.see_member == 7 {
            see_member_members = Some(self.get_limit_see_member_include_members(Some(20), Some(0)));
        }
        else {
            see_member_members = None;
        }

        if private.see_info == 6 {
            see_info_members = Some(self.get_limit_see_info_exclude_members(Some(20), Some(0)));
        }
        else if private.see_info == 7 {
            see_info_members = Some(self.get_limit_see_info_include_members(Some(20), Some(0)));
        }
        else {
            see_info_members = None;
        }

        if private.see_settings == 6 {
            see_settings_members = Some(self.get_limit_see_settings_exclude_members(Some(20), Some(0)));
        }
        else if private.see_settings == 7 {
            see_settings_members = Some(self.get_limit_see_settings_include_members(Some(20), Some(0)));
        }
        else {
            see_settings_members = None;
        }

        if private.see_log == 6 {
            see_log_members = Some(self.get_limit_see_log_exclude_members(Some(20), Some(0)));
        }
        else if private.see_log == 7 {
            see_log_members = Some(self.get_limit_see_log_include_members(Some(20), Some(0)));
        }
        else {
            see_log_members = None;
        }

        if private.see_stat == 6 {
            see_stat_members = Some(self.get_limit_see_stat_exclude_members(Some(20), Some(0)));
        }
        else if private.see_stat == 7 {
            see_stat_members = Some(self.get_limit_see_stat_include_members(Some(20), Some(0)));
        }
        else {
            see_stat_members = None;
        }
    
        return EditCommunityPrivateResp {
            see_member:           Community::get_private_field(private.see_member),
            see_info:             Community::get_private_field(private.see_info),
            see_settings:         Community::get_private_field(private.see_settings),
            see_log:              Community::get_private_field(private.see_log),
            see_stat:             Community::get_private_field(private.see_stat),
            see_member_members:   see_member_members,
            see_info_members:     see_info_members,
            see_settings_members: see_settings_members,
            see_log_members:      see_log_members,
            see_stat_members:     see_stat_members,
        };
    }
    pub fn is_identified_send(&self) -> bool {
        return self.types > 6 && self.types < 10;
    }
    pub fn is_identified(&self) -> bool {
        return self.types > 13 && self.types < 16;
    }

    pub fn get_community_detail_json(&self) -> CommunityDetailJson {
        let description: Option<String>;
        let cover: Option<String>; 
        let image: Option<String>;
        let avatar_id: Option<i32>;

        let info = self.get_info_model();
        match info {
          Ok(_ok) => {
            description = _ok.description; 
            cover = _ok.cover;
            image = _ok.b_avatar;
            avatar_id = _ok.avatar_id;
          },
          Err(_error) => {
            description = None;
            cover = None;
            image = None;
            avatar_id = None;
          },
        };

        let identified: i16;
        if self.is_identified() {
            identified = 1;
        }
        else {
            identified = 0;
        }

        let user_json = CommunityDetailJson {
             id:          self.id, 
             name:        self.name.clone(),
             status:      self.status.clone(),
             slug:        self.get_slug(),
             description: description,
             cover:       cover,
             image:       image,
             avatar_id:   avatar_id,
             identified:  identified,
         };
         return user_json;
    }

    pub fn get_all_communities (
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardCommunityJson> {
        use crate::schema::communitys::dsl::communitys;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::types.lt(20))
            .select((
                schema::communitys::user_id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::s_avatar.nullable(),
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
            .expect("E.");
    }
    pub fn search_all_communities (
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardCommunityJson> {
        use crate::schema::communitys::dsl::communitys;

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::name.ilike(&q))
            .filter(schema::communitys::types.lt(20))
            .select((
                schema::communitys::user_id,
                schema::communitys::name,
                schema::communitys::link,
                schema::communitys::s_avatar.nullable(),
                schema::communitys::members,
            ))
            .load::<CardCommunityJson>(&_connection)
            .expect("E.");
    }
    pub fn get_communities_for_attach(ids: Vec<i32>) -> Vec<AttachCommunityResp> {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
 
        return communitys
            .filter(schema::communitys::id.eq_any(ids))
            .filter(schema::communitys::types.lt(21))
            .select((
                schema::communitys::id,
                schema::communitys::name,
                schema::communitys::types,
                schema::communitys::link,
                schema::communitys::s_avatar,
            ))
            .load::<AttachCommunityResp>(&_connection)
            .expect("E.");
    }  

    pub fn edit_notify ( 
        &self, 
        field: &str, 
        value: i16
    ) -> i16 {
        let _connection = establish_connection();
        let notify = self.get_notify_model().expect("E.");
        let _update_field = match field {
            "connection_request" => diesel::update(&notify)
                .set(schema::community_notifications::connection_request.eq(value))
                .execute(&_connection)
                .expect("E."),
            "new_member" => diesel::update(&notify)
                .set(schema::community_notifications::connection_confirmed.eq(value))
                .execute(&_connection)
                .expect("E."),
            "community_invite" => diesel::update(&notify)
                .set(schema::community_notifications::community_invite.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
        };
        return 1;
    }

    pub fn edit_private (
        &self, 
        field:     &str, 
        value:     i16, 
        items_ids: Option<Vec<i32>>
    ) -> i16 {
        let is_ie_mode = vec![6,7].iter().any(|&i| i==value);
        if value < 1 || value > 120 || is_ie_mode && items_ids.is_none() {
            return 0;
        }

        let _connection = establish_connection();
        let private = self.get_private_model().expect("E.");
        let _update_field = match field {
            "see_member" => diesel::update(&private)
                .set(schema::community_privates::see_member.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_info" => diesel::update(&private)
                .set(schema::community_privates::see_info.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_settings" => diesel::update(&private)
                .set(schema::community_privates::see_settings.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_log" => diesel::update(&private)
                .set(schema::community_privates::see_log.eq(value))
                .execute(&_connection)
                .expect("E."),
            "see_stat" => diesel::update(&private)
                .set(schema::community_privates::see_stat.eq(value))
                .execute(&_connection)
                .expect("E."),
            _ => 0,
        };

        // ?????????? ?????????????? ???? ???????????? ??????, ?????? ?????? ???????? ????????????
        // ?? ???????????????????????????????? ??????????????.
        if is_ie_mode {
            use crate::schema::community_visible_perms::dsl::community_visible_perms;

            match value {
                1 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(11))
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
                3 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(13))
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
                5 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(15))
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
                12 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(2))
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
                14 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(4))
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
                101 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(111))
                    )
                    .execute(&_connection)
                    .expect("E"),
                102 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(112))
                    )
                    .execute(&_connection)
                    .expect("E"),
                103 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(113))
                    )
                    .execute(&_connection)
                    .expect("E"),
                104 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(114))
                    )
                    .execute(&_connection)
                    .expect("E"),
                105 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(115))
                    )
                    .execute(&_connection)
                    .expect("E"),
                111 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(101))
                    )
                    .execute(&_connection)
                    .expect("E"),
                112 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(102))
                    )
                    .execute(&_connection)
                    .expect("E"),
                113 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(103))
                    )
                    .execute(&_connection)
                    .expect("E"),
                114 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(104))
                    )
                    .execute(&_connection)
                    .expect("E"),
                115 => diesel::delete (
                    community_visible_perms
                        .filter(schema::community_visible_perms::community_id.eq(self.id))
                        .filter(schema::community_visible_perms::types.eq(105))
                    )
                    .execute(&_connection)
                    .expect("E"),
                _ => 0,
            };
        }

        if items_ids.is_some() {
            for item_id in items_ids.unwrap().iter() {
                let _new_perm = NewCommunityVisiblePerm {
                    community_id: self.id,
                    item_id:      *item_id,
                    types:        value,
                };
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }

    pub fn suspend_item(&self) -> i16 {
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

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn delete_item(&self) -> i16 {
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

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn restore_item(&self) -> i16 {
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

            if o.is_ok() {
                return 1;
            }
            else {
                return 0;
            }
        }
        return 0;
    }
    pub fn get_longest_penalties(&self) -> String {
        return "".to_string();
    }
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
        };
        let _community_info = diesel::insert_into(schema::community_infos::table)
            .values(&_new_community_info)
            .get_result::<CommunityInfo>(&_connection)?;

        return Ok(_community_info);
    }

    pub fn plus_members(&self, count: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::communitys::members.eq(self.members + count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus_members(&self, count: i32) -> () {
        if (self.members + count) >= 0 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(schema::communitys::members.eq(self.members - count))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn plus_lists(&self, count: i32) -> bool {
        let _connection = establish_connection();
        let _u = diesel::update(self)
            .set(schema::communitys::lists.eq(self.lists + count))
            .execute(&_connection);
        if _u.is_ok() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn minus_lists(&self, count: i32) -> bool {
        if self.lists > 0 {
            let _connection = establish_connection();
            let _u = diesel::update(self)
                .set(schema::communitys::lists.eq(self.lists - count))
                .execute(&_connection);
            if _u.is_ok() {
                return true;
            }
            else {
                return false;
            }
        }
        return false;
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
        return self.types == 3 || self.types == 9 || self.types == 15;
    }
    pub fn is_close(&self) -> bool {
        return self.types == 2 || self.types == 8 || self.types == 14;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 1 || self.types == 7 || self.types == 13;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 20;
    }

    pub fn create_banned_user(&self, user: User, ban_types: i16) -> i16 {
        use chrono::Duration;

        let ban_to: Option<chrono::NaiveDateTime> = match ban_types {
            1 => Some(chrono::Local::now().naive_utc() + Duration::hours(1)),
            2 => Some(chrono::Local::now().naive_utc() + Duration::days(1)),
            3 => Some(chrono::Local::now().naive_utc() + Duration::days(7)),
            4 => Some(chrono::Local::now().naive_utc() + Duration::days(30)),
            5 => Some(chrono::Local::now().naive_utc() + Duration::days(366)),
            _ => None,
        };

        let _connection = establish_connection();
        let new_banned_user = NewCommunityBannedUser {
            community_id: self.id,
            user_id:      user.id,
            ban_to:       ban_to,
        };
        let banned_user = diesel::insert_into(schema::community_banned_users::table)
            .values(&new_banned_user)
            .execute(&_connection);
        if ban_to.is_none() { 
            user.leave_community(self);
        }
        if banned_user.is_ok() {
            return 1; 
        }
        else {
            return 0;
        }
    }
    pub fn delete_banned_user(&self, user: User) -> i16 {
        use crate::schema::community_banned_users::dsl::community_banned_users;

        let _connection = establish_connection();
        //let ban_user = community_banned_users
        //    .filter(schema::community_banned_users::community_id.eq(self.id))
        //    .filter(schema::community_banned_users::user_id.eq(user.id))
        //    .first::<CommunityBannedUser>(&_connection)
        //    .expect("E.");

        let banned_user = diesel::delete (
            community_banned_users
                .filter(schema::community_banned_users::community_id.eq(self.id))
                .filter(schema::community_banned_users::user_id.eq(user.id))
            )
            .execute(&_connection);

        if banned_user.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn is_user_in_ban(&self, user_id: i32) -> bool {
        use crate::schema::community_banned_users::dsl::community_banned_users;

        let _connection = establish_connection();
        return community_banned_users
            .filter(schema::community_banned_users::user_id.eq(user_id))
            .filter(schema::community_banned_users::community_id.eq(self.id))
            .select(schema::community_banned_users::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

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
            lists:       0,
            members:     0,
        };
        let new_community = diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .get_result::<Community>(&_connection)
            .expect("Error.");

        let community_id = new_community.id;

        // ?????????????? ?????????????????????? ???????????? ????????????????????
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

        // ?????????????? ???????????????????? ???????????? ????????????????????
        let _info = NewCommunityInfo {
            community_id: community_id,
            avatar_id:    None,
            b_avatar:     None,
            status:       None,
            level:        100,
            cover:        None,
            created:      chrono::Local::now().naive_utc(),
            description:  None,
        };
        diesel::insert_into(schema::community_infos::table)
            .values(&_info)
            .execute(&_connection)
            .expect("E.");

        // ?????????????? ?????????????????????? ???????????? ????????????????????
        let _community_notification = NewCommunityNotification {
            community_id:         community_id,
            connection_request:   1,
            connection_confirmed: 1,
            community_invite:     1, 
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

        // ?????????????? ???????????????? ???????????? ?????????????????????? ????????????????????
        use crate::models::NewMembershipsList;
        let new_list_f = NewMembershipsList {
            name:         "????????????????????".to_string(),
            community_id: community_id,
            types:        0,
            position:     0,
            count:        0,
            repost:       0,
            see_el:       1,
        };
        diesel::insert_into(schema::memberships_lists::table)
            .values(&new_list_f)
            .execute(&_connection)
            .expect("Error.");

        return NewCommunityJson {
            name:  new_community.name.clone(),
            types: new_community.types,
            link:  new_community.link.clone(),
        };
    }

    pub fn count_members(&self) -> i32 {
        return self.members;
    }
    pub fn count_members_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_members(),
            " ??????????????????".to_string(),
            " ????????????????????".to_string(),
            " ??????????????????????".to_string(),
        );
    }
    pub fn count_members_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_members(),
            " ??????????????????".to_string(),
            " ????????????????????".to_string(),
            " ??????????????????????".to_string(),
        );
    }

    pub fn count_lists(&self) -> i32 {
        return self.lists;
    }
    pub fn count_lists_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_lists(),
            " ????????????".to_string(),
            " ????????????".to_string(),
            " ??????????????".to_string(),
        );
    }
    pub fn count_lists_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt (
            self.count_lists(),
            " ????????????".to_string(),
            " ????????????".to_string(),
            " ??????????????".to_string(),
        );
    }

    pub fn create_administrator(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(5))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
        };
    }
    pub fn create_editor(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(3))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
        };
    }
    pub fn create_moderator(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(2))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
        };
    }
    pub fn create_advertiser(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(4))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
        };
    }
    pub fn delete_staff_member(&self, user_id: i32) -> i16 {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .first::<CommunitiesMembership>(&_connection);

        return match member {
            Ok(_ok) => {
                diesel::update(&_ok)
                    .set(schema::communities_memberships::level.eq(1))
                    .execute(&_connection)
                    .expect("Error.");
                return 1;
            },
            Err(_error) => 0,
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

    pub fn get_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_members (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_6_members(&self) -> Vec<CardUserJson> {
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_administrators(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(5))
            .limit(_limit)
            .offset(_offset)
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_administrators (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(5))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_editors(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(3))
            .limit(_limit)
            .offset(_offset)
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_editors (
        &self,
        q:      &String,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(3))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_moderators(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(2))
            .limit(_limit)
            .offset(_offset)
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_moderators (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(2))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_advertisers(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(4))
            .limit(_limit)
            .offset(_offset)
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
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }
    pub fn search_advertisers (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            communities_memberships::dsl::communities_memberships,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(4))
            .select(schema::communities_memberships::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
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
        let _connection = establish_connection();

        let _new_community_private = NewCommunityPrivate {
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

    pub fn get_private_model_json(&self) -> Result<CommunityPrivateJson, Error> {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => Ok(CommunityPrivateJson {
              see_member:   _ok.see_member,
              see_info:     _ok.see_info,
              see_settings: _ok.see_settings,
              see_log:      _ok.see_log,
              see_stat:     _ok.see_stat,
          }),
          Err(_error) => Err(_error),
        };
    }

    pub fn is_user_member(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }
    pub fn is_user_staff(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        return communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.id))
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
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::level.eq(5))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
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
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::item_id.eq(user_id))
            .filter(schema::community_visible_perms::types.eq(types))
            .select(schema::community_visible_perms::item_id)
            .first::<i32>(&_connection)
            .is_ok() &&
        communities_memberships
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_ok();
    }

    pub fn is_user_list_perm_exists (
        &self,
        user_id: i32,
        types:   i16, 
    ) -> bool { 
        // ??????????????????, ???????? ???? ???????????????????????? ?? ???????????? ??????????????????????,
        // ?????????????? ?????????? ?????? ???? ???????????? ?????? ???? ???????????? (types)
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            communities_memberships::dsl::communities_memberships,
        };

        let _connection = establish_connection();
        let list_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::types.eq(types))
            .select(schema::community_visible_perms::id)
            .load::<i32>(&_connection);
        if communities_memberships 
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .select(schema::communities_memberships::id)
            .first::<i32>(&_connection)
            .is_err() || list_ids.is_err() {
            return false;
        };
        return self.is_user_in_memberships_lists(list_ids.expect("E."));
    }

    pub fn get_ie_members_for_types (
        &self, 
        types:  i16,
        limit:  Option<i64>, 
        offset: Option<i64>,
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            community_visible_perms::dsl::community_visible_perms,
            users::dsl::users,
        };

        let _connection = establish_connection();
        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let items_ids = community_visible_perms
            .filter(schema::community_visible_perms::community_id.eq(self.id))
            .filter(schema::community_visible_perms::item_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::types.eq(types))
            .limit(_limit)
            .offset(_offset)
            .select(schema::community_visible_perms::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        return users
            .filter(schema::users::id.eq_any(items_ids))
            .filter(schema::users::types.lt(31))
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar,
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E");
    }

    pub fn get_limit_see_member_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(11, limit, offset); 
    }
    pub fn get_limit_see_member_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(1, limit, offset); 
    }
    pub fn get_limit_see_info_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(12, limit, offset); 
    }
    pub fn get_limit_see_info_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(2, limit, offset); 
    }
    pub fn get_limit_see_settings_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(13, limit, offset); 
    }
    pub fn get_limit_see_settings_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(3, limit, offset); 
    }
    pub fn get_limit_see_log_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(14, limit, offset); 
    }
    pub fn get_limit_see_log_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(4, limit, offset); 
    }
    pub fn get_limit_see_stat_exclude_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(15, limit, offset); 
    }
    pub fn get_limit_see_stat_include_members(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        return self.get_ie_members_for_types(5, limit, offset); 
    }

    pub fn is_user_see_info(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_info {
              1 => true,
              2 => self.is_user_member(user_id),
              3 => self.is_user_staff(user_id),
              4 => self.is_user_admin(user_id),
              5 => self.user_id == user_id,
              6 => !self.is_user_perm_exists(user_id, 12),
              7 => self.is_user_perm_exists(user_id, 2),
              8 => !self.is_user_list_perm_exists(user_id, 112),
              9 => self.is_user_list_perm_exists(user_id, 102),
              _ => false},
          Err(_) => false,
        };
    }

    pub fn is_user_see_member(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_member {
              1 => true,
              2 => self.is_user_member(user_id),
              3 => self.is_user_staff(user_id),
              4 => self.is_user_admin(user_id),
              5 => self.user_id == user_id,
              6 => !self.is_user_perm_exists(user_id, 11),
              7 => self.is_user_perm_exists(user_id, 1),
              8 => !self.is_user_list_perm_exists(user_id, 111),
              9 => self.is_user_list_perm_exists(user_id, 101),
              _ => false},
          Err(_) => false,
        };
    }
    pub fn is_user_see_settings(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_settings {
              1 => true,
              2 => self.is_user_member(user_id),
              3 => self.is_user_staff(user_id),
              4 => self.is_user_admin(user_id),
              5 => self.user_id == user_id,
              6 => !self.is_user_perm_exists(user_id, 13),
              7 => self.is_user_perm_exists(user_id, 3),
              8 => !self.is_user_list_perm_exists(user_id, 113),
              9 => self.is_user_list_perm_exists(user_id, 103),
              _ => false},
          Err(_) => false,
        };
    }
    pub fn is_user_see_log(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_log {
              1 => true,
              2 => self.is_user_member(user_id),
              3 => self.is_user_staff(user_id),
              4 => self.is_user_admin(user_id),
              5 => self.user_id == user_id,
              6 => !self.is_user_perm_exists(user_id, 14),
              7 => self.is_user_perm_exists(user_id, 4),
              8 => !self.is_user_list_perm_exists(user_id, 114),
              9 => self.is_user_list_perm_exists(user_id, 104),
              _ => false},
          Err(_) => false,
        };
    }
    pub fn is_user_see_stat(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        return match private {
          Ok(_ok) => match _ok.see_stat {
              1 => true,
              2 => self.is_user_member(user_id),
              3 => self.is_user_staff(user_id),
              4 => self.is_user_admin(user_id),
              5 => self.user_id == user_id,
              6 => !self.is_user_perm_exists(user_id, 15),
              7 => self.is_user_perm_exists(user_id, 5),
              8 => !self.is_user_list_perm_exists(user_id, 115),
              9 => self.is_user_list_perm_exists(user_id, 105),
              _ => false},
          Err(_) => false,
        };
    }

    pub fn is_anon_user_see_info(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_info == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_member(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_member == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_settings(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_settings == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_log(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_log == 1,
            Err(_) => false,
        }
    }
    pub fn is_anon_user_see_stat(&self) -> bool {
        let private = self.get_private_model();
        return match private {
            Ok(_ok) => _ok.see_stat == 1,
            Err(_) => false,
        }
    }

    pub fn get_community_all_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == self.user_id {
            return vec![true, true, true, true, true];
        }
        let private = self.get_private_model();
        let mut bool_stack = Vec::new();

        return match private {
          Ok(_ok) => {
            let bool_see_info = match _ok.see_info {
                1 => true,
                2 => self.is_user_member(user_id),
                3 => self.is_user_staff(user_id),
                4 => self.is_user_admin(user_id),
                5 => self.user_id == user_id,
                6 => !self.is_user_perm_exists(user_id, 12),
                7 => self.is_user_perm_exists(user_id, 2),
                8 => !self.is_user_list_perm_exists(user_id, 112),
                9 => self.is_user_list_perm_exists(user_id, 102),
                _ => false
             };
             let bool_see_member = match _ok.see_member {
                1 => true,
                2 => self.is_user_member(user_id),
                3 => self.is_user_staff(user_id),
                4 => self.is_user_admin(user_id),
                5 => self.user_id == user_id,
                6 => !self.is_user_perm_exists(user_id, 11),
                7 => self.is_user_perm_exists(user_id, 1),
                8 => !self.is_user_list_perm_exists(user_id, 111),
                9 => self.is_user_list_perm_exists(user_id, 101),
                _ => false
             };
             let bool_see_settings = match _ok.see_settings {
                1 => true,
                2 => self.is_user_member(user_id),
                3 => self.is_user_staff(user_id),
                4 => self.is_user_admin(user_id),
                5 => self.user_id == user_id,
                6 => !self.is_user_perm_exists(user_id, 13),
                7 => self.is_user_perm_exists(user_id, 3),
                8 => !self.is_user_list_perm_exists(user_id, 113),
                9 => self.is_user_list_perm_exists(user_id, 103),
                _ => false
            };
            let bool_see_log = match _ok.see_log {
                1 => true,
                2 => self.is_user_member(user_id),
                3 => self.is_user_staff(user_id),
                4 => self.is_user_admin(user_id),
                5 => self.user_id == user_id,
                6 => !self.is_user_perm_exists(user_id, 14),
                7 => self.is_user_perm_exists(user_id, 4),
                8 => !self.is_user_list_perm_exists(user_id, 114),
                9 => self.is_user_list_perm_exists(user_id, 104),
                _ => false
            };

            let bool_see_stat = match _ok.see_stat {
                1 => true,
                2 => self.is_user_member(user_id),
                3 => self.is_user_staff(user_id),
                4 => self.is_user_admin(user_id),
                5 => self.user_id == user_id,
                6 => !self.is_user_perm_exists(user_id, 15),
                7 => self.is_user_perm_exists(user_id, 5),
                _ => false
            };
            bool_stack.push(bool_see_info);
            bool_stack.push(bool_see_member);
            bool_stack.push(bool_see_settings);
            bool_stack.push(bool_see_log);
            bool_stack.push(bool_see_stat);
            bool_stack
          },
          Err(_) => vec![false, false, false, false, false],
        }
    }
    pub fn get_anon_community_all_see(&self) -> Vec<bool> {
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();
        return match private {
            Ok(_ok) => {
                bool_stack.push(_ok.see_info == 1);
                bool_stack.push(_ok.see_member == 1);
                bool_stack.push(_ok.see_settings == 1);
                bool_stack.push(_ok.see_log == 1);
                bool_stack.push(_ok.see_stat == 1);
                bool_stack
            },
            Err(_) => vec![false, false, false, false, false],
        }
    }

    pub fn get_follows_users(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            community_follows::dsl::community_follows,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = community_follows
            .filter(schema::community_follows::community_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
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
            .expect("E.");
        return _users;
    }
    pub fn search_follows_users (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            community_follows::dsl::community_follows,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = community_follows
            .filter(schema::community_follows::community_id.eq(self.id))
            .select(schema::community_follows::user_id)
            .load::<i32>(&_connection)
            .expect("E.");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_banned_user(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<CardUserJson> {
        use crate::schema::{
            community_banned_users::dsl::community_banned_users,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = community_banned_users
            .filter(schema::community_banned_users::community_id.eq(self.id))
            .limit(_limit)
            .offset(_offset)
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
            .expect("E.");
        return _users;
    }
    pub fn search_banned_user (
        &self,
        q:      &String,
        limit:  Option<i64>,
        offset: Option<i64>
    ) -> Vec<CardUserJson> {
        use crate::schema::{
            community_banned_users::dsl::community_banned_users,
            users::dsl::users,
        };

        let (_limit, _offset) = get_limit_offset(limit, offset, 20);
        let _connection = establish_connection();
        let items = community_banned_users
            .filter(schema::community_banned_users::community_id.eq(self.id))
            .select(schema::community_banned_users::user_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _users = users
            .filter(schema::users::id.eq_any(items))
            .filter(schema::users::first_name.ilike(&q))
            .or_filter(schema::users::last_name.ilike(&q))
            .limit(_limit)
            .offset(_offset)
            .select((
                schema::users::user_id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::link,
                schema::users::s_avatar.nullable(),
            ))
            .load::<CardUserJson>(&_connection)
            .expect("E.");
        return _users;
    }

    pub fn get_token_detail(&self, token_id: i32) -> TokenDetailJson {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection(); 
        let _token = owners
            .filter(schema::owners::id.eq(token_id))
            .filter(schema::owners::types.eq(3))
            .first::<Owner>(&_connection)
            .expect("E.");

        let mut services = Vec::new();
        for s in _token.get_services().iter() {
            services.push (TokenServiceJson {
                id:   s.id,
                name: s.name.clone(),
            });
        }

        return TokenDetailJson {
            id:          _token.id,
            name:        _token.name.clone(),
            description: _token.description.clone(),
            is_active:   _token.is_active,
            services:    services,
        }
    }

    pub fn get_tokens(&self) -> Vec<TokenJson> {
        use crate::schema::owners::dsl::owners;
        use crate::models::{Owner, TokenServiceJson};

        let _connection = establish_connection();
        let mut list = Vec::new();

        let _tokens = owners
            .filter(schema::owners::user_id.eq(self.id))
            .filter(schema::owners::types.eq(3))
            .load::<Owner>(&_connection)
            .expect("E.");

        for i in _tokens.iter() {
            let mut services = Vec::new();
            for s in i.get_services().iter() {
                services.push (TokenServiceJson {
                    id:   s.id,
                    name: s.name.clone(),
                });
            }
            list.push (
                TokenJson {
                    id:        i.id,
                    name:      i.name.clone(),
                    is_active: i.is_active,
                    services:  services,
                }
            );
        }

        return list;
    }
}

/*
CommunityMembership
level
1 ??????????????????
2 ??????????????????
3 ????????????????
4 ??????????????????
5 ??????????????????????????
*/
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
    pub fn create_membership (
        user_id: i32,
        community: &Community,
        level: i16
    ) -> bool {
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
            .execute(&_connection);

        if new_member.is_ok() {
            community.plus_members(1);
            return true;
        }
        else {
            return false;
        }
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
}

/*
CommunityPrivate //////
1 ?????? ????????????????????????
2 ????????????????????
3 ????????????????
4 ????????????????????????????
5 ???????????????? ????????????????????
6 ????????????????????, ??????????
7 ?????????????????? ????????????????????
*/
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
    pub connection_request:   i16,
    pub connection_confirmed: i16,
    pub community_invite:     i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_notifications"]
pub struct NewCommunityNotification {
    pub community_id:         i32,
    pub connection_request:   i16,
    pub connection_confirmed: i16,
    pub community_invite:     i16,
}

/*
?????????????????? ?? ???????????????????? ?????? ?????????????????????????? ???????????????????? ?????????????????????? ????????????????????
1 ?????????? ???????????? ??????????????????????
2 ?????????? ???????????? ????????????????????
3 ?????????? ???????????? ??????????????????
4 ?????????? ???????????? ????????
5 ?????????? ???????????? ????????????????????
11 ???? ?????????? ???????????? ??????????????????????
12 ???? ?????????? ???????????? ????????????????????
13 ???? ?????????? ???????????? ??????????????????
14 ???? ?????????? ???????????? ????????
15 ???? ?????????? ???????????? ????????????????????

101 ???????????? ?????????? ???????????? ??????????????????????
102 ???????????? ?????????? ???????????? ????????????????????
103 ???????????? ?????????? ???????????? ??????????????????
104 ???????????? ?????????? ???????????? ????????
105 ???????????? ?????????? ???????????? ????????????????????
111 ???????????? ???? ?????????? ???????????? ??????????????????????
112 ???????????? ???? ?????????? ???????????? ????????????????????
113 ???????????? ???? ?????????? ???????????? ??????????????????
114 ???????????? ???? ?????????? ???????????? ????????
115 ???????????? ???? ?????????? ???????????? ????????????????????
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityVisiblePerm {
    pub id:           i32,
    pub community_id: i32, // ?????????? ???????????????????? ??????????????????
    pub item_id:      i32, // ???????????? ???????????????????????? ??????????????????
    pub types:        i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_visible_perms"]
pub struct NewCommunityVisiblePerm {
    pub community_id: i32,
    pub item_id:      i32,
    pub types:        i16,
}

/////// CommunityBannedUser //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityBannedUser {
    pub id:           i32,
    pub community_id: i32,
    pub user_id:      i32,
    pub ban_to:       Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_banned_users"]
pub struct NewCommunityBannedUser {
    pub community_id: i32,
    pub user_id:      i32,
    pub ban_to:       Option<chrono::NaiveDateTime>,
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
