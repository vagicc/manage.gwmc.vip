use crate::db::get_connection;
use crate::schema::rights;
use crate::schema::rights::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Rights {
    pub right_id: i32,
    pub right_name: Option<String>,
    pub path_full: String,
    pub right_detail: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "rights"]
pub struct NewRights {
    pub right_name: Option<String>,
    pub path_full: String,
    pub right_detail: Option<String>,
}
impl NewRights {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(rights)
            .values(self)
            .returning(right_id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

// 更新数据结构体
#[derive(AsChangeset)]
#[table_name = "rights"]
pub struct UpdateRights {
    pub right_name: Option<String>,
    pub path_full: String,
    pub right_detail: Option<String>,
}

//结构体类型更新数据
pub fn modify(id: i32, data: &UpdateRights) -> Option<Rights> {
    let query = diesel::update(rights.find(id)).set(data);
    log::debug!(
        "rights表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    let result = query.get_result::<Rights>(&mut conn);

    match result {
        Ok(changed) => Some(changed),
        Err(e) => {
            log::error!("rights表修改数据失败：{}", e);
            None
        }
    }
}

//删除一条记录
pub fn delete(id: i32) -> usize {
    let query = diesel::delete(rights.find(id));
    log::debug!(
        "rights表删除SQL：{:?}",
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
            log::error!("rights表删除数据失败：{}", e);
            0
        }
    }
}

pub fn find_right(id: i32) -> Option<Rights> {
    let query = rights.find(id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_right查询SQL：{:?}", sql);
    let mut connection = get_connection();

    match query.first::<Rights>(&mut connection) {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("find_right查无数据：{}", e);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: u32  每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: u32) -> (i64, Vec<Rights>, String) {
    let mut limit: i64 = per as i64; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
        //u32是无符号整数,也就是大于0
        // if offset < 0 {
        //     offset = 0;
        // }
    }

    let query_count = rights.count();
    log::debug!(
        "rights_list分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("rights_list分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Rights> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = rights
        .order_by(right_id.desc())
        .then_order_by(path_full.desc())
        .limit(limit)
        .offset(offset);

    log::debug!(
        "rights_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Rights>(&mut conn).unwrap_or(data_null);
    pages = crate::pager::default_full("rights/index", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

pub fn get_all_right() -> Option<Vec<Rights>> {
    let query = rights
        .order_by(path_full.desc())
        .then_order_by(right_id.desc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::error!("get_all_right查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.get_results::<Rights>(&mut connection);
    match result {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_all_right查无数据：{}", err);
            None
        }
    }
}

pub fn get_right_id(url_path: String) -> Option<i32> {
    let query = rights.select(right_id).filter(path_full.eq(url_path));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    // log::debug!("get_right_id查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<i32>(&mut connection);
    match result {
        Ok(r_id) => Some(r_id),
        Err(e) => {
            log::debug!("get_right_id查无数据：{},应该再主动插入此条权限", e);
            None
        }
    }
}

/// 是否有允许访问权限
/// url_path:String 路径path_full
/// 返回bool
pub fn permit(url_path: String, admin: &crate::models::admins_model::Admins) -> bool {
    let role = admin.role.unwrap_or(0);

    //处理url_path="/role/index/8"这样子的分页
    // let page_url="/role/index/8".to_string();
    // let k:Vec<&str>=url_path.trim_matches('/').split("/").collect();
    // let len=k.len();
    // println!("full_path:{},长:{}",url_path,len);
    // println!("{:#?}",k);

    // 查找路径,如不存在权限表则新插入
    let rightid = get_right_id(url_path.clone());
    if rightid.is_none() {
        //插入新的权限
        let newdata = NewRights {
            right_name: None,
            path_full: url_path,
            right_detail: None,
        };
        let _ = newdata.insert();

        //非admin组,得分配权限后才能访问
        if role != 1 {
            return false;
        }
    }

    //role=1是admin的“超级用户组，”直接返回，拥有所有权限
    if role == 1 {
        return true;
    }

    let rightid = rightid.unwrap();
    //查找所属角色组,再对比是否有些权限ID
    use crate::models::roles_model;
    let roles = roles_model::find_role(role);
    if roles.is_some() {
        return is_in_rights(rightid, &roles.unwrap());

        // let rights_option = roles.unwrap().rights;
        // if rights_option.is_some() {
        //     let rights_array = rights_option.unwrap();
        //     // let k=rights_array.get_key();
        //     for r in rights_array {
        //         if r == rightid {
        //             return true;
        //         }
        //     }
        // }
    }

    false
}

///检查权限ID是否存在于权限数组里
pub fn is_in_rights(rightid: i32, roles: &crate::models::roles_model::Role) -> bool {
    let rights_option = roles.rights.clone();
    if rights_option.is_some() {
        let rights_array = rights_option.unwrap();
        // let k=rights_array.get_key();
        for r in rights_array {
            if r.is_none() {
                continue;
            }
            if r.unwrap() == rightid {
                return true;
            }
        }
    }
    false
}
