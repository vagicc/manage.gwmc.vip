use crate::handlers::logout_handler;
use warp::Filter;

/// 退出登录：GET: /logout
pub fn quit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and_then(logout_handler::quit)
}
