use crate::session::Session;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

// type ResultWarp<T> = std::result::Result<T, Rejection>;

/// GET: /
/// 响应/请求的返回
pub async fn index(session: Session) -> Result<impl Reply, Rejection> {
    // log::debug!("GET: / 处理");
    let mut data = Map::new();

    data.insert("seo_title".to_string(), to_json("技术派"));
    data.insert("seo_keyword".to_string(), to_json(" dd"));
    data.insert("seo_description".to_string(), to_json("dd"));

    let html = view("index.html", data, session);

    // let html = "欢迎访问<跟我买车>后台首页(Hi Luck)";
    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回
}
