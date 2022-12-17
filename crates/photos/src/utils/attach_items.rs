use serde::{Serialize, Deserialize};
use crate::utils::AttachPhoto;

//////////////// Выдача прикрепов /////////
// Список фото
#[derive(Serialize)]
pub struct AttachPhotoListResp {
    pub owner:     Option<AttachOwner>,     // данные владельца
    pub community: Option<AttachCommunity>, // данные сообщества
    pub data:      AttachList,              // данные самого списка
}

// Фото
#[derive(Serialize)]
pub struct AttachPhotoResp {
    pub owner:     Option<AttachOwner>,    // данные владельца
    pub community: Option<AttachCommunity>,// данные сообщества
    pub list:      AttachPermList,         // данные приватности для элемента
    pub data:      AttachPhoto,            // данные фото
}

// Комментарий к фото
#[derive(Serialize)]
pub struct AttachPhotoCommentResp {
    pub owner:     Option<AttachOwner>,    // данные владельца
    pub community: Option<AttachCommunity>,// данные сообщества
    pub list:      Option<AttachPermList>, // данные приватности для элемента
    pub data:      AttachPhotoComment,     // данные коммента
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AttachOwner {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub types:      i16,
    pub link:       String,
    pub s_avatar:   Option<String>,
    pub see_all:    i16,
}
#[derive(Deserialize, Serialize)]
pub struct AttachCommunity {
    pub id:       i32,
    pub name:     String,
    pub types:    i16,
    pub link:     String,
    pub s_avatar: Option<String>,
}
#[derive(Serialize)]
pub struct AttachPermList {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub list_types:   i16,
    pub types:        i16,
    pub see_el:       i16,
    pub copy_el:      i16,
}

#[derive(Serialize)]
pub struct AttachList {
    pub id:      i32,
    pub name:    String,
    pub types:   i16,
    pub image:   Option<String>,
    pub count:   i32,
    pub see_el:  i16,
    pub copy_el: i16,
}

#[derive(Serialize)]
pub struct AttachPhotoComment {
    pub id:          i32,
    pub content:     Option<String>,
    pub attachments: Option<AttachmentsJson>,
    pub types:       i16,
    pub created:     chrono::NaiveDateTime,
    pub repost:      i32,
    pub reactions:   i32,
}

////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct AttachListJson {
    pub attachments: Option<AttachmentsJson>,
}

#[derive(Serialize, Deserialize)]
pub struct AttachmentsJson {
    pub items:    Option<ItemsAttach>,
    pub lists:    Option<ListsAttach>,
    pub comments: Option<CommentsAttach>,
    //pub planners: Option<PlannersAttach>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsAttach {
    pub photos:      Option<Vec<PhotoAttach>>,
    pub videos:      Option<Vec<VideoAttach>>,
    pub docs:        Option<Vec<DocAttach>>,
    pub audios:      Option<Vec<AudioAttach>>,
    pub goods:       Option<Vec<GoodAttach>>,
    pub users:       Option<Vec<UserAttach>>,
    pub communities: Option<Vec<CommunityAttach>>,
    pub sites:       Option<Vec<SiteAttach>>,
    pub surveys:     Option<Vec<SurveyAttach>>,
    pub wikis:       Option<Vec<WikiAttach>>,
    pub articles:    Option<Vec<ArticleAttach>>,
}

#[derive(Serialize, Deserialize)]
pub struct PhotoAttach {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub server_id:    i16,
    pub file:         String,
}
#[derive(Serialize, Deserialize)]
pub struct VideoAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        String,
    pub file:         String,
    pub view:         i32,
}
#[derive(Serialize, Deserialize)]
pub struct DocAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub file:         String,
}
#[derive(Serialize, Deserialize)]
pub struct AudioAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub artist_id:    Option<i32>,
    pub file:         String,
    pub image:        Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct GoodAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub price:        Option<i32>,
    pub image:        Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct UserAttach {
    pub user_id:    i32,
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub s_avatar:   Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct CommunityAttach {
    pub community_id: i32,
    pub name:         String,
    pub link:         String,
    pub s_avatar:     Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct SiteAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub item_id:      i32,
}
#[derive(Serialize, Deserialize)]
pub struct SurveyAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub is_anonymous: bool,
    pub is_multiple:  bool,
    pub is_no_edited: bool,
    pub time_end:     Option<String>,
    pub vote:         i32,
}
#[derive(Serialize, Deserialize)]
pub struct WikiAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ArticleAttach {
    pub title:        String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
}

//////////////////////////////
#[derive(Serialize, Deserialize)]
pub struct ListsAttach {
    pub post_lists:     Option<Vec<PostListAttach>>,
    pub music_lists:    Option<Vec<MusicListAttach>>,
    pub doc_lists:      Option<Vec<DocListAttach>>,
    pub survey_lists:   Option<Vec<SurveyListAttach>>,
    pub photo_lists:    Option<Vec<PhotoListAttach>>,
    pub video_lists:    Option<Vec<VideoListAttach>>,
    pub good_lists:     Option<Vec<GoodListAttach>>,
    pub forum_lists:    Option<Vec<ForumListAttach>>,
    pub wiki_lists:     Option<Vec<WikiListAttach>>,
    pub articles_lists: Option<Vec<ArticleListAttach>>,
    pub folder_lists:   Option<Vec<ForumListAttach>>,
    pub stickers_lists: Option<Vec<StickersListAttach>>,
}

#[derive(Serialize, Deserialize)]
pub struct PostListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct MusicListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct DocListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct SurveyListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct PhotoListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct VideoListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct GoodListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct ForumListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct WikiListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct ArticleListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct FolderListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}
#[derive(Serialize, Deserialize)]
pub struct StickersListAttach {
    pub name:         String,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub list_id:      i32,
    pub item_id:      i32,
    pub image:        Option<String>,
    pub count:        i32,
}

//////////////

#[derive(Serialize, Deserialize)]
pub struct CommentsAttach {
    pub post_comments:    Option<Vec<CommentAttach>>,
    pub photo_comments:   Option<Vec<CommentAttach>>,
    pub video_comments:   Option<Vec<CommentAttach>>,
    pub good_comments:    Option<Vec<CommentAttach>>,
    pub forum_comments:   Option<Vec<CommentAttach>>,
    pub wiki_comments:    Option<Vec<CommentAttach>>,
}
#[derive(Serialize, Deserialize)]
pub struct CommentAttach {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub content:      Option<String>,
    pub attach:       Option<String>,
    pub comment_id:   i32,
    pub item_id:      i32,
    pub item_type:    String,  // например, коммент к записи.
    pub image:        Option<String>,
    pub count:        i32,
    pub created:      chrono::NaiveDateTime,
}

//#[derive(Serialize, Deserialize)]
//pub struct PlannersAttach {
//    pub workspaces:       Option<Vec<WorkspaceAttach>>,
//    pub boards:           Option<Vec<BoardAttach>>,
//    pub columns:          Option<Vec<ColumnAttach>>,
//    pub planner_cards:    Option<Vec<PlannerCardAttach>>,
//    pub planner_comments: Option<Vec<PlannerCommentAttach>>,
//}
