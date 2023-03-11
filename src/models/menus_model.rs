use crate::db::get_connection;
use crate::schema::menus;
use crate::schema::menus::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询，AsChangeset：更新) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Menu {
    pub id: i32,
    pub order_by: i16,
    pub path_full: Option<String>,
    pub name: String,
    pub level: Option<i16>,
    pub parent: Option<i32>,
    pub icon: Option<String>,
    pub department: Option<i32>,
    pub is_show: bool,
}

// 新增及修改结构体
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "menus"]
pub struct NewMenu {
    pub order_by: i16,
    pub path_full: Option<String>,
    pub name: String,
    pub level: Option<i16>,
    pub parent: Option<i32>,
    pub icon: Option<String>,
    pub department: Option<i32>,
    pub is_show: bool,
}

impl NewMenu {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(menus)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

// 通过url_path取得一行
pub fn get_current(url_path: String) -> Option<Menu> {
    let query = menus.filter(path_full.eq(url_path));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_current查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Menu>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("get_current查无数据：{}", err);
            None
        }
    }
}

pub fn get_parent(parent_id: i32) -> Option<Vec<Menu>> {
    let query = menus.filter(parent.eq(parent_id)).order_by(order_by.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_parent查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.get_results::<Menu>(&mut connection);
    match result {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_parent查无数据：{}", err);
            None
        }
    }
}

pub fn find_menu(pk: i32) -> Option<Menu> {
    let query = menus.find(pk);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_menu查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Menu>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_admin查无数据：{}", err);
            None
        }
    }
}

pub fn get_menu_level(lid: i16) -> Option<Vec<Menu>> {
    let query = menus.filter(level.eq(lid));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_menu_level查询SQL：{:?}", sql);
    let mut connection = get_connection();

    let result = query.get_results::<Menu>(&mut connection);
    match result {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_menu_level查无数据：{}", err);
            None
        }
    }
}

pub fn modify(pk: i32, data: &NewMenu) -> Option<Menu> {
    let query = diesel::update(menus.find(pk)).set(data);
    log::error!(
        "menus表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Menu>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("admins表修改数据失败：{}", err);
            None
        }
    }
}

//删除一条记录
pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(menus.find(pky));
    log::debug!(
        "menus表删除SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let deleted_rows = query.execute(&mut conn);
    // crate::common::type_v(deleted_rows);
    //变量值：Ok(1)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功1条数据
    //变量值：Ok(0)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功0条数据

    match deleted_rows {
        Ok(row) => row,
        Err(e) => {
            log::error!("menus表删除数据失败：{}", e);
            0
        }
    }
}

//后台左右菜单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeftMenu {
    pub id: i32,
    pub order_by: i16,
    pub path_full: Option<String>,
    pub name: String,
    pub level: Option<i16>,
    pub parent: Option<i32>,
    pub icon: Option<String>,
    pub department: Option<i32>,
    pub is_show: bool,
    pub child: Option<Vec<Menu>>,
}

pub fn get_left_menus() -> Option<Vec<LeftMenu>> {
    let query = menus
        .filter(level.eq(1))
        .filter(is_show.eq(true))
        .order_by(order_by.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_left_menus查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.get_results::<Menu>(&mut connection);

    if result.is_err() {
        log::debug!("get_left_menus查无数据：{:?}", result);
        return None;
    }

    let mut left_menus: Vec<LeftMenu> = Vec::new();

    for menu in result.unwrap() {
        left_menus.push(LeftMenu {
            id: menu.id,
            order_by: menu.order_by,
            path_full: menu.path_full,
            name: menu.name,
            level: menu.level,
            parent: menu.parent,
            icon: menu.icon,
            department: menu.department,
            is_show: menu.is_show,
            child: get_show_parent(menu.id),
        });
    }

    Some(left_menus)
}

pub fn get_show_parent(parent_id: i32) -> Option<Vec<Menu>> {
    let query = menus
        .filter(parent.eq(parent_id))
        .filter(is_show.eq(true))
        .order_by(order_by.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_show_parent查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.get_results::<Menu>(&mut connection);
    match result {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_show_parent查无数据：{}", err);
            None
        }
    }
}

///根据role_id返回菜单
pub fn role_left_menus(role_id: i32) -> Option<Vec<LeftMenu>> {
    let left_menus = get_left_menus();
    if left_menus.is_none() {
        return None;
    }

    if role_id == 1 {
        return left_menus;
    }

    use crate::models::roles_model;
    let roles = roles_model::find_role(role_id);
    if roles.is_none() {
        return None;
    }

    let roles = roles.unwrap();
    let left_menus = left_menus.unwrap();
    let mut my_menus: Vec<LeftMenu> = Vec::new();

    for left_menu in left_menus {
        if left_menu.path_full.is_some() {
            let url_path = left_menu.path_full.clone().unwrap();

            if is_have(url_path, &roles) {
                //存在,有权限,插入到my_menus,这用不处理child
                my_menus.push(left_menu);
            }
        } else {
            if left_menu.child.is_none() {
                continue;
            }
            let mut child: Option<Vec<Menu>> = None;
            let mut child: Vec<Menu> = Vec::new();
            for child_menu in left_menu.child.unwrap() {
                if child_menu.path_full.is_none() {
                    continue;
                }
                let url_path = child_menu.path_full.clone().unwrap();
                if is_have(url_path, &roles) {
                    //存在,有权限,插入到child
                    child.push(child_menu);
                }
            }
            //插入到my_menus,
            my_menus.push(LeftMenu {
                id: left_menu.id,
                order_by: left_menu.order_by,
                path_full: left_menu.path_full,
                name: left_menu.name,
                level: left_menu.level,
                parent: left_menu.parent,
                icon: left_menu.icon,
                department: left_menu.department,
                is_show: left_menu.is_show,
                child: Some(child),
            });
        }
    }

    Some(my_menus)
}

fn is_have(url_path: String, roles: &crate::models::roles_model::Role) -> bool {
    use crate::models::rights_model;
    // 查找路径,如不存在权限表则新插入
    let rightid = rights_model::get_right_id(url_path.clone());
    if rightid.is_none() {
        //插入新的权限
        let newdata = rights_model::NewRights {
            right_name: None,
            path_full: url_path,
            right_detail: None,
        };
        let _insert_id = newdata.insert();
        //这是没分配过权限
    } else {
        return rights_model::is_in_rights(rightid.unwrap(), roles);
    }
    false
}
