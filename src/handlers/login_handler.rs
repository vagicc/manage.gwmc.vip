use crate::template::to_html_single;
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// 登录表单
pub async fn login_form() -> ResultWarp<impl Reply> {
    log::info!("输出登录表单");

    let mut data = Map::new();

    let html = to_html_single("login.html", data);

    let id = 0;

    // 判断用户是否登录过
    if id == 0 {
        log::info!("输出正常登录");
        Ok(warp::reply::html(html))
    } else {
        log::info!("输出404");

        Err(warp::reject::not_found()) //返回404页面
    }
}
