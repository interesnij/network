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
    get_post_comment,
    get_user_permission,
    get_anon_user_permission,
    get_community_permission,
    get_anon_community_permission,
    establish_connection,
};
use crate::models::{User, PostList, Post, PostComment, Community};
use crate::diesel::RunQueryDsl;
use serde::{Serialize, Deserialize};


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    //config.route("/add_user_list/", web::get().to(add_user_list_page));
    //config.route("/edit_user_list/{id}/", web::get().to(edit_user_list_page));
    //config.route("/add_community_list/{id}", web::get().to(add_community_list_page));
    //config.route("/edit_community_list/{id}/", web::get().to(edit_community_list_page));
    //config.route("/edit_post/{id}/", web::get().to(edit_post_page));

    config.route("/load_list/", web::get().to(load_list_page));
    //config.route("/load_post/{id}/", web::get().to(load_post_page));
    //config.route("/load_comments/{id}/", web::get().to(load_comments_page));
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
#[derive(Debug, Serialize)]
pub struct ErrorParams {
    pub info: String,
}

pub async fn load_list_page(req: HttpRequest) -> impl Responder {
    let params_some = web::Query::<LoadListParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let _limit: i64;
        let _offset: i64;
        let list = get_post_list(params.list_id).expect("E.");

        let params = params_some.unwrap();
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
            let _request_user = get_user(user_id).expect("E.");

            if list.community_id.is_some() {
                let community = list.get_community().expect("E.");
                let _tuple = get_community_permission(&community, &_request_user);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        info: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_community_post_lists(list.community_id.unwrap(), 10, 0);
                    let body = serde_json::to_string(&PostList::get_json_community_post_list(
                        community,
                        user_id,
                        list,
                        lists,
                        _limit,
                        _offset
                    ));
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let owner = list.get_creator().expect("E.");
                let _tuple = get_user_permission(&owner, &_request_user);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        info: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_user_post_lists(list.user_id, 10, 0);
                    let body = serde_json::to_string(&PostList::get_json_user_post_list (
                        owner,
                        user_id,
                        list,
                        lists,
                        _limit,
                        _offset
                    ));
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
                        info: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_community_post_lists(list.community_id.unwrap(), 10, 0);
                    let body = serde_json::to_string(&PostList::get_json_anon_community_post_list(
                        community,
                        list,
                        lists,
                        _limit,
                        _offset
                    ));
                    HttpResponse::Ok().body(body)
                }
            }
            else {
                let owner = list.get_creator().expect("E.");
                let _tuple = get_user_permission(&owner, &_request_user);
                if _tuple.0 == false {
                    let body = serde_json::to_string(&ErrorParams {
                        info: _tuple.1.to_string(),
                    }).unwrap();
                    HttpResponse::Ok().body(body)
                }
                else {
                    let lists = PostList::get_user_post_lists(list.user_id, 10, 0);
                    let body = serde_json::to_string(&PostList::get_json_anon_user_post_list (
                        owner,
                        list,
                        lists,
                        _limit,
                        _offset
                    ));
                    HttpResponse::Ok().body(body)
                }
            }
        }
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            info: "Минимальный запрос: list_id".to_string(),
        }).unwrap();
        HttpResponse::Ok().body(body)
    }
}
