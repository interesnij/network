use serde::Serialize;

#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct UsersListJson {
    pub description: String, // описание, что за список, на всякий случай.
    pub users:       Vec<CardUserJson>,
    pub next_page:   i32,
}
#[derive(Serialize, Queryable)]
// это объект пользователя
pub struct CardUserJson {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      Option<String>,
}
