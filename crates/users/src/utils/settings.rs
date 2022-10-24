use crate::utils::{
    establish_connection,
    ListsUserCommunitiesJson,
    UniversalUserCommunityKeysJson,
    DesignSettingsJson,
    UserPrivateJson,
    UserProfileNotificationJson,
    UserPopulateStickerJson,
    UserPopulateSmileJson,
    UserVisiblePermJson,
    PhoneCodeJson,
    UsersListJson,
    CardUserJson,
};
use actix_web::web::Json;


pub fn get_blocked_users_json(&self, page: i32, limit: i32) -> Json<UsersListJson> {
    let mut next_page_number = 0;
    let users: Vec<CardUserJson>;
    let have_next: i32;

    if page > 1 {
        have_next = page * limit + 1;
        users = self.get_blocked_users(limit.into(), ((page - 1) * limit).into());
    }
    else {
        users = self.get_blocked_users(limit.into(), 0);
        have_next = limit + 1;
    }
    if self.get_blocked_users(1, have_next.into()).len() > 0 {
        next_page_number = page + 1;
    }

    return Json(UsersListJson {
        description: "Черный спсок".to_string(),
        users: users,
        next_page: next_page_number,
    });
}

pub fn get_friends(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
    use crate::schema::{
        users::dsl::users,
        friends::dsl::friends,
    };

    let _connection = establish_connection();
    let friend_ids = friends
        .filter(schema::friends::user_id.eq(self.id))
        .limit(limit)
        .offset(offset)
        .select(schema::friends::target_id)
        .load::<i32>(&_connection)
        .expect("E.");
    let friends = users
        .filter(schema::users::id.eq_any(friend_ids))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.");

    let mut json = Vec::new();
    for user in friends {
        json.push (CardUserJson {
            id:         user.id,
            first_name: user.first_name.clone(),
            last_name:  user.last_name.clone(),
            link:       user.link.clone(),
            image:      user.s_avatar.clone(),
        });
    }
    return json;
}

pub fn get_populate_smiles_json(&self) -> Json<Vec<UserPopulateSmileJson>> {
    use crate::schema::user_populate_smiles::dsl::user_populate_smiles;

    let _connection = establish_connection();
    let all_populate_smiles = user_populate_smiles
        .filter(schema::user_populate_smiles::user_id.eq(self.id))
        .order(schema::user_populate_smiles::count.desc())
        .limit(20)
        .select((schema::user_populate_smiles::smile_id, schema::user_populate_smiles::image))
        .load::<(i32, String)>(&_connection)
        .expect("E");
    let mut smiles_json = Vec::new();
    for smile in all_populate_smiles.iter() {
        smiles_json.push(UserPopulateSmileJson {
            smile_id: smile.0,
            image:    smile.1.clone(),
        });
    }
    return Json(smiles_json);
}
