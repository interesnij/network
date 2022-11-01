use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    Responder,
};
use crate::utils::{
    get_community,
    get_user,
    get_post_list,
    get_post,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    establish_connection,
    ErrorParams,
};
use crate::models::{User, PostList, Post, PostComment, Community};
use crate::diesel::RunQueryDsl;
use serde::{Serialize, Deserialize};


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/add_user_list/", web::get().to(add_user_list_page));
    config.route("/edit_user_list/", web::get().to(edit_user_list_page));
    config.route("/add_community_list/", web::get().to(add_community_list_page));
    config.route("/edit_community_list/", web::get().to(edit_community_list_page));
    config.route("/edit_post/", web::get().to(edit_post_page));

    config.route("/load_list/", web::get().to(load_list_page));
    config.route("/load_post/", web::get().to(load_post_page));
    config.route("/load_comments/", web::get().to(load_comments_page));
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body (
        "<div style='background: #ccc;position:absolute;top:0;left:0;right:0;bottom:0'>
            <p style='text-align: center'>
                hello, I'm posts server.
            </p>
        </div>")
}

#[derive(Debug, Deserialize)]
pub struct LoadListParams {
    pub list_id: i32,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

pub async fn load_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let _limit: i64;
        let _offset: i64;
        let list: PostList;
        let list_res = get_post_list(params.list_id);
        if list_res.is_ok() {
            list = list_res.expect("E");
        }
        else {
            let body = serde_json::to_string(&ErrorParams {
                error: "list not found!".to_string(),
            }).unwrap();
            return HttpResponse::Ok().body(body);
        }

