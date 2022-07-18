use crate::handlers::reptile_handler;
use warp::{filters::BoxedFilter, Filter};

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("reptile" / "list"))
        .and(warp::path::end())
        .and_then(reptile_handler::list)
        .or(detail())
}

pub fn detail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("detail"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(reptile_handler::detail)
}
