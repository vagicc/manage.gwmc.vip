use crate::models::site_introduction_m;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

/// GET: site/introduction
pub async fn list(page: u32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    log::debug!("网站介绍固定文章列表-分页");

    let (count, list, pages) =
        site_introduction_m::list(Some(page), Some(crate::constants::PER_PAGE));

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("site/introduction_list.html", data, session);

    Ok(warp::reply::html(html))
}

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let html = view("site/introduction_create.html", data, session);
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = site_introduction_m::find_introduction(id);
    if edit.is_none() {
        log::warn!("查无此数据:site_introduction表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/admins/index"));
        data.insert("message".to_string(), to_json("查无此数据:Admins表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("site/introduction_edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: ArticlePost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let new_data = site_introduction_m::NewIntroduction {
                title: post.title,
                seo_title: Some(post.seo_title),
                seo_keywords: Some(post.seo_keywords),
                seo_description: Some(post.seo_description),
                content: Some(post.content),
                last_time: None, //这里应该设置为当前时间
            };
            let updated = site_introduction_m::modify(id, &new_data);
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
        "/site/introduction",
    )))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArticlePost {
    pub title: String,
    pub seo_title: String,
    pub seo_keywords: String,
    pub seo_description: String,
    pub content: String,
}
impl ArticlePost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.title.is_empty() || self.content.is_empty() {
            return Err("标题或文章内容不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn new_article(
    form: ArticlePost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let new_data = site_introduction_m::NewIntroduction {
                title: post.title,
                seo_title: Some(post.seo_title),
                seo_keywords: Some(post.seo_keywords),
                seo_description: Some(post.seo_description),
                content: Some(post.content),
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
        "/site/introduction",
    )))
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = site_introduction_m::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/site/introduction",
    )))
}

/* 多选删除 */
pub async fn expurgate(
    ids: Vec<i32>,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    for id in ids {
        let _deleted_rows = site_introduction_m::delete(id);
    }
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/site/introduction",
    )))
}
