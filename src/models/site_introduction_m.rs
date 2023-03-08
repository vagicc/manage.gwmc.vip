use crate::db::get_connection;
use crate::schema::site_introduction;
use crate::schema::site_introduction::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，AsChangeset:更新，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Introduction {
    pub id: i32,
    pub title: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub content: Option<String>,
    pub last_time: Option<NaiveDateTime>,
}

///新增及更新结构体
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "site_introduction"]
pub struct NewIntroduction {
    pub title: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub content: Option<String>,
    pub last_time: Option<NaiveDateTime>,
}
impl NewIntroduction {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(site_introduction)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

///删除一条记录
/// pk: i32  表的主键ID,这里是id
pub fn delete(pk: i32) -> usize {
    let query = diesel::delete(site_introduction.find(pk));
    log::debug!(
        "site_introduction表删除SQL：{:?}",
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
            log::error!("site_introduction表删除数据失败：{}", e);
            0
        }
    }
}

pub fn modify(pky: i32, data: &NewIntroduction) -> Option<Introduction> {
    let query = diesel::update(site_introduction.find(pky)).set(data);
    log::debug!(
        "site_introduction表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Introduction>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("site_introduction表修改数据失败：{}", err);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<Introduction>, String) {
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

    let query_count = site_introduction.count();
    log::debug!(
        "site_introduction_list分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("site_introduction_list分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Introduction> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = site_introduction
        .order_by(id.desc())
        .limit(limit)
        .offset(offset);
    log::debug!(
        "site_introduction_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<Introduction>(&mut conn)
        .unwrap_or(data_null);

    pages = crate::pager::default_full("site/introduction", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

pub fn find_introduction(pky: i32) -> Option<Introduction> {
    let query = site_introduction.find(pky);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_introduction查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Introduction>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_introduction查无数据：{}", err);
            None
        }
    }
}
