use crate::handlers::introduction_h;
use warp::Filter;

/// GET: /site/introduction
pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path!("site" / "introduction"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion: crate::session::Session| async {
            introduction_h::list(1, sesion).await
        });

    warp::get()
        .and(warp::path("site"))
        .and(warp::path("introduction"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(introduction_h::list)
        .or(first)
        .or(add())
        .or(edit())
        .or(delete())
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("introduction"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(introduction_h::new_article);

    warp::get()
        .and(warp::path!("introduction" / "create"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(introduction_h::create_html)
        .or(post)
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("introduction"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(introduction_h::do_edit);

    warp::get()
        .and(warp::path("introduction"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(introduction_h::edit)
        .or(post)
}

pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let del_arr = warp::post()
        .and(warp::path("introduction"))
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
                introduction_h::expurgate(ids, sesion).await
            },
        );

    warp::get()
        .and(warp::path("introduction"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(introduction_h::delete)
        .or(del_arr)
}
