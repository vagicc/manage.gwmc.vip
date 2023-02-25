use crate::handlers::summernote_h;
use crate::session::with_session;
use warp::Filter;

pub fn summernote() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("summernote"))
        .and(warp::path::end())
        .and(warp::multipart::form()) //warp::multipart::form()
        .and(with_session())
        .and_then(summernote_h::summernote);

    warp::get()
        .and(warp::path("upload"))
        .and(warp::path("summernote"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(summernote_h::upload_html)
        .or(post)
        .or(demo())
}

pub fn demo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("demo"))
        .and(warp::path::end())
        .and(warp::multipart::form()) //warp::multipart::form()
        .and(with_session())
        .and_then(summernote_h::upload_demo);

    warp::get()
        .and(warp::path("upload"))
        .and(warp::path("demo"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(summernote_h::demo_html)
        .or(post)
}
