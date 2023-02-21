use crate::models::admins_model;
use crate::template::{to_html_single, view};
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn list(page: u32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    log::debug!("后台用户列表-分页");

    let (count, list, pages) = admins_model::list(Some(page), crate::constants::PER_PAGE);

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("admins/admins_list.html", data, session);

    Ok(warp::reply::html(html))
}

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    use crate::models::roles_model::get_all_role;
    let all_roles = get_all_role();

    let mut data = Map::new();

    data.insert("all_roles".to_string(), to_json(all_roles));

    let html = view("admins/admins_create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AdminPost {
    pub username: String,
    pub password: String,
    pub email: String,
    pub mobile: String,
    pub role: i32,
    pub status: i64,
}
impl AdminPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            return Err("登录必填");
        }
        Ok(self.clone())
    }
}

pub async fn do_new(
    form: AdminPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let salt = admins_model::get_new_salt();
            let passwd = admins_model::encryption(&post.password, &salt);
            let new_data = admins_model::NewAdmin {
                username: post.username,
                password: passwd,
                salt: salt,
                email: Some(post.email),
                mobile: Some(post.mobile),
                role: Some(post.role),
                status: Some(post.status),
                create_time: None,
                last_login: None,
            };
            log::error!("插入new_data={:#?}", new_data);

            let insert_id = new_data.insert();
            log::error!("插入ID={}", insert_id);

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
        "/admins/index",
    )))
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = admins_model::find_admin(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/admins/index"));
        data.insert("message".to_string(), to_json("查无此数据:Admins表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    use crate::models::roles_model::get_all_role;
    let all_roles = get_all_role();

    data.insert("all_roles".to_string(), to_json(all_roles));
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("admins/admins_edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: AdminPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let updated = admins_model::edit(id, &post);
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
        "/admins/index",
    )))
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = admins_model::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/admins/index",
    )))
}
