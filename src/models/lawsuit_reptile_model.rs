use crate::db::get_connection;
use crate::schema::lawsuit_reptile;
use crate::schema::lawsuit_reptile::dsl::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::NaiveDateTime;
use diesel::data_types::Cents; //i64 单位为分, Money的列表时直接用i64
use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, Queryable, PartialEq, Eq)]
pub struct LawsuitReptile {
    pub id: i32,
    pub title: String,
    pub list_img: Option<String>,
    pub price_base: Cents,
    pub current_price: Cents,
    pub assess_price: Cents,
    pub margin: Cents,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub lng: Option<BigDecimal>,
    pub lat: Option<BigDecimal>,
    pub address: Option<String>,
    pub disposal_unit: Option<String>,
    pub external_url: Option<String>,
    pub belong: Option<i16>,
    pub stage: Option<String>,
    pub status: i16,
    pub push: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
}
// 加上Serialize特征: Cents与BigDecimal无特征,所以手动添加
impl Serialize for LawsuitReptile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 19).unwrap();
        s.serialize_field("id", &self.id).unwrap();
        s.serialize_field("title", &self.title).unwrap();
        s.serialize_field("list_img", &self.list_img).unwrap();
        // let mut pricebase = 0.00;
        // if self.price_base.0 > 0 {
        //     let te = self.price_base.0 as f64;
        //     pricebase = te / 100.;
        // }

        // 处理金额 Cents PgMoney
        let mut pricebase = self.price_base.0 as f64;
        pricebase /= 100.;
        s.serialize_field("price_base", &pricebase).unwrap();
        let pricebase = (self.current_price.0 as f64) / 100.;
        s.serialize_field("current_price", &pricebase).unwrap();
        let pricebase = (self.assess_price.0 as f64) / 100.;
        s.serialize_field("assess_price", &pricebase).unwrap();
        let pricebase = (self.margin.0 as f64) / 100.;
        s.serialize_field("margin", &pricebase).unwrap();
        s.serialize_field("start_time", &self.start_time).unwrap();
        s.serialize_field("end_time", &self.end_time).unwrap();

        // 处理BigDecimal
        let temp = self.lng.clone();
        if temp.is_none() {
            s.serialize_field("lng", "").unwrap();
        } else {
            // let kd = temp.unwrap().to_f32().unwrap();
            let kd = temp.unwrap().to_f64().unwrap();
            s.serialize_field("lng", &kd).unwrap();
        }

        let temp = self.lat.clone();
        if temp.is_none() {
            s.serialize_field("lat", "").unwrap();
        } else {
            // let kd = temp.unwrap().to_f32().unwrap();
            let kd = temp.unwrap().to_f64().unwrap();
            s.serialize_field("lat", &kd).unwrap();
        }

        s.serialize_field("address", &self.address).unwrap();
        s.serialize_field("disposal_unit", &self.disposal_unit)
            .unwrap();
        s.serialize_field("external_url", &self.external_url)
            .unwrap();
        s.serialize_field("belong", &self.belong).unwrap();
        s.serialize_field("stage", &self.stage).unwrap();
        s.serialize_field("status", &self.status).unwrap();
        s.serialize_field("push", &self.push).unwrap();
        s.serialize_field("create_time", &self.create_time).unwrap();

        s.end()
    }
}

pub fn get_id(primary: i32) -> Option<LawsuitReptile> {
    let query = lawsuit_reptile.find(primary);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_id查询SQL：{:?}", sql);

    let mut conn = get_connection();
    let result = query.first::<LawsuitReptile>(&mut conn);

    match result {
        Ok(data) => Some(data),
        Err(error) => {
            match error {
                diesel::result::Error::NotFound => {
                    log::debug!("表lawsuit_reptile查无ID（{}）数据", primary);
                }
                _ => {
                    log::error!("查询出错：{:#?}", error);
                    // panic!("查找用户质次申请数据出错"); //这里&connection可能不要中断程序
                }
            }
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<crate::handlers::reptile_handler::GetQuery>,
) -> (i64, Vec<LawsuitReptile>, String) {
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

    let mut push_where = false;
    let mut tilte_where = String::new();

    if let Some(get_data) = whe {
        push_where = get_data.push;
        tilte_where = get_data.title;
    }

    // let mut kt = push.eq(push_where);
    // let mut query = lawsuit_reptile.filter(kt);
    let mut query = lawsuit_reptile.filter(push.eq(push_where));
    // if !tilte_where.is_empty() {
    //     log::error!("标题非空:{}", tilte_where);
    //     query.filter(title.like(tilte_where));
    // }

    let query_count = query.count();
    log::debug!(
        "lawsuit_reptile分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("lawsuit_reptile分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<LawsuitReptile> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::warn!(
        "lawsuit_reptile分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<LawsuitReptile>(&mut conn)
        .unwrap_or(data_null);

    // let page = page.unwrap_or(1);
    pages = crate::pager::default_full("reptile/list", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

// 返回所有数据,无分页
pub fn get_list() -> Vec<LawsuitReptile> {
    let mut conn = get_connection();
    // let query = lawsuit_reptile
    //     .order_by(id.desc())
    //     .get_results::<LawsuitReptile>(&conn)
    //     .unwrap_or_else(|_op| {
    //         let temp: Vec<LawsuitReptile> = Vec::new();
    //         return temp;
    //     });

    let query = lawsuit_reptile.filter(push.eq(false)).order_by(id.desc());
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("list查询SQL：{:?}", sql);

    let list = query
        .get_results::<LawsuitReptile>(&mut conn)
        .unwrap_or_else(|_op| {
            let temp: Vec<LawsuitReptile> = Vec::new();
            return temp;
        });

    list
}

// 更改推送状态
pub fn update_push(pkey: i32, is_push: bool) {
    let query = diesel::update(lawsuit_reptile.find(pkey)).set(push.eq(is_push));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("update_push=>SQL：{:?}", sql);

    let mut conn = get_connection();
    let update_result = query.get_result::<LawsuitReptile>(&mut conn);
}

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_reptile"]
pub struct NewLawsuitReptile {
    pub title: String,
    pub price_base: Cents,    //起拍价
    pub current_price: Cents, //当前价
    pub assess_price: Cents,  //评估价
    pub margin: Cents,        //保证金
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub address: Option<String>,       //标地物详细地址
    pub disposal_unit: Option<String>, //处置单位:所属法院
    pub external_url: Option<String>,
    pub belong: Option<i16>,   //所属平台（1.淘宝、2.京东）
    pub stage: Option<String>, //拍卖阶段（一拍、二拍、变卖、撤回）
    pub status: i16,           //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
    pub create_time: Option<NaiveDateTime>,
}
impl NewLawsuitReptile {
    pub fn insert(&self) -> i32 {
        /* 处理创建时间 */
        // if self.create_time.is_none() {
        //     let now_date_time = crate::common::now_naive_date_time();
        //     self.create_time = Some(now_date_time);
        // }

        let mut connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_reptile)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0);
        insert_id
    }
}
