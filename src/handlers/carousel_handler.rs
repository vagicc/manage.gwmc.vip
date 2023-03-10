use crate::models::carousel_model;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

/// GET: carousel/list
pub async fn list(page: u32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    log::debug!("网站介绍固定文章列表-分页");

    let (count, list, pages) = carousel_model::list(Some(page), Some(crate::constants::PER_PAGE));

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("carousel/list.html", data, session);

    Ok(warp::reply::html(html))
}

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let html = view("carousel/create.html", data, session);
    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CarouselPost {
    pub subhead: String,
    pub title: String,
    pub summary: String,
    pub link: String,
    pub path: String,
    pub show: bool,
    pub sort_order: i16,
}
impl CarouselPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.title.is_empty() {
            return Err("导航栏按纽名不能为空");
        }
        if self.link.is_empty() {
            return Err("导航栏链接");
        }

        Ok(self.clone())
    }
}

pub async fn new_carousel(
    form: CarouselPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let new_data = carousel_model::NewCarousel {
                subhead: Some(post.subhead),
                title: post.title,
                summary: Some(post.summary),
                link: post.link,
                path: Some(post.path),
                show: Some(post.show),
                sort_order: post.sort_order,
                last_time: None,
            };
            let insert_id = new_data.insert();
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
        "/carousel/list",
    )))
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = carousel_model::find_carousel(id);
    if edit.is_none() {
        log::warn!("查无此数据:carousel表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/carousel/list"));
        data.insert("message".to_string(), to_json("查无此数据:carousel表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("carousel/edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: CarouselPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let new_data = carousel_model::NewCarousel {
                subhead: Some(post.subhead),
                title: post.title,
                summary: Some(post.summary),
                link: post.link,
                path: Some(post.path),
                show: Some(post.show),
                sort_order: post.sort_order,
                last_time: None,
            };
            let updated = carousel_model::modify(id, &new_data);
            if updated.is_none() {
                // return //更新出错
            }
            // return //更新成功
        }
        Err(message) => {
            log::debug!("修改表单认证不通过：{}", message);
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/carousel/list",
    )))
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = carousel_model::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/carousel/list",
    )))
}

/* 多选删除 */
pub async fn expurgate(
    ids: Vec<i32>,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    for id in ids {
        let _deleted_rows = carousel_model::delete(id);
    }
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/carousel/list",
    )))
}