use crate::handlers::lawsuit_autocar_handler;
use warp::Filter;

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let one_page = warp::get()
        .and(warp::path!("lawsuit" / "autocar"))
        .and(warp::path::end())
        // .and_then(lawsuit_autocar_handler::list_no_page);
        .and_then(|| async { lawsuit_autocar_handler::list(1).await });

    warp::get()
        .and(warp::path("lawsuit"))
        .and(warp::path("autocar"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(lawsuit_autocar_handler::list)
        .or(one_page)
        .or(edit())
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("lawsuit"))
        .and(warp::path("autocar"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form()) //warp::multipart::form()
        .and_then(lawsuit_autocar_handler::edit);     

    warp::get()
        .and(warp::path("lawsuit"))
        .and(warp::path("autocar"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(lawsuit_autocar_handler::detail)
        .or(post)
}
