use crate::handlers::carousel_handler;
use warp::Filter;

/// GET: /carousel/list
pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path!("carousel" / "list"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion: crate::session::Session| async {
            carousel_handler::list(1, sesion).await
        });

    warp::get()
        .and(warp::path("carousel"))
        .and(warp::path("list"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(carousel_handler::list)
        .or(first)
        .or(add())
        .or(edit())
        .or(delete())
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("carousel"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(carousel_handler::new_carousel);

    warp::get()
        .and(warp::path!("carousel" / "create"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(carousel_handler::create_html)
        .or(post)
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("carousel"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(carousel_handler::do_edit);

    warp::get()
        .and(warp::path("carousel"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(carousel_handler::edit)
        .or(post)
}

pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let del_arr = warp::post()
        .and(warp::path("carousel"))
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(
            |simple_map: std::collections::HashMap<String, String>,
             sesion: crate::session::Session| async move {
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
                carousel_handler::expurgate(ids, sesion).await
            },
        );

    warp::get()
        .and(warp::path("carousel"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(carousel_handler::delete)
        .or(del_arr)
}
