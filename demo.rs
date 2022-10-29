view:
use actix_web::{
  web::{block, Data, Json},
  Result,
}

let _res = block(move || CookieStat::get_stat_list(user_id, page, 20)).await?;
  let _dict = match _res {
    Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
    Err(_error) => {object_list = Vec::new(); next_page_number = 0},
  };
==========================
models:
use crate::errors::Error;

pub fn get_stat_list(user_id: i32, page: i32, limit: i32) -> Result<(Vec<CookieStat>, i32), Error> {
      let mut next_page_number = 0;
      let have_next: i32;
      let object_list: Vec<CookieStat>;

      if page > 1 {
          let step = (page - 1) * 20;
          have_next = page * limit + 1;
          object_list = CookieStat::get_stat_items(user_id, limit.into(), step.into())?;
      }
      else {
          have_next = limit + 1;
          object_list = CookieStat::get_stat_items(user_id, limit.into(), 0)?;
      }
      if CookieStat::get_stat_items(user_id, 1, have_next.into())?.len() > 0 {
          next_page_number = page + 1;
      }
      let _tuple = (object_list, next_page_number);
      Ok(_tuple)
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
