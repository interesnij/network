use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ReactionsJson {
    pub reactions: Vec<ReactionJson>,
}
#[derive(Deserialize, Serialize, Queryable)]
pub struct ReactionJson {
    pub id:    i32,
    pub image: String,
    pub name:  String,
}
#[derive(Deserialize, Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}
#[derive(Deserialize, Serialize, Queryable)]
// это объект сообщества
pub struct CardCommunityJson {
    pub id:    i32,
    pub name:  String,
    pub link:  String,
    pub image: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorParams {
    pub error: String,
}

#[derive(Deserialize, Serialize)]
pub struct InfoParams {
    pub info: String,
}

#[derive(Deserialize, Deserialize)]
pub struct RegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}
