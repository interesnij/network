use serde::Serialize;


////////
#[derive(Serialize)]
pub struct UserDetailJson {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub is_man:        bool, 
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub image:         Option<String>,
    pub birthday:      String,
    pub last_activity: String,
    pub identified:    i16,
    pub slug:          String,
}

////////
#[derive(Serialize)]
pub struct LocationsJson {
    pub locations: Vec<LocationJson>,
    pub next_page: i32,
}
#[derive(Serialize, Queryable)]
pub struct LocationJson {
    pub city_ru:    Option<String>,
    //pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    //pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    //pub country_en: Option<String>,
}

////////
#[derive(Serialize)]
pub struct IpsJson {
    pub ips:       Vec<IpJson>,
    pub next_page: i32,
}
#[derive(Serialize, Queryable)]
pub struct IpJson {
    pub ip: String,
}

////////
#[derive(Serialize)]
pub struct ListsUserCommunitiesJson {
    pub lists:     Vec<ListUserCommunitiesJson>,
    pub next_page: i32,
}
#[derive(Serialize, Queryable)]
pub struct ListUserCommunitiesJson {
    pub id:    i32,
    pub name:  String,
    pub types: String,
}

////////
#[derive(Serialize)]
pub struct LoveStatusJson {
    pub male_status:   String,
    pub female_status: String,
}


//////// FeaturedUserCommunities, NewsUserCommunities, NotifyUserCommunities
////////
#[derive(Serialize)]
pub struct UniversalUserCommunityKeysJson {
    pub keys:      Vec<UniversalUserCommunityKeyJson>,
    pub next_page: i32,
}
#[derive(Serialize, Queryable)]
pub struct UniversalUserCommunityKeyJson {
    pub id:           i32,
    pub list_id:      Option<i32>,
    pub mute:         bool,
    pub sleep:        String,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}

////////
#[derive(Serialize)]
pub struct UserPrivateJson {
    pub see_all:    i16,
    pub see_info:   i16,
    pub see_friend: i16,
}

////////
#[derive(Serialize)]
pub struct UserProfileNotificationJson {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
    pub message:              bool,
}

////////
#[derive(Serialize)]
pub struct UserPopulateStickerJson {
    pub sticker_id: i32,
    pub image:      String,
}

////////
#[derive(Serialize)]
pub struct UserPopulateSmileJson {
    pub smile_id: i32,
    pub image:    String,
}

////////
#[derive(Serialize)]
pub struct UserVisiblePermJson {
    pub see_info:            String,
    pub see_friend:          String,
    pub see_all:             String,
}

////////
#[derive(Serialize)]
pub struct PhoneCodeJson {
    pub phone: String,
    pub code:  i32,
}
