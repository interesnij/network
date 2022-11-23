use serde::{Serialize, Deserialize};


#[derive(Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}


#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}

#[derive(Deserialize)]
pub struct RegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

#[derive(Deserialize)]
pub struct TargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub target_id: Option<i32>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchRegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub q:       Option<String>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

#[derive(Deserialize)]
pub struct SearchTargetListData {
    pub token:     Option<String>,
    pub user_id:   Option<i32>,
    pub q:         Option<String>,
    pub target_id: Option<i32>,
    pub limit:     Option<i64>,
    pub offset:    Option<i64>,
}
