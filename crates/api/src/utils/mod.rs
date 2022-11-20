#[derive(Debug, Serialize)]
pub struct ReactionsJson {
    pub reactions: Vec<ReactionJson>,
}
#[derive(Debug, Serialize, Queryable)]
pub struct ReactionJson {
    pub id:    i32,
    pub image: String,
    pub name:  String,
}
