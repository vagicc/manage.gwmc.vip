use crate::handlers::reptile_handler;
use warp::{filters::BoxedFilter, Filter};

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //接收GET查询条件
    let opt_query = warp::query::<reptile_handler::GetQuery>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<reptile_handler::GetQuery>,), std::convert::Infallible>((None,))
        });

    let first = warp::get()
        .and(warp::path!("reptile" / "list"))
        .and(warp::path::end())
        .and(opt_query)
        .and_then(|get: Option<reptile_handler::GetQuery>| async {
            reptile_handler::list_page(1,get).await
        });

    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("list"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(opt_query)
        .and_then(reptile_handler::list_page)
        .or(first)
        .or(new())
        .or(detail())

    // warp::get()
    //     .and(warp::path!("reptile" / "list"))
    //     .and(warp::path::end())
    //     .and_then(reptile_handler::list)
    //     .or(detail())
}

//
pub fn new() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("reptile"))
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(reptile_handler::new_reptile);
    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("new"))
        .and(warp::path::end())
        .and_then(reptile_handler::new_html)
        .or(post)
}

pub fn detail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("reptile"))
        .and(warp::path("detail"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form()) //warp::multipart::form()
        // .and(warp::multipart::form()) //warp::body::form()
        .and_then(reptile_handler::push_lawsuit_autocar);

    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("detail"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(reptile_handler::detail)
        .or(post)
}
