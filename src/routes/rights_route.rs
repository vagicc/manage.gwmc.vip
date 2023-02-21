use crate::handlers::rights_handler;
use warp::Filter;

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path!("rights" / "index"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion:crate::session::Session| async {
            rights_handler::list(1, sesion).await
        });

    warp::get()
        .and(warp::path("rights"))
        .and(warp::path("index"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(rights_handler::list)
        .or(first)
        .or(add())
        .or(edit())
        .or(delete())
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("rights"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(rights_handler::do_new);

    warp::get()
        .and(warp::path!("rights" / "create"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(rights_handler::create_html)
        .or(post)
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("rights"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(rights_handler::do_edit);

    warp::get()
        .and(warp::path("rights"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(rights_handler::edit)
        .or(post)
}

pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("rights"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(rights_handler::delete)
}
