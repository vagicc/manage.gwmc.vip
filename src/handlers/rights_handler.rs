use crate::template::{to_html_single, view};
use crate::{models::rights_model, schema::roles::rights};
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = rights_model::find_right(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/rights/index"));
        data.insert("message".to_string(), to_json("查无此数据:rights表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("rights/rights_edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();

    // data.insert("all_right".to_string(), to_json(all_right));

    let html = view("rights/rights_create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = rights_model::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/rights/index",
    )))
}

pub async fn do_edit(
    id: i32,
    form: RightPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let update = rights_model::UpdateRights {
                right_name: Some(post.right_name),
                path_full: post.path_full,
                right_detail: Some(post.right_detail),
            };
            let updated = rights_model::modify(id, &update);
            if updated.is_none() {
                // return //更新出错
            }
            // return //更新成功
        }
        Err(message) => {
            // return 表单认证失败
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/rights/index",
    )))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RightPost {
    pub right_name: String,
    pub path_full: String,
    pub right_detail: String,
}
impl RightPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.path_full.is_empty() {
            return Err("路径必填");
        }
        Ok(self.clone())
    }
}

pub async fn do_new(
    form: RightPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let new_data = rights_model::NewRights {
                right_name: Some(post.right_name.clone()),
                path_full: post.path_full.clone(),
                right_detail: Some(post.right_name.clone()),
            };
            let insert_id = new_data.insert();

            if insert_id == 0 {
                // return 成败了
            }
            // return 成功了
        }
        Err(e) => {
            // return 表单认证失败
        }
    }

    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/rights/index",
    )))
}

pub async fn list(page: u32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    log::debug!("角色列表-分页");

    let (count, list, pages) = rights_model::list(Some(page), crate::constants::PER_PAGE);

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("rights/rights_list.html", data, session);

    Ok(warp::reply::html(html))
}