        if params.limit.is_some() {
            _limit = params.limit.unwrap();
        }
        else {
            _limit = 20;
        }
        if params.offset.is_some() {
            _offset = params.offset.unwrap();
        }
        else {
            _offset = 0;
        }
        if params.user_id.is_some() {
            let user_id = params.user_id.unwrap();

            if list.community_id.is_some() {
                let community = list.get_community().expect("E.");
                let _tuple = get_community_permission(&community, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_community_post_lists(list.community_id.unwrap(), 10, 0);
                    let body = serde_json::to_string(&PostList::get_community_post_list_json (
                        community,
                        user_id,
                        list,
                        lists,
                        _limit,
                        _offset
                    )).unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let owner = list.get_creator().expect("E.");
                let _tuple = get_user_permission(&owner, user_id);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_user_post_lists(list.user_id, 10, 0);
                    let body = serde_json::to_string(&PostList::get_user_post_list_json (
                        owner,
                        user_id,
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
            if list.community_id.is_some() {
                let community = list.get_community().expect("E.");
                let _tuple = get_anon_community_permission(&community);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        error: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_community_post_lists(list.community_id.unwrap(), 10, 0);
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
                    let lists = PostList::get_user_post_lists(list.user_id, 10, 0);
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
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'list_id' not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct AddUserListParams {
    pub user_id: Option<i32>,
}
pub async fn add_user_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<AddUserListParams>::from_query(&req.query_string());
    if params_some.is_ok() { 
        let params = params_some.unwrap();
        if params.user_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'user_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if get_user(params.user_id.unwrap()).is_err() {
            let body = serde_json::to_string(&ErrorParams {
                error: "user not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let body = serde_json::to_string(&PostList::get_add_list_json().expect("E."))
                .unwrap();
                HttpResponse::Ok().body(body)
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'user_id' not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct EditUserListParams {
    pub user_id: Option<i32>,
    pub list_id: Option<i32>,
}
pub async fn edit_user_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<EditUserListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.user_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'user_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'list_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PostList;
            let user_id = params.user_id.unwrap();
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
            error: "parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct AddCommunityListParams {
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
}
pub async fn add_community_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<AddCommunityListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.user_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'user_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'community_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let user_id = params.user_id.unwrap();
            let user_ok = get_user(user_id);
            let community_ok = get_community(params.community_id.unwrap());

            if user_ok.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "user not found!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
            else if community_ok.is_err() {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
                }).unwrap();
                HttpResponse::Ok().body(body)
            }
            else {
                let community = community_ok.expect("E.");
                if !community.is_user_create_list(user_id) {
                    let body = serde_json::to_string(&ErrorParams {
                        error: "Permission Denied".to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let body = serde_json::to_string(&PostList::get_add_list_json().expect("E.")).unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct EditCommunityListParams {
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub list_id:      Option<i32>,
}
pub async fn edit_community_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<EditCommunityListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.user_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'user_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.list_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'list_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.community_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'community_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let list: PostList;
            let community: Community;
            let user_id = params.user_id.unwrap();
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

            let community_res = get_community(params.community_id.unwrap());
            if community_res.is_ok() {
                community = community_res.expect("E");
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "community not found!".to_string(),
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
            error: "parametrs not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadItemParams {
    pub item_id: Option<i32>,
    pub user_id: Option<i32>,
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
}

pub async fn load_post_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'item_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {

            let _limit: i64;
            let _offset: i64;

            if params.limit.is_some() {
                _limit = params.limit.unwrap();
            }
            else {
                _limit = 20;
            }
            if params.offset.is_some() {
                _offset = params.offset.unwrap();
            }
            else {
                _offset = 0;
            }

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
            if params.user_id.is_some() {
                let user_id = params.user_id.unwrap();

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
                        let body = serde_json::to_string(&item.get_detail_post_json (
                            Some(user_id),
                            _limit,
                            _offset
                        )).unwrap();
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
                        let body = serde_json::to_string(&item.get_detail_post_json (
                            Some(user_id),
                            _limit,
                            _offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                if item.community_id.is_some() {
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
                            None,
                            _limit,
                            _offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
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
                            None,
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
            error: "parametr 'item_id' not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

pub async fn load_comments_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'item_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {

            let _limit: i64;
            let _offset: i64;

            if params.limit.is_some() {
                _limit = params.limit.unwrap();
            }
            else {
                _limit = 20;
            }
            if params.offset.is_some() {
                _offset = params.offset.unwrap();
            }
            else {
                _offset = 0;
            }

            let item: Post;
            let mut reactions_list: Vec<i32> = Vec::new();
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
            let list_res = get_post_list(item.post_list_id);
            if list_res.is_ok() {
                let list = list_res.expect("E");
                reactions_list = list.get_reactions_list();
            }
            else {
                let body = serde_json::to_string(&ErrorParams {
                    error: "list not found!".to_string(),
                }).unwrap();
                return HttpResponse::Ok().body(body);
            }

            if params.user_id.is_some() {
                let user_id = params.user_id.unwrap();

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
                        let body = serde_json::to_string(&item.get_comments (
                            Some(user_id),
                            reactions_list.clone(),
                            _limit,
                            _offset
                        )).unwrap();
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
                        let body = serde_json::to_string(&item.get_comments (
                            Some(user_id),
                            reactions_list.clone(),
                            _limit,
                            _offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                }
            }
            else {
                if item.community_id.is_some() {
                    let community = item.get_community().expect("E.");
                    let _tuple = get_anon_community_permission(&community);
                    if _tuple.0 == false {
                        let body = serde_json::to_string(&ErrorParams {
                            error: _tuple.1.to_string(),
                        }).unwrap();
                        HttpResponse::Ok().body(body)
                    }
                    else {
                        let body = serde_json::to_string(&item.get_comments (
                            None,
                            reactions_list.clone(),
                            _limit,
                            _offset
                        )).unwrap();
                        HttpResponse::Ok().body(body)
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
                        let body = serde_json::to_string(&item.get_comments (
                            None,
                            reactions_list.clone(),
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
            error: "parametr 'item_id' not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[derive(Debug, Deserialize)]
pub struct EditItemParams {
    pub item_id: Option<i32>,
    pub user_id: Option<i32>,
}

pub async fn edit_post_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<EditItemParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();

        if params.item_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'item_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else if params.user_id.is_none() {
            let body = serde_json::to_string(&ErrorParams {
                error: "parametr 'user_id' not found!".to_string(),
            }).unwrap();
            HttpResponse::Ok().body(body)
        }
        else {
            let user_id = params.user_id.unwrap();
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
                    let body = serde_json::to_string(&item.get_edit_data_json()).unwrap();
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
                    let body = serde_json::to_string(&item.get_edit_data_json())
                        .unwrap();
                    HttpResponse::Ok().body(body)
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'item_id' not found!".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}
