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
