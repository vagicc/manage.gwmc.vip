use crate::db::get_connection;
use crate::schema::lawsuit_autocar_photo;
use crate::schema::lawsuit_autocar_photo::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

//查询结构体,特别注意,这里的字段顺序也要和schema.rs中一样
#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct LawsuitAutocarPhoto {
    pub lapid: i32,                        //主键自增ID
    pub laid: i32,                         //司法拍卖机动车表(lawsuit_autocar)ID
    pub external_small: Option<String>,    //外链小图
    pub external_middle: Option<String>,   //外链中图
    pub external_original: Option<String>, //外链原图
    pub small: Option<String>,
    pub middle: Option<String>,
    pub original: Option<String>,
    pub path: Option<String>,
    pub title: Option<String>,
    pub extension: Option<String>,
    pub type_: Option<String>,
    pub front_cover: Option<bool>, //是否为封面图
    pub create_time: Option<NaiveDateTime>,
}

pub fn get_autocar_photo(autocar_id: i32) -> Vec<LawsuitAutocarPhoto> {
    let query = lawsuit_autocar_photo
        .filter(laid.eq(autocar_id))
        .order_by(lapid.asc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_autocar_photo查询SQL：{:?}", sql);

    let mut conn = get_connection();
    query
        .load::<LawsuitAutocarPhoto>(&mut conn)
        .unwrap_or_else(|op| {
            let temp: Vec<LawsuitAutocarPhoto> = Vec::new();
            return temp;
        })
}

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_autocar_photo"]
pub struct NewLawsuitAutocarPhoto {
    pub laid: i32,                         //司法拍卖机动车表(lawsuit_autocar)ID
    pub external_small: Option<String>,    //外链小图
    pub external_middle: Option<String>,   //外链中图
    pub external_original: Option<String>, //外链原图
    pub front_cover: Option<bool>,         //是否为封面图
}
impl NewLawsuitAutocarPhoto {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_autocar_photo)
            .values(self)
            .returning(lapid)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0);
        insert_id
    }
}
