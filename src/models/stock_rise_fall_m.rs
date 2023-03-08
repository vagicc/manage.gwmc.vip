use crate::db::get_connection;
use crate::schema::stock_rise_fall;
use crate::schema::stock_rise_fall::dsl::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，AsChangeset:更新，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct StockRiseFall {
    pub id: i32,
    pub record_date: NaiveDate,
    pub week: Option<String>,
    pub m_rise: Option<i32>,
    pub m_fall: Option<i32>,
    pub m_rise_limit: Option<i32>,
    pub m_limit_drop: Option<i32>,
    pub n_rise: Option<i32>,
    pub n_fall: Option<i32>,
    pub n_rise_limit: Option<i32>,
    pub n_limit_drop: Option<i32>,
    pub e_rise: Option<i32>,
    pub e_fall: Option<i32>,
    pub e_rise_limit: Option<i32>,
    pub e_limit_drop: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub last_time: Option<NaiveDateTime>,
}

///新增及更新结构体
#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "stock_rise_fall"]
pub struct NewStockRiseFall {
    pub record_date: NaiveDate,
    pub week: Option<String>,
    pub m_rise: Option<i32>,
    pub m_fall: Option<i32>,
    pub m_rise_limit: Option<i32>,
    pub m_limit_drop: Option<i32>,
    pub n_rise: Option<i32>,
    pub n_fall: Option<i32>,
    pub n_rise_limit: Option<i32>,
    pub n_limit_drop: Option<i32>,
    pub e_rise: Option<i32>,
    pub e_fall: Option<i32>,
    pub e_rise_limit: Option<i32>,
    pub e_limit_drop: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub last_time: Option<NaiveDateTime>,
}
impl NewStockRiseFall {
    pub fn insert(&mut self) -> i32 {
        /* 处理expires有效时间 */
        if self.create_time.is_none() {
            let now_date_time = crate::common::now_naive_date_time();
            self.last_time = Some(now_date_time);
        }

        let mut connection = get_connection();
        diesel::insert_into(stock_rise_fall)
            .values(self.clone())
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

///删除一条记录
/// pk: i32  表的主键ID,这里是id
pub fn delete(pk: i32) -> usize {
    let query = diesel::delete(stock_rise_fall.find(pk));
    log::debug!(
        "stock_rise_fall表删除SQL：{:?}",
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
            log::error!("stock_rise_fall表删除数据失败：{}", e);
            0
        }
    }
}

pub fn modify(pky: i32, data: &NewStockRiseFall) -> Option<StockRiseFall> {
    let query = diesel::update(stock_rise_fall.find(pky)).set(data);
    log::debug!(
        "stock_rise_fall表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<StockRiseFall>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("stock_rise_fall表修改数据失败：{}", err);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<StockRiseFall>, String) {
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

    let query_count = stock_rise_fall.count();
    log::debug!(
        "stock_rise_fall分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("stock_rise_fall分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<StockRiseFall> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = stock_rise_fall
        .order_by(record_date.desc())
        .limit(limit)
        .offset(offset);
    log::debug!(
        "stock_rise_fall分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<StockRiseFall>(&mut conn)
        .unwrap_or(data_null);

    pages = crate::pager::default_full(
        "stock/rise_fall",
        count,
        page.unwrap_or(1),
        limit as u32,
    );
    (count, list, pages)
}

pub fn find_stock_rise_fall(pky: i32) -> Option<StockRiseFall> {
    let query = stock_rise_fall.find(pky);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_stock_rise_fall查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<StockRiseFall>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_stock_rise_fall查无数据：{}", err);
            None
        }
    }
}
