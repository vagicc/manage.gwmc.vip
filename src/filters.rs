use warp::Filter;

use crate::routes::home_route;
use crate::routes::lawsuit_autocar_route;
use crate::routes::login_route;
use crate::routes::reptile_route;
use crate::routes::websockets_route;

pub fn all_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let home = home_route::index();
    let login = login_route::login();
    // let demo = demo_route::all();

    let reptile = reptile_route::list();
    let lawsuit_autocar = lawsuit_autocar_route::list();

    let websocket = websockets_route::echo();

    let routes = home
        .or(dir)
        .or(favicon)
        .or(login)
        .or(reptile)
        .or(lawsuit_autocar)
        .or(websocket);
    routes
}
