use crate::handlers::stock_handler;
use warp::Filter;

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let demo = warp::get()
        .and(warp::path!("stock" / "demo"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(stock_handler::demo_html);

    let first = warp::get()
        .and(warp::path!("stock" / "rise_fall"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion: crate::session::Session| async {
            stock_handler::rise_fall_list(1, sesion).await
        });

    warp::get()
        .and(warp::path("stock"))
        .and(warp::path("rise_fall"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(stock_handler::rise_fall_list)
        .or(first)
        .or(demo)
        .or(add())
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("stock"))
        .and(warp::path("rise_fall"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(stock_handler::new_rise_fall);

    let create_html = warp::get()
        .and(warp::path("stock"))
        .and(warp::path("rise_fall"))
        .and(warp::path("create"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|id: i32, session: crate::session::Session| async {
            stock_handler::create_html(session).await
        });

    let noon_and_evening = warp::post()
        .and(warp::path("stock"))
        .and(warp::path("rise_fall"))
        .and(warp::path("create"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(stock_handler::create_noon_and_evening);

    warp::get()
        .and(warp::path!("stock" / "rise_fall" / "create"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(stock_handler::create_html)
        .or(post)
        .or(create_html)
        .or(noon_and_evening)
}
