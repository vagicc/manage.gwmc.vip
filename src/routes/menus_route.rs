use crate::handlers::menus_handler;
use warp::Filter;

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path!("menus" / "index"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion:crate::session::Session| async {
            menus_handler::list(0, sesion).await
        });

    warp::get()
        .and(warp::path("menus"))
        .and(warp::path("index"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(menus_handler::list)
        .or(first)
        .or(add())
        .or(edit())
        .or(delete())
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("menus"))
        .and(warp::path("create"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(menus_handler::do_new);

    warp::get()
        .and(warp::path("menus"))
        .and(warp::path("create"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(menus_handler::create_html)
        .or(post)
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("menus"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(menus_handler::do_edit);

    warp::get()
        .and(warp::path("menus"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(menus_handler::edit)
        .or(post)
}

pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let del_arr = warp::post()
        .and(warp::path("menus"))
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(
            |simple_map: std::collections::HashMap<String, String>,
             sesion:crate::session::Session| async move {
                // println!("post:{:#?}", simple_map);
                /*
                post:{
                    "ids": "2",
                    "del_ids": "13,12,11,4,2",
                    "checkbox1": "on",
                } */
                let del = simple_map.get("del_ids");

                let mut ids: Vec<i32> = Vec::new();
                if del.is_some() {
                    ids = del
                        .unwrap()
                        .split(",")
                        .map(|id| id.parse::<i32>().expect("多选删除转ID出错"))
                        .collect();
                }
                menus_handler::expurgate(ids, sesion).await
            },
        );

    warp::get()
        .and(warp::path("menus"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(menus_handler::delete)
        .or(del_arr)
}
