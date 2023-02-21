use crate::models::roles_model;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn create_html(session: crate::session::Session) -> Result<impl Reply, Rejection> {
    use crate::models::rights_model::get_all_right;
    let all_right = get_all_right();

    let mut data = Map::new();

    data.insert("all_right".to_string(), to_json(all_right));

    let html = view("roles/role_create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn list(page: u32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    log::debug!("角色列表-分页");
    let (count, list, pages) = roles_model::list(Some(page), Some(crate::constants::PER_PAGE));

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    let html = view("roles/role_list.html", data, session);

    Ok(warp::reply::html(html))
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = roles_model::find_role(id);
    if edit.is_none() {
        log::warn!("查无此数据:rights表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/role/index"));
        data.insert("message".to_string(), to_json("查无此数据:Roles表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    let edit = edit.unwrap();

    use crate::models::rights_model::get_all_right;
    let all_right = get_all_right();
    //这里还要做是否选中处理
    let mut checkbox_data: Vec<CheckboxData> = Vec::new();
    if all_right.is_some() {
        checkbox_data = checkbox_is_checked(all_right.unwrap(), &edit.rights);
    }

    data.insert("all_right".to_string(), to_json(checkbox_data));
    data.insert("edit".to_string(), to_json(edit));
    let html = view("roles/role_edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct CheckboxData {
    right_id: i32,
    right_name: Option<String>,
    path_full: String,
    right_detail: Option<String>,
    is_checked: bool,
}

fn checkbox_is_checked(
    all: Vec<crate::models::rights_model::Rights>,
    checked: &Option<Vec<Option<i32>>>,
) -> Vec<CheckboxData> {
    let mut data: Vec<CheckboxData> = Vec::new();
    for rights in all {
        let mut is_checked = false;
        if checked.is_some() {
            let k = checked.clone().unwrap();
            is_checked = is_in_vec(&k.clone(), rights.right_id)
        }
        // let is_checked = is_in_vec(checked, &rights.right_id);
        data.push(CheckboxData {
            right_id: rights.right_id,
            right_name: rights.right_name,
            path_full: rights.path_full,
            right_detail: rights.right_detail,
            is_checked,
        });
    }
    data
}

fn is_in_vec(arr: &Vec<Option<i32>>, seach: i32) -> bool {
    for i in arr.iter() {
        if i.is_none(){
            continue;
        }
        if seach == i.unwrap() {
            return true;
        }
    }
    false
}

pub async fn do_edit(
    id: i32,
    form: RolePost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let rights: Vec<i32> = post
                .rights
                .split(',')
                .map(|x| x.parse::<i32>().expect("字符串转i32类型出错"))
                .collect();

            let new = roles_model::NewRole {
                name: post.name,
                rights: Some(rights),
                default: Some(post.default),
            };
            let updated = roles_model::modify(id, &new);
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
        "/role/index",
    )))
}

// use serde_json::Value;
// use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RolePost {
    pub name: String,
    pub rights: String,
    // pub rights: Map<String, Value>,
    // pub rights: Value,
    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    // pub rights: Vec<String>,
    // pub rights: Vec<Map<String, Value>>,
    // pub tags[]:Value,
    // pub rights: HashMap<String, String>,
    pub default: String,
}
impl RolePost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.name.is_empty() {
            return Err("角色名必填");
        }
        if self.rights.is_empty() {
            return Err("权限不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn do_new(
    form: RolePost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            println!("=======接受到的POST=====:{:#?}", post);

            let rights: Vec<i32> = post
                .rights
                .split(',')
                .map(|x| x.parse::<i32>().expect("字符串转i32类型出错"))
                .collect();
            println!("{:?}", rights);
            // let temp: Vec<&str> = post.rights.split(',').collect();
            // println!("{:?}", temp);
            // let rights: Vec<i32> = temp
            //     .iter()
            //     .map(|x| x.parse::<i32>().expect("字符串转i64类型出错"))
            //     .collect();
            // println!("{:?}", rights);

            let data = roles_model::NewRole {
                name: post.name,
                rights: Some(rights),
                default: Some(post.default),
            };
            println!("插入的数据:{:#?}", data);

            let insert_id = data.insert();
            log::error!("插入ID={}", insert_id);

            if insert_id == 0 {
                // return 成败了
            }
            // return 成功了
        }
        Err(e) => {
            log::warn!("表单认证不通过：{}", e);
            // return 表单认证失败
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/role/index",
    )))
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = roles_model::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/role/index",
    )))
}

/* 多选删除 */
pub async fn expurgate(
    ids: Vec<i32>,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    for id in ids {
        let _deleted_rows = roles_model::delete(id);
    }
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/role/index",
    )))
}
