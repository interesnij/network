use serde::{Serialize, Deserialize};
use diesel::{
    //RunQueryDsl,
    //ExpressionMethods,
    //QueryDsl,
    PgConnection,
    Connection,
};

// url серверов, куда шлем запросы
pub static POSTS_URL: &str = "http:194.58.90.123:9003";
pub static USERS_URL: &str = "http:194.58.90.123:9001";
pub static COMMUNITIES_URL: &str = "http:194.58.90.123:9002";

// список url сервисов, на которых присутствуют копии пользователей.
// нужно, к примеру, для синхронизации данных пользователей.
pub USERS_SERVICES: Vec<&str> = [
    "194.58.90.123:9002",
    "194.58.90.123:9003",
].to_vec();

// список url сервисов, на которых присутствуют копии сообществ.
pub COMMUNITIES_SERVICES: Vec<&str> = [
    "194.58.90.123:9003",
].to_vec();

// список url сервисов, на которых присутствуют копии элементов для прикрепов и папок.
pub ATTACH_SERVICES: Vec<&str> = [
    "194.58.90.123:9003".to_string(),
].to_vec();

pub fn get_error_response(_error: reqwest::Error) -> String {
    //if _error.is_status() {
    //    println!("Нет соединения");
    //    "Нет соединения".to_string()
    //}
    if _error.is_timeout() {
        println!("Время запроса вышло");
        "Время запроса вышло".to_string()
    }
    else if _error.is_connect() {
        println!("Нет подключения к серверу");
        "Нет подключения к серверу".to_string()
    }
    else if _error.is_body() {
        println!("Получен неожиданный ответ");
        "Получен неожиданный ответ".to_string()
    }
    else {
        println!("Неизвестная ошибка");
        "Неизвестная ошибка".to_string()
    }
}
pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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
