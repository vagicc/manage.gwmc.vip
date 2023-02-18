use crate::session;
use crate::template::to_html_single;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

// 退出，注销登录
pub async fn quit() -> Result<impl Reply, Rejection> {
    log::info!("logout:退出登录处理");

    //直接重新设置cookie里的data为空就行
    let cookie = session::Cookie::new();
    log::warn!("Cookie:{:#?}", cookie);

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/login"));
    data.insert("message".to_string(), to_json("退出登录成功!!"));
    let html = to_html_single("hint.html", data);

    let response = warp::http::Response::builder()
        .status(warp::http::StatusCode::OK)
        .header(
            warp::http::header::SET_COOKIE,
            format!(
                "{}={}; SameSite=Strict; Max-Age=0; HttpOpnly",
                crate::constants::SESSION_NAME,
                serde_json::json!(cookie)
            ),
        )
        .body(html);
    Ok(response)
}
