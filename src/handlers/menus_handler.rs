use crate::common::remove_menus_cache;
use crate::models::menus_model;
use crate::template::{to_html_single, view};
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn list(
    parent_id: i32,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    log::debug!("菜单列表");
    log::warn!("session:{:#?}", session);

    let mut parent_data: Option<menus_model::Menu> = None;
    if parent_id > 0 {
        parent_data = menus_model::find_menu(parent_id);
    }

    let list = menus_model::get_parent(parent_id);

    let mut data = Map::new();
    data.insert("list".to_string(), to_json(list)); //
    data.insert("parent_data".to_string(), to_json(parent_data)); //
    data.insert("parent_id".to_string(), to_json(parent_id)); //

    Ok(warp::reply::html(view(
        "menus/menus_list.html",
        data,
        session,
    )))
}

pub async fn create_html(
    parent_id: i32,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    //department
    let department = menus_model::get_menu_level(1);
    let left_menus = menus_model::role_left_menus(1);

    let mut data = Map::new();

    data.insert("department".to_string(), to_json(department));
    data.insert("menus".to_string(), to_json(left_menus));
    data.insert("parent_id".to_string(), to_json(parent_id));

    let html = view("menus/menus_create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MenuPost {
    pub order_by: i16,
    pub path_full: String,
    pub name: String,
    pub level: i16,
    // #[serde(skip_serializing_if = "0", default)] //设置默认值
    pub parent: i32,
    pub icon: String,
    pub department: i32,
    pub is_show: bool,
}
impl MenuPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.name.is_empty() {
            return Err("菜单名必填");
        }

        Ok(self.clone())
    }
}

pub async fn do_new(
    parent_id: i32,
    form: MenuPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            println!("=======接受到的POST=====:{:#?}", post);
            let data = menus_model::NewMenu {
                order_by: post.order_by,
                path_full: Some(post.path_full),
                name: post.name,
                level: Some(post.level),
                parent: Some(post.parent),
                icon: Some(post.icon),
                department: Some(post.department),
                is_show: post.is_show,
            };
            println!("插入的数据:{:#?}", data);

            let insert_id = data.insert();
            log::error!("插入ID={}", insert_id);

            if insert_id == 0 {
                // return 成败了
            }
            remove_menus_cache(); //删除菜单缓存

            // return 成功了
        }
        Err(e) => {
            log::warn!("表单认证不通过：{}", e);
            // return 表单认证失败
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/menus/index",
    )))
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = menus_model::find_menu(id);
    if edit.is_none() {
        log::warn!("查无此数据:menus表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/rights/index"));
        data.insert("message".to_string(), to_json("查无此数据:menus表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    data.insert("edit".to_string(), to_json(edit.unwrap()));

    let left_menus = menus_model::role_left_menus(1);
    data.insert("menus".to_string(), to_json(left_menus));

    let department = menus_model::get_menu_level(1);
    data.insert("department".to_string(), to_json(department));

    let html = view("menus/menus_edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: MenuPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            let update = menus_model::NewMenu {
                order_by: post.order_by,
                path_full: Some(post.path_full),
                name: post.name,
                level: Some(post.level),
                parent: Some(post.parent),
                icon: Some(post.icon),
                department: Some(post.department),
                is_show: post.is_show,
            };
            let updated = menus_model::modify(id, &update);
            if updated.is_none() {
                // return //更新出错
            }
            remove_menus_cache(); //删除菜单缓存

            // return //更新成功
        }
        Err(message) => {
            // return 表单认证失败
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/menus/index",
    )))
}

pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = menus_model::delete(id);

    remove_menus_cache(); //删除菜单缓存

    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/menus/index",
    )))
}

/* 多选删除 */
pub async fn expurgate(
    ids: Vec<i32>,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    for id in ids {
        let _deleted_rows = menus_model::delete(id);
    }

    remove_menus_cache(); //删除菜单缓存

    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/menus/index",
    )))
}
