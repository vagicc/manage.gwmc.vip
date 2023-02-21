use crate::db::get_connection;
use crate::schema::roles;
use crate::schema::roles::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    // pub rights: Option<String>, //字段设置“数组类型”，http://www.postgres.cn/docs/12/arrays.html
    // std::vec::Vec
    pub rights: Option<Vec<Option<i32>>>,
    //rights -> Nullable<Array<Nullable<Int4>>>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "roles"]
pub struct NewRole {
    pub name: String,
    pub rights: Option<Vec<i32>>,
    pub default: Option<String>,
}
impl NewRole {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(roles)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

pub fn get_all_role() -> Option<Vec<Role>> {
    let query = roles.order_by(id.desc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::error!("get_all_role查询SQL：{:?}", sql);

    let mut connection = get_connection();
    match query.get_results::<Role>(&mut connection) {
        Ok(list) => Some(list),
        Err(err) => {
            log::debug!("get_all_role查无数据：{}", err);
            None
        }
    }
}

pub fn find_role(role_id: i32) -> Option<Role> {
    let query = roles.find(role_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_role查询SQL：{:?}", sql);
    let mut connection = get_connection();
    match query.first::<Role>(&mut connection) {
        Ok(role) => Some(role),
        Err(e) => {
            log::debug!("find_role查无数据：{}", e);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<Role>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
        //u32是无符号整数,也就是大于0
        // if limit < 1 {
        //     limit = 1;
        // }
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
        //u32是无符号整数,也就是大于0
        // if offset < 0 {
        //     offset = 0;
        // }
    }

    let query_count = roles.count();
    log::debug!(
        "roles_list分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("roles_list分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Role> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = roles.order_by(id.desc()).limit(limit).offset(offset);
    log::debug!(
        "roles_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Role>(&mut conn).unwrap_or(data_null);

    pages = crate::pager::default_full("role/index", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

///删除一条记录
/// pk: i32  表的主键ID,这里是id
pub fn delete(pk: i32) -> usize {
    let query = diesel::delete(roles.find(pk));
    log::debug!(
        "roles表删除SQL：{:?}",
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
            log::error!("admins表删除数据失败：{}", e);
            0
        }
    }
}

pub fn modify(pky: i32, data: &NewRole) -> Option<Role> {
    let query = diesel::update(roles.find(pky)).set(data);
    log::error!(
        "roles表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Role>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("admins表修改数据失败：{}", err);
            None
        }
    }
}
