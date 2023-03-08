use warp::Filter;

use crate::routes::admins_route;
use crate::routes::carousel_route;
use crate::routes::home_route;
use crate::routes::lawsuit_autocar_route;
use crate::routes::login_route;
use crate::routes::logout_route;
use crate::routes::menus_route;
use crate::routes::navbar_route;
use crate::routes::reptile_route;
use crate::routes::rights_route;
use crate::routes::role_route;
use crate::routes::site_route;
use crate::routes::stock_route;
use crate::routes::upload_route;
use crate::routes::websockets_route;

pub fn all_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    //.well-known目录用于申请https证书
    let well = warp::path(".well-known").and(warp::fs::dir("./static/.well-known"));
    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let home = home_route::index();
    let login = login_route::index();
    let logout = logout_route::quit();
    // let demo = demo_route::all();

    let menus = menus_route::index();
    let admins = admins_route::index();
    let role = role_route::index();
    let rights = rights_route::index();
    let upload = upload_route::summernote();

    let reptile = reptile_route::list();
    let lawsuit_autocar = lawsuit_autocar_route::list();
    let site = site_route::index();
    let navbar = navbar_route::list();
    let carousel = carousel_route::list();
    let stock = stock_route::list();

    let websocket = websockets_route::echo();

    let routes = home
        .or(well)
        .or(dir)
        .or(favicon)
        .or(login)
        .or(logout)
        .or(menus)
        .or(admins)
        .or(role)
        .or(rights)
        .or(upload)
        .or(reptile)
        .or(lawsuit_autocar)
        .or(site)
        .or(navbar)
        .or(carousel)
        .or(stock)
        .or(websocket)
        .recover(crate::session::inaccessible);
    routes
}
