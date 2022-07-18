use crate::models::lawsuit_reptile_model;
use crate::template::to_html_single;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// 登录表单
pub async fn list() -> ResultWarp<impl Reply> {
    log::info!("抓到要推荐的车列表");

    let list = lawsuit_reptile_model::get_list();

    let mut data = Map::new();

    data.insert("list_len".to_string(), to_json(list.len())); //
                                                              // data.insert("list".to_string(), to_json(list)); //
    let list_json = to_json(list);
    // log::debug!("list_json:{:#?}", list_json);
    data.insert("list".to_string(), list_json); //

    let html = to_html_single("reptile_list.html", data);

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

pub async fn detail(id: i32) -> std::result::Result<impl Reply, Rejection> {
    match lawsuit_reptile_model::get_id(id) {
        Some(autocar) => {
            let mut data = Map::new();
            data.insert("autocar".to_string(), to_json(autocar));
            let html = to_html_single("reptile_detail.html", data);
            log::info!("输出详情");
            Ok(warp::reply::html(html))
        }
        None => {
            log::info!("查无数据，输出404");

            Err(warp::reject::not_found()) //返回404页面
        }
    }
}
