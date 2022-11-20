use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionsJson {
    pub reactions: Vec<ReactionJson>,
}
#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct ReactionJson {
    pub id:    i32,
    pub image: String,
    pub name:  String,
}
#[derive(Debug, Deserialize, Serialize)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
// это объект сообщества
pub struct CardCommunityJson {
    pub id:    i32,
    pub name:  String,
    pub link:  String,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
// это объект списка записей
pub struct CardPostListJson {
    pub name:        String,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub image:       Option<String>,
    pub types:       String,
    pub count:       i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorParams {
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoParams {
    pub info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegListData {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}
