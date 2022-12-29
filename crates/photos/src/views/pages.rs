use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
    web::block,
};
use crate::utils::{
    get_community,
    get_user,
    get_photo_list,
    get_photo,
    get_photo_comment,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_owner_data, 
    ErrorParams,
    SearchTargetListData, SearchRegListData,
    CardPhotoListJson, CardPhotoJson,
};
use crate::models::{
    PhotoList, Photo, PhotoComment,
    User, Community, 
    SearchAllPhotos, SearchAllComments,
};
use serde::{Deserialize, Serialize};
use crate::errors::Error;
use crate::AppState;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/edit_user_list", web::get().to(edit_user_list_page));
    config.route("/edit_community_list", web::get().to(edit_community_list_page));
    config.route("/edit_photo", web::get().to(edit_photo_page));

    config.route("/load_list", web::get().to(load_list_page));
    config.route("/load_photo", web::get().to(load_photo_page));
    config.route("/load_comments", web::get().to(load_comments_page));
    config.route("/photo_reactions", web::get().to(photo_reactions_page));
    config.route("/comment_reactions", web::get().to(comment_reactions_page));

    config.route("/search_lists", web::get().to(search_lists_page));
    config.route("/search_user_lists", web::get().to(search_user_lists_page));
    config.route("/search_community_lists", web::get().to(search_community_lists_page));

    config.route("/search_photos", web::get().to(search_photos_page));
    config.route("/search_user_photos", web::get().to(search_user_photos_page));
    config.route("/search_community_photos", web::get().to(search_community_photos_page));
    config.route("/search_list_photos", web::get().to(search_list_photos_page));

    config.route("/search_comments", web::get().to(search_comments_page));
    config.route("/search_user_comments", web::get().to(search_user_comments_page));
    config.route("/search_community_comments", web::get().to(search_community_comments_page));
    config.route("/search_list_comments", web::get().to(search_list_comments_page));
    config.route("/search_photo_comments", web::get().to(search_photo_comments_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I'm photos server.
            </p>
        </div>"
    )
}

#[derive(Debug, Deserialize)]
pub struct LoadListParams {
    pub token:   Option<String>, // токен приложения
    pub list_id: Option<i32>,    // какой список интересует
    pub limit:   Option<i64>,    // кол-во постов
    pub offset:  Option<i64>,    // число смещения
} 

