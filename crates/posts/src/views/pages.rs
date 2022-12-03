use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use crate::utils::{
    get_community,
    get_post_list,
    get_post,
    get_post_comment,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    get_owner_data,
    ErrorParams,
    AttachPostListResp,
    AttachPostResp,
    AttachPostCommentResp,
};
use crate::models::{
    PostList,
    Post,
    Community,
    PostComment,
};
use serde::Deserialize;
use crate::errors::Error;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/edit_user_list/", web::get().to(edit_user_list_page));
    config.route("/edit_community_list/", web::get().to(edit_community_list_page));
    config.route("/edit_post/", web::get().to(edit_post_page));

    config.route("/load_list/", web::get().to(load_list_page));
    config.route("/load_post/", web::get().to(load_post_page));
    config.route("/load_comments/", web::get().to(load_comments_page));
    config.route("/post_reactions/", web::get().to(post_reactions_page));
    config.route("/comment_reactions/", web::get().to(comment_reactions_page));

    //config.route("/search_lists/", web::get().to(search_lists_page));
    //config.route("/search_posts/", web::get().to(search_posts_page));
    //config.route("/search_comments/", web::get().to(search_comments_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I'm posts server.
            </p>
        </div>"
    )
}

#[derive(Debug, Deserialize)]
pub struct LoadListParams {
    pub token:   Option<String>, // токен приложения
    pub user_id: Option<i32>,    // кто запрашивает
    pub list_id: Option<i32>,    // какой список интересует
    pub limit:   Option<i64>,    // кол-во постов
    pub offset:  Option<i64>,    // число смещения
}

pub async fn load_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadListParams>::from_query(req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let list: PostList;
            let list_res = get_post_list(params.list_id.unwrap());
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
                            let lists = PostList::get_community_post_lists(c_id, Some(10), Some(0));
                            let body = serde_json::to_string(&PostList::get_community_post_list_json (
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
                        let lists = PostList::get_user_post_lists(list.user_id, Some(10), Some(0));
                        let body = serde_json::to_string(&PostList::get_user_post_list_json (
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
                            let lists = PostList::get_community_post_lists(c_id, Some(10), Some(0));
                            let body = serde_json::to_string(&PostList::get_anon_community_post_list_json (
                                community,
                                list,
                                lists,
                                _limit,
                                _offset
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
                        let lists = PostList::get_user_post_lists(list.user_id, Some(10), Some(0));
                        let body = serde_json::to_string(&PostList::get_anon_user_post_list_json (
                            owner,
                            list,
                            lists,
                            _limit,
                            _offset
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
    pub user_id: Option<i32>,    // кто запрашивает
    pub list_id: Option<i32>,
}
pub async fn edit_user_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<EditUserListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, _community_id) = get_owner_data(params.token.clone(), params.user_id);
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
                error: "Parametr 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PostList;
            let list_res = get_post_list(params.list_id.unwrap());
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
    pub user_id: Option<i32>,
    pub list_id: Option<i32>,
}
pub async fn edit_community_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<EditCommunityListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if (user_id < 1 && community_id < 1) {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'list_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PostList;
            let community: Community;
            let list_res = get_post_list(params.list_id.unwrap());
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
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

pub async fn load_post_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        // если параметры строки запроса правильные...
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let item: Post;
            let item_res = get_post(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "post not found!".to_string(),
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
                            let body = serde_json::to_string(&item.get_detail_post_json (
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
                        let body = serde_json::to_string(&item.get_detail_post_json (
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
                            let body = serde_json::to_string(&item.get_detail_post_json (
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
                        let body = serde_json::to_string(&item.get_detail_post_json (
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

pub async fn load_comments_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        // если параметры строки запроса правильные...
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            // если какой то токен совпал...
            let item: Post;
            let list: PostList;
            let reactions_list: Vec<i32>;
            let item_res = get_post(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                // если список по id не найден...
                let body = serde_json::to_string(&ErrorParams {
                    error: "posts list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            let list_res = get_post_list(item.post_list_id);
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
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
}

pub async fn edit_post_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<ItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if (user_id < 1 && community_id < 1) {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: Post;
            let item_res = get_post(params.item_id.unwrap());
            if item_res.is_ok() {
                item = item_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "post not found!".to_string(),
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
                 let body = serde_json::to_string(&item.get_edit_data_json()).unwrap();
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
    pub user_id:     Option<i32>,
    pub item_id:     Option<i32>,
    pub reaction_id: Option<i32>,
    pub limit:       Option<i64>,
    pub offset:      Option<i64>,
}
pub async fn post_reactions_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<ItemReactionsParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась или запрос анонимный...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if (user_id < 1 && community_id < 1) {
            let body = serde_json::to_string(&ErrorParams {
                error: "Permission Denied!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.reaction_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'reaction_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let item: Post;
            let item_res = get_post(params.item_id.unwrap());
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

pub async fn comment_reactions_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<ItemReactionsParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let (err, user_id, community_id) = get_owner_data(params.token.clone(), params.user_id);
        if err.is_some() {
            // если проверка токена не удалась или запрос анонимный...
            let body = serde_json::to_string(&ErrorParams {
                error: err.unwrap(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }
        else if (user_id < 1 && community_id < 1) {
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
                error: "Parametr 'item_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.reaction_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "Parametr 'reaction_id' is required!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let comment: PostComment;
            let item: Post; 
            let comment_res = get_post_comment(params.item_id.unwrap());
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

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub token:   Option<String>,
    pub user_id: Option<i32>,
    pub q:       Option<String>,
    pub item_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}
