use crate::db::get_connection;
use crate::schema::navbar;
use crate::schema::navbar::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，AsChangeset:更新，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Navbar {
    pub id: i32,
    pub menu: String,
    pub link: String,
    pub show: Option<bool>,
    pub sort_order: i16,
    pub last_time: Option<NaiveDateTime>,
}

///新增及更新结构体
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "navbar"]
pub struct NewNavbar {
    pub menu: String,
    pub link: String,
    pub show: Option<bool>,
    pub sort_order: i16,
    pub last_time: Option<NaiveDateTime>,
}
impl NewNavbar {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(navbar)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

///删除一条记录
/// pk: i32  表的主键ID,这里是id
pub fn delete(pk: i32) -> usize {
    let query = diesel::delete(navbar.find(pk));
    log::debug!(
        "navbar表删除SQL：{:?}",
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
            log::error!("navbar表删除数据失败：{}", e);
            0
        }
    }
}

pub fn modify(pky: i32, data: &NewNavbar) -> Option<Navbar> {
    let query = diesel::update(navbar.find(pky)).set(data);
    log::debug!(
        "navbar表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Navbar>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("navbar表修改数据失败：{}", err);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<Navbar>, String) {
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

    let query_count = navbar.count();
    log::debug!(
        "navbar分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("navbar分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Navbar> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = navbar
        .order_by(sort_order.asc())
        .limit(limit)
        .offset(offset);
    log::debug!(
        "navbar分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Navbar>(&mut conn).unwrap_or(data_null);

    pages = crate::pager::default_full("navbar/list", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

pub fn find_navbar(pky: i32) -> Option<Navbar> {
    let query = navbar.find(pky);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_navbar查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Navbar>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_navbar查无数据：{}", err);
            None
        }
    }
}

// 返回可展示的导航栏
pub fn show_navbar() -> Vec<Navbar> {
    let query = navbar.filter(show.eq(true)).order_by(sort_order.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::warn!("show_navbar查询SQL：{:?}", sql);
    let mut connection = get_connection();
    query.get_results::<Navbar>(&mut connection).unwrap_or({
        // log::warn!("show_navbar查无数据");
        let default: Vec<Navbar> = Vec::new();
        default
    })
}