pub async fn load_list_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> impl Responder {
    let params_some = web::Query::<LoadListParams>::from_query(req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let list: PhotoList;
            let list_res = get_photo_list(params.list_id.unwrap());
            if list_res.is_ok() {
                list = list_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if user_id > 0 {
                // если есть id пользователя запрашивающего

                if list.community_id.is_some() {
                    // если список сообщества
                    let c_id = list.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = list.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            // если пользователь не может просматривать информацию сообщества
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let lists = PhotoList::get_community_photo_lists(c_id, Some(10), Some(0));
                            let body = serde_json::to_string(&PhotoList::get_community_photo_list_json (
                                community,
                                user_id,
                                list,
                                lists,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    // если список пользователя
                    let owner = list.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        // если пользователь не может просматривать информацию владельца списка
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let lists = PhotoList::get_user_photo_lists(list.user_id, Some(10), Some(0));
                        let body = serde_json::to_string(&PhotoList::get_user_photo_list_json (
                            owner,
                            user_id,
                            list,
                            lists,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                // если пользователь анонимный, то есть параметра user_id нет
                if list.community_id.is_some() {
                    let c_id = list.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = list.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let lists = PhotoList::get_community_photo_lists(c_id, Some(10), Some(0));
                            let body = serde_json::to_string(&PhotoList::get_anon_community_photo_list_json (
                                community,
                                list,
                                lists,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = list.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let lists = PhotoList::get_user_photo_lists(list.user_id, Some(10), Some(0));
                        let body = serde_json::to_string(&PhotoList::get_anon_user_photo_list_json (
                            owner,
                            list,
                            lists,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct EditUserListParams {
    pub token:   Option<String>,
    pub list_id: Option<i32>,
}
pub async fn edit_user_list_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<EditUserListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PhotoList;
            let list_res = get_photo_list(params.list_id.unwrap());
            if list_res.is_ok() {
                list = list_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if list.user_id == user_id && list.community_id.is_none() {
                let body = serde_json::to_string(&list.get_edit_list_json().expect("E."))
                    .unwrap();
                    HttpResponse::Ok().body(body)
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied.".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct EditCommunityListParams {
    pub token:   Option<String>,
    pub list_id: Option<i32>,
}
pub async fn edit_community_list_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<EditCommunityListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 38).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 && community_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PhotoList;
            let community: Community;
            let list_res = get_photo_list(params.list_id.unwrap());
            if list_res.is_ok() {
                list = list_res.expect("E");
                community = list.get_community().expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if community.is_user_create_list(user_id) {
                let body = serde_json::to_string(&list.get_edit_list_json().expect("E."))
                    .unwrap();
                    HttpResponse::Ok().body(body)
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Permission Denied.".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadItemParams {
    pub token:   Option<String>,
    pub item_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

pub async fn load_photo_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        // если параметры строки запроса правильные...
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let item: Photo;
            let item_res = get_photo(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if user_id > 0 {
                // если есть id пользователя запрашивающего

                if item.community_id.is_some() {
                    // если список сообщества
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            // если пользователь не может просматривать информацию сообщества
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.get_detail_photo_json (
                                user_id,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    // если список пользователя
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        // если пользователь не может просматривать информацию владельца списка
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.get_detail_photo_json (
                            user_id,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                // если пользователь анонимный, то есть параметра user_id нет
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.get_detail_photo_json (
                                user_id,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.get_detail_photo_json (
                            user_id,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

pub async fn load_comments_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        // если параметры строки запроса правильные...
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let item: Photo;
            let list: PhotoList;
            let reactions_list: Vec<i32>;
            let item_res = get_photo(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            let list_res = get_photo_list(item.photo_list_id);
            if list_res.is_ok() {
                list = list_res.expect("E");
                reactions_list = list.get_reactions_list();
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if user_id > 0 {
                // если есть id пользователя запрашивающего

                if list.community_id.is_some() {
                    // если список сообщества
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = list.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            // если пользователь не может просматривать информацию сообщества
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.get_comments (
                                user_id,
                                reactions_list.clone(),
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    // если список пользователя
                    let owner = list.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        // если пользователь не может просматривать информацию владельца списка
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.get_comments (
                            user_id,
                            reactions_list.clone(),
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                // если пользователь анонимный, то есть параметра user_id нет
                if list.community_id.is_some() {
                    let c_id = list.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        // если токен сообщества, но список не этого сообщества
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = list.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.get_comments (
                                user_id,
                                reactions_list.clone(),
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = list.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.get_comments (
                            user_id,
                            reactions_list.clone(),
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}


#[derive(Debug, Deserialize)]
pub struct ItemParams {
    pub token:   Option<String>,
    pub item_id: Option<i32>,
}
#[derive(Debug, Serialize)]
pub struct DescriptionResp {
    pub description: Option<String>,
}

pub async fn edit_photo_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<ItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 38).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 && community_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'item_id' is required!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else {
            let item: Photo;
            let item_res = get_photo(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }
            let list = item.get_list().expect("E.");
            if (community_id > 0 && list.community_id.is_some() && list.community_id.unwrap() == community_id)
                ||
                list.user_id == user_id
                ||
                (list.is_user_create_el(user_id) && item.user_id == user_id)
             {
                 let body = serde_json::to_string (
                    &DescriptionResp {
                        description: item.description.clone(),
                    }
                ).unwrap();
                HttpResponse::Ok().body(body)
             }
             else {
                 let body = serde_json::to_string(&ErrorParams {
                     error: "Permission Denied.".to_string(),
                 }).unwrap();
                 HttpResponse::Ok().body(body)
             }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemReactionsParams {
    pub token:       Option<String>,
    pub item_id:     Option<i32>,
    pub reaction_id: Option<i32>,
    pub limit:       Option<i64>,
    pub offset:      Option<i64>,
}
pub async fn photo_reactions_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<ItemReactionsParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 && community_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.reaction_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'reaction_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: Photo;
            let item_res = get_photo(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "item not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if item.community_id.is_some() {
                let community = item.get_community().expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let body = serde_json::to_string (
                        &item.get_users_of_reaction (
                            user_id,
                            params.reaction_id.unwrap(),
                            params.limit,
                            params.offset,
                        )
                    )
                    .unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let owner = item.get_creator().expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let body = serde_json::to_string (
                        &item.get_users_of_reaction (
                            user_id,
                            params.reaction_id.unwrap(),
                            params.limit,
                            params.offset,
                        )
                    )
                        .unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

pub async fn comment_reactions_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let params_some = web::Query::<ItemReactionsParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 && community_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if user_id < 1 {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.reaction_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'reaction_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let comment: PhotoComment;
            let item: Photo; 
            let comment_res = get_photo_comment(params.item_id.unwrap());
            if comment_res.is_ok() {
                comment = comment_res.expect("E");
                item = comment.get_item().expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "comment not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if comment.community_id.is_some() {
                let community = item.get_community().expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let body = serde_json::to_string (
                        &comment.get_users_of_reaction (
                            user_id,
                            params.reaction_id.unwrap(),
                            params.limit,
                            params.offset,
                        )
                    )
                    .unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let owner = item.get_creator().expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let body = serde_json::to_string (
                        &comment.get_users_of_reaction (
                            user_id,
                            params.reaction_id.unwrap(),
                            params.limit,
                            params.offset,
                        )
                    )
                    .unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}


pub async fn search_lists_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<Vec<CardPhotoListJson>>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let _res = block(move || PhotoList::search_photo_lists(&q, params.limit, params.offset)).await?;
            Ok(Json(_res))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_lists_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<Vec<CardPhotoListJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if user_id > 0 {
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_photo_lists(&q, params.limit, params.offset)).await?;
                    Ok(Json(body)) 
                }
            }
            else {
                let _tuple = get_anon_user_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_photo_lists(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_community_lists_page (
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<Json<Vec<CardPhotoListJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: Community;
            let _id: i32;
            if community_id > 0 {
                _id = community_id;
            } else {
                _id = params.target_id.unwrap();
            }
            let owner_res = get_community(_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            if user_id > 0 {
                let _tuple = get_community_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_photo_lists(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
            else {
                let _tuple = get_anon_community_permission(&owner);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    Err(Error::BadRequest(body))
                }
                else {
                    let body = block(move || owner.search_photo_lists(&q, params.limit, params.offset)).await?;
                    Ok(Json(body))
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_photos_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<SearchAllPhotos>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is required!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let _res = block(move || Photo::search_photos(&q, user_id, params.limit, params.offset)).await?;
            Ok(Json(_res))  
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_photos_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardPhotoJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.search_photos(&q, user_id, params.limit, params.offset)).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_community_photos_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<Vec<CardPhotoJson>>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: Community;
            let _id: i32;
            if community_id > 0 {
                _id = community_id;
            } else {
                _id = params.target_id.unwrap();
            }
            let owner_res = get_community(_id);
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let body = block(move || owner.search_photos(&q, user_id, params.limit, params.offset)).await?;
             Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}


pub async fn search_list_photos_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> impl Responder {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: PhotoList;
            let item_res = get_photo_list(params.target_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }

            else if user_id > 0 {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_items (
                                user_id, 
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_items (
                            user_id,
                            &q,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_items (
                                user_id,
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_items (
                            user_id,
                            &q,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}


pub async fn search_comments_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<SearchAllComments>, Error> {
    let params_some = web::Query::<SearchRegListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let _res = block(move || PhotoComment::search_comments (
                &q, 
                user_id, 
                params.limit, 
                params.offset
            )).await?;
            Ok(Json(_res))   
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_user_comments_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<SearchAllComments>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: User;
            let owner_res = get_user(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            let body = block(move || owner.search_comments(&q, user_id, params.limit, params.offset)).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_community_comments_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> Result<Json<SearchAllComments>, Error> {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
        else {
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }
            let owner: Community;
            let owner_res = get_community(params.target_id.unwrap());
            if owner_res.is_ok() {
                owner = owner_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "owner not found!".to_string(),
                }).unwrap();
                return Err(Error::BadRequest(body));
            }

            let body = block(move || owner.search_comments(&q, user_id, params.limit, params.offset)).await?;
            Ok(Json(body))
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
}

pub async fn search_list_comments_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> impl Responder {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: PhotoList;
            let item_res = get_photo_list(params.target_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }

            else if user_id > 0 {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else if !community.is_user_see_comment(user_id) || !item.is_user_see_comment(user_id) {
                            let body = serde_json::to_string(&ErrorParams {
                                error: "Permission Denied".to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_items (
                                user_id, 
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else if !owner.is_user_see_comment(user_id) || !item.is_user_see_comment(user_id) {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_items (
                            user_id,
                            &q,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else if !community.is_anon_user_see_comment() || !item.is_anon_user_see_comment() {
                            let body = serde_json::to_string(&ErrorParams {
                                error: "Permission Denied".to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_items (
                                user_id,
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else if !owner.is_anon_user_see_comment() || !item.is_anon_user_see_comment() {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_items (
                            user_id,
                            &q,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

pub async fn search_photo_comments_page (
    req: HttpRequest,
    state: web::Data<AppState>
) -> impl Responder {
    let params_some = web::Query::<SearchTargetListData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(&req, state, params.token.clone(), 8).await;
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.target_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'target_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.q.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Field 'q' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: Photo;
            let item_res = get_photo(params.target_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "photo list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }
            let list = item.get_list().expect("E");
            let q = params.q.clone().unwrap();
            if q.is_empty() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "Field 'q' is empty!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }

            else if user_id > 0 {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_community_permission(&community, user_id);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else if !community.is_user_see_comment(user_id) || !list.is_user_see_comment(user_id) {
                            let body = serde_json::to_string(&ErrorParams {
                                error: "Permission Denied".to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_comments (
                                user_id,
                                list.get_reactions_list(),
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_user_permission(&owner, user_id);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else if !owner.is_user_see_comment(user_id) || !list.is_user_see_comment(user_id) {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_comments (
                            user_id,
                            list.get_reactions_list(),
                            &q,
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                if item.community_id.is_some() {
                    let c_id = item.community_id.unwrap();
                    if community_id > 0 && c_id != community_id {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied.".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let community = item.get_community().expect("E.");
                        let _tuple = get_anon_community_permission(&community);
                        if _tuple.0 == false {
                            let body = serde_json::to_string(&ErrorParams {
                                error: _tuple.1.to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else if !community.is_anon_user_see_comment() || !list.is_anon_user_see_comment() {
                            let body = serde_json::to_string(&ErrorParams {
                                error: "Permission Denied".to_string(),
                            }).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                        else {
                            let body = serde_json::to_string(&item.search_comments (
                                user_id,
                                list.get_reactions_list(),
                                &q,
                                params.limit,
                                params.offset
                            )).unwrap();
                            HttpResponse::Ok().body(body)
                        }
                    }
                }
                else {
                    let owner = item.get_creator().expect("E.");
                    let _tuple = get_anon_user_permission(&owner);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else if !owner.is_anon_user_see_comment() || !list.is_anon_user_see_comment() {
                        let body = serde_json::to_string(&ErrorParams {
                            error: "Permission Denied".to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.search_comments (
                            user_id,
                            list.get_reactions_list(),
                            &q, 
                            params.limit,
                            params.offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}