use bcrypt::{hash, verify, DEFAULT_COST};
use rbatis::crud::CRUD;
use rbatis::{core::Result, rbatis::Rbatis, snowflake::Snowflake};

use crate::models::{User, UserSignup};

pub async fn create(user_data: &UserSignup, rb: &Rbatis, sflake: &Snowflake) -> Option<User> {
    //log::info!("Create user");
    let users_count = rb.fetch_list_by_column("id",&["1".to_string()]).await.unwrap().len() + 1;
    let link = "/id".to_string() + &users_count.to_string() + &"/".to_string();

    let user = User {
        id:            sflake.generate().unsigned_abs(),
        first_name:    user_data.first_name.clone(),
        last_name:     user_data.last_name.clone(),
        phone:         user_data.phone.clone(),
        types:         1,
        is_man:        user_data.is_man.clone(),
        level:         100,
        password:      hash(user_data.password.clone(), 8).unwrap(),
        link:          link,
        s_avatar:      None,
        last_activity: *rbatis::DateTimeNative::now(),
    };

    match rb.save(&user, &[]).await {
        Ok(res) =>
        {
            log::info!("Successfully create user {}", user.last_name.clone());
            Some(user)
        },
        Err(err) => {
            log::error!("Failed create user: {}", err.to_string());
            None
        }
    }
}

pub async fn update() -> Result<User> {
    todo!()
}

pub async fn find_by_id(id: u64) -> Result<User> {
    let res: Result<User> = rb.fetch_by_column("id", id).await;
    match res {
        Ok(user) => Some(user),
        Err(err) => {
            log::error!("Failed find by id: {}", err.to_string());
            None
        }
    }
}

pub async fn find_by_phone(phone: &String, rb: &Rbatis) -> Option<User> {
    let res: Result<User> = rb.fetch_by_column("phone", phone).await;
    match res {
        Ok(user) => Some(user),
        Err(err) => {
            log::error!("Failed find by phone: {}", err.to_string());
            None
        }
    }
}
