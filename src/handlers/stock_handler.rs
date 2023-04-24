use crate::models::stock_rise_fall_m;
use crate::template::to_html_single;
use crate::template::view;
use chrono::Datelike;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

/// GET: stock/list
pub async fn rise_fall_list(
    page: u32,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    log::debug!("网站介绍固定文章列表-分页");

    let (count, list, pages) =
        stock_rise_fall_m::list(Some(page), Some(crate::constants::PER_PAGE));

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("stock/rise_fall_list.html", data, session);

    Ok(warp::reply::html(html))
}

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let html = view("stock/rise_fall_create.html", data, session);
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn create_noon_and_evening_html(
    id: i32,
    r#type: String,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    data.insert("stock_type".to_string(), to_json(r#type)); //
    let html = view("stock/rise_fall_create.html", data, session);
    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RiseFallPost {
    /*
    morning
    noon
    evening
    */
    pub r#type: String,
    pub rise: i32,
    pub fall: i32,
    pub rise_limit: i32,
    pub limit_drop: i32,
}
impl RiseFallPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.rise < 0 {
            return Err("导航栏按纽名不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn create_noon_and_evening(
    id: i32,
    r#type: String,
    post: RiseFallPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match stock_rise_fall_m::find_stock_rise_fall(id) {
        Some(stock_rise) => {
            let mut updated: Option<stock_rise_fall_m::StockRiseFall> = None;
            if post.r#type.eq("noon") {
                let mut new_data = stock_rise_fall_m::NewStockRiseFall {
                    record_date: stock_rise.record_date,
                    week: stock_rise.week,
                    m_rise: stock_rise.m_rise,
                    m_fall: stock_rise.m_fall,
                    m_rise_limit: stock_rise.m_rise_limit,
                    m_limit_drop: stock_rise.m_limit_drop,
                    n_rise: Some(post.rise),
                    n_fall: Some(post.fall),
                    n_rise_limit: Some(post.rise_limit),
                    n_limit_drop: Some(post.limit_drop),
                    e_rise: None,
                    e_fall: None,
                    e_rise_limit: None,
                    e_limit_drop: None,
                    create_time: None,
                    last_time: None,
                };
                updated = stock_rise_fall_m::modify(id, &new_data);
            } else if post.r#type.eq("evening") {
                let mut new_data = stock_rise_fall_m::NewStockRiseFall {
                    record_date: stock_rise.record_date,
                    week: stock_rise.week,
                    m_rise: stock_rise.m_rise,
                    m_fall: stock_rise.m_fall,
                    m_rise_limit: stock_rise.m_rise_limit,
                    m_limit_drop: stock_rise.m_limit_drop,
                    n_rise: stock_rise.n_rise,
                    n_fall: stock_rise.n_fall,
                    n_rise_limit: stock_rise.n_rise_limit,
                    n_limit_drop: stock_rise.n_limit_drop,
                    e_rise: Some(post.rise),
                    e_fall: Some(post.fall),
                    e_rise_limit: Some(post.rise_limit),
                    e_limit_drop: Some(post.limit_drop),
                    create_time: stock_rise.create_time,
                    last_time: None,
                };
                updated = stock_rise_fall_m::modify(id, &new_data);
            }
        }
        None => {}
    }

    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/stock/rise_fall",
    )))
}

pub async fn new_rise_fall(
    form: RiseFallPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            println!("post:{:#?}", post);

            let mut insert_id: i32 = 0;
            let record_date = crate::common::now_naive_date();
            let week_num = record_date.weekday().num_days_from_monday();
            // | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
            // |Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday
            // | 星期一 | 星期二 | 星期三 | 星期四 | 星期五 | 星期六 | 星期日
            // | 0     | 1     | 2     | 3     | 4     | 5     | 6
            let week_arr = [
                "星期一",
                "星期二",
                "星期三",
                "星期四",
                "星期五",
                "星期六",
                "星期日",
            ];
            let week = week_arr[week_num as usize].to_string();
            println!("今天是星期几：{}", week);

            if post.r#type.eq("morning") {
                let mut new_data = stock_rise_fall_m::NewStockRiseFall {
                    record_date: record_date,
                    week: Some(week),
                    m_rise: Some(post.rise),
                    m_fall: Some(post.fall),
                    m_rise_limit: Some(post.rise_limit),
                    m_limit_drop: Some(post.limit_drop),
                    n_rise: None,
                    n_fall: None,
                    n_rise_limit: None,
                    n_limit_drop: None,
                    e_rise: None,
                    e_fall: None,
                    e_rise_limit: None,
                    e_limit_drop: None,
                    create_time: None,
                    last_time: None,
                };
                insert_id = new_data.insert();
            } else if post.r#type.eq("noon") {
                let mut new_data = stock_rise_fall_m::NewStockRiseFall {
                    record_date: record_date,
                    week: Some(week),
                    m_rise: None,
                    m_fall: None,
                    m_rise_limit: None,
                    m_limit_drop: None,
                    n_rise: Some(post.rise),
                    n_fall: Some(post.fall),
                    n_rise_limit: Some(post.rise_limit),
                    n_limit_drop: Some(post.limit_drop),
                    e_rise: None,
                    e_fall: None,
                    e_rise_limit: None,
                    e_limit_drop: None,
                    create_time: None,
                    last_time: None,
                };
                insert_id = new_data.insert();
            } else if post.r#type.eq("evening") {
                let mut new_data = stock_rise_fall_m::NewStockRiseFall {
                    record_date: record_date,
                    week: Some(week),
                    m_rise: None,
                    m_fall: None,
                    m_rise_limit: None,
                    m_limit_drop: None,
                    n_rise: None,
                    n_fall: None,
                    n_rise_limit: None,
                    n_limit_drop: None,
                    e_rise: Some(post.rise),
                    e_fall: Some(post.fall),
                    e_rise_limit: Some(post.rise_limit),
                    e_limit_drop: Some(post.limit_drop),
                    create_time: None,
                    last_time: None,
                };
                insert_id = new_data.insert();
            }

            log::debug!("插入ID={}", insert_id);
            if insert_id == 0 {
                // return 成败了
            }
            // return 成功了
        }
        Err(msg) => {
            log::debug!("表单认证不通过：{}", msg);
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/stock/rise_fall",
    )))
}

pub async fn demo_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let html = view("stock/demo.html", data, session);
    Ok(warp::reply::html(html)) //直接返回html
}
