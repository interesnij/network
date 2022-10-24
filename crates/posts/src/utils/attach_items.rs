use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Attachments {
    pub items: Vec<Attachment>,
}

#[derive(Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub item: (),
}

#[derive(Serialize, Deserialize)]
pub struct MusicList {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub list_types:   i16,
    pub types:        i16,
    pub image:        Option<String>,
    pub count:        i32,
}
