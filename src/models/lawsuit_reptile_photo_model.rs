use crate::db::get_connection;
use crate::schema::lawsuit_reptile_photo;
use crate::schema::lawsuit_reptile_photo::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// 查询数据结构体
#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct LawsuitReptilePhoto {
    pub lrpid: i32,                        //自增ID
    pub lrid: i32,                         //司法拍卖机动车表(lawsuit_reptile)ID
    pub external_small: Option<String>,    //外链小图
    pub external_middle: Option<String>,   //外链中图
    pub external_original: Option<String>, //外链原图
    pub front_cover: Option<bool>,         //是否为封面图
}

pub fn get_front_cover(id: i32) -> Option<LawsuitReptilePhoto> {
    let query = lawsuit_reptile_photo
        .filter(lrid.eq(id))
        .filter(front_cover.eq(true));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_reptile_photo查询SQL：{:?}", sql);

    let mut conn = get_connection();
    let result = query.first::<LawsuitReptilePhoto>(&mut conn);

    match result {
        Ok(data) => Some(data),
        Err(error) => {
            match error {
                diesel::result::Error::NotFound => {
                    log::debug!("表lawsuit_reptile_photo查get_front_cover=（{}）数据", id);
                }
                _ => {
                    log::error!("查询出错：{:#?}", error);
                    // panic!("查找用户质次申请数据出错"); //这里可能不要中断程序
                }
            }
            None
        }
    }
}

pub fn get_reptile_photo(id: i32) -> Option<Vec<LawsuitReptilePhoto>> {
    let query = lawsuit_reptile_photo
        .filter(lrid.eq(id))
        .order_by(lrpid.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_reptile_photo查询SQL：{:?}", sql);

    let mut conn = get_connection();
    let result = query.get_results::<LawsuitReptilePhoto>(&mut conn);

    match result {
        Ok(data) => Some(data),
        Err(error) => {
            match error {
                diesel::result::Error::NotFound => {
                    log::debug!("表lawsuit_reptile_photo查无lrid=（{}）数据", id);
                }
                _ => {
                    log::error!("查询出错：{:#?}", error);
                    // panic!("查找用户质次申请数据出错"); //这里可能不要中断程序
                }
            }
            None
        }
    }
}
