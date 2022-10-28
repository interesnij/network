use serde::Serialize;


////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct UsersJson {
    pub users:     Vec<CardUserJson>,
    pub next_page: i32,
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

////////
#[derive(Serialize, Queryable)]
pub struct CommunityCategoryJson {
    pub id:     i32,
    pub name:   String,
    pub avatar: Option<String>,
}

////////
#[derive(Serialize, Queryable)]
pub struct CommunitySubcategoryJson {
    pub id:     i32,
    pub name:   String,
    pub avatar: Option<String>,
}
////////
#[derive(Serialize)]
pub struct CommunityDetailJson {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub types:       i16,
    pub perm:        String,
    pub link:        String, // community.get_link()
    pub image:       String,
    pub cover:       Option<String>,
    pub user_id:     i32,
}
#[derive(Serialize)]
pub struct NewCommunityJson {
    pub name:  String,
    pub types: i16,
    pub link:  String,
}

////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct CommunityInvitesJson {
    pub users:     Vec<CardCommunityInviteJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
// это объект пользователя
pub struct CardCommunityInviteJson {
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      String,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityInfoJson {
    pub posts:     i16,
    pub members:   i16,
    pub photos:    i16,
    pub goods:     i16,
    pub tracks:    i16,
    pub videos:    i16,
    pub docs:      i16,
    pub articles:  i16,
    pub survey:    i16,
    pub planners:  i16,
}

////////
#[derive(Serialize)]
pub struct CommunityPrivateJson {
    pub see_member:   i16,
    pub see_info:     i16,
    pub see_settings: i16,
    pub see_log:      i16,
    pub see_stat:     i16,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityNotificationJson {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}

////////
#[derive(Serialize)]
pub struct CommunityVisiblePermJson {
    pub see_info:   i16,
    pub see_member: i16,
}
