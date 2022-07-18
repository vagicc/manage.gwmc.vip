use crate::handlers::login_handler;
use warp::{filters::BoxedFilter, Filter};

pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let login_form = warp::get()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and_then(login_handler::login_form);
    login_form
}
