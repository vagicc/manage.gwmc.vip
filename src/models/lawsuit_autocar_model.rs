use crate::db::get_connection;
use crate::schema::lawsuit_autocar;
use crate::schema::lawsuit_autocar::dsl::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::NaiveDateTime;
use diesel::data_types::Cents; //i64 单位为分, Money的列表时直接用i64
use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
use serde::ser::{Serialize, SerializeStruct, Serializer};

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_autocar"]
pub struct NewLawsuitAutocar {
    pub acid: Option<i32>,                  //车辆分类表ID
    pub title: String,                      //标题
    pub summary: String,                    //车摘要
    pub list_img: Option<String>,           //封面图-列表图
    pub visit: i64,                         //浏览次数
    pub price_base: Cents,                  //起拍价
    pub current_price: Cents,               //当前价
    pub assess_price: Cents,                //评估价
    pub margin: Cents,                      //保证金
    pub recommended_price: Cents,           //最高推荐价
    pub start_time: Option<NaiveDateTime>,  //开拍时间
    pub end_time: Option<NaiveDateTime>,    //结束时间
    pub recommend: i16,                     //推荐星数1-10
    pub address: Option<String>,            //标地物详细地址
    pub disposal_unit: Option<String>,      //处置单位:所属法院
    pub external_url: Option<String>,       //拍卖详情URL
    pub belong: Option<i16>,                //所属平台（1.淘宝、2.京东）
    pub stage: Option<String>,              //拍卖阶段（一拍、二拍、变卖、撤回）
    pub status: i16, //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
    pub show: Option<bool>, //是否展示
    pub create_time: Option<NaiveDateTime>, //创建时间
}
impl NewLawsuitAutocar {
    pub fn insert(&mut self) -> i32 {
        /* 处理创建时间 */
        if self.create_time.is_none() {
            let now_date_time = crate::common::now_naive_date_time();
            self.create_time = Some(now_date_time);
        }

        let connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_autocar)
            .values(self.clone())
            .returning(id)
            .get_result::<i32>(&connection)
            .unwrap_or(0);
        insert_id
    }
}
