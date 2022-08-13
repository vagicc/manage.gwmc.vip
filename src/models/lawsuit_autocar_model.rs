use crate::db::get_connection;
use crate::schema::lawsuit_autocar;
use crate::schema::lawsuit_autocar::dsl::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::data_types::Cents; //i64 单位为分, Money的列表时直接用i64
use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
use serde::ser::{Serialize, SerializeStruct, Serializer};

//查询结构体,  diesel表字段最多支持32个,所以^
#[derive(Debug, Clone, Queryable)]
pub struct LawsuitAutocar {
    pub id: i32,                            //主键ID
    pub title: String,                      //标题
    pub summary: String,                    //车摘要
    pub list_img: Option<String>,           //封面图-列表图
    pub license: Option<String>,            //车牌号
    pub violating: Option<String>,          //违章
    pub universal_model: Option<String>,    //通用车型号
    pub gearbox: Option<String>,            //变速箱(手动6档,自动档)
    pub fuel_type: Option<String>,          //燃料:汽油,柴油,纯电,油电混合,氢能电池,氢能
    pub kilometer: Option<i32>,             //已行驶公里数
    pub registration: Option<NaiveDate>,    //注册登记日期
    pub production_date: Option<NaiveDate>, //生产日期
    pub autocar_model: Option<String>,      //厂家车型
    pub vim: Option<String>,                //车架号
    pub engine_number: Option<String>,      //发动机号
    pub emission: Option<String>,           //排放阶段
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
// 加上Serialize特征: Cents与BigDecimal无特征,所以手动添加
impl Serialize for LawsuitAutocar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 19).unwrap();
        s.serialize_field("id", &self.id).unwrap();
        s.serialize_field("title", &self.title).unwrap();
        s.serialize_field("summary", &self.summary).unwrap();
        s.serialize_field("list_img", &self.list_img).unwrap();
        s.serialize_field("license", &self.license).unwrap();
        s.serialize_field("violating", &self.violating).unwrap();
        s.serialize_field("universal_model", &self.universal_model)
            .unwrap();
        s.serialize_field("gearbox", &self.gearbox).unwrap();
        s.serialize_field("fuel_type", &self.fuel_type).unwrap();
        s.serialize_field("kilometer", &self.kilometer).unwrap();
        s.serialize_field("registration", &self.registration)
            .unwrap();
        s.serialize_field("production_date", &self.production_date)
            .unwrap();
        s.serialize_field("autocar_model", &self.autocar_model)
            .unwrap();
        s.serialize_field("vim", &self.vim).unwrap();
        s.serialize_field("engine_number", &self.engine_number)
            .unwrap();
        s.serialize_field("emission", &self.emission).unwrap();

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
        s.serialize_field(
            "recommended_price",
            &((self.recommended_price.0 as f64) / 100.),
        )
        .unwrap();

        s.serialize_field("start_time", &self.start_time).unwrap();
        s.serialize_field("end_time", &self.end_time).unwrap();
        s.serialize_field("recommend", &self.recommend).unwrap();

        // 处理BigDecimal  sql: "lat" decimal DEFAULT NULL,
        // let temp = self.lng.clone();
        // if temp.is_none() {
        //     s.serialize_field("lng", "").unwrap();
        // } else {
        //     // let kd = temp.unwrap().to_f32().unwrap();
        //     let kd = temp.unwrap().to_f64().unwrap();
        //     s.serialize_field("lng", &kd).unwrap();
        // }

        // let temp = self.lat.clone();
        // if temp.is_none() {
        //     s.serialize_field("lat", "").unwrap();
        // } else {
        //     // let kd = temp.unwrap().to_f32().unwrap();
        //     let kd = temp.unwrap().to_f64().unwrap();
        //     s.serialize_field("lat", &kd).unwrap();
        // }

        s.serialize_field("address", &self.address).unwrap();
        s.serialize_field("disposal_unit", &self.disposal_unit)
            .unwrap();
        s.serialize_field("external_url", &self.external_url)
            .unwrap();
        s.serialize_field("belong", &self.belong).unwrap();
        s.serialize_field("stage", &self.stage).unwrap();
        s.serialize_field("status", &self.status).unwrap();
        s.serialize_field("show", &self.show).unwrap();
        s.serialize_field("create_time", &self.create_time).unwrap();

        s.end()
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
pub fn get_list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<LawsuitAutocar>) {
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

    let query = lawsuit_autocar.filter(show.eq(true));

    let query_count = query.count();
    log::debug!(
        "数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let conn = get_connection();
    let count: i64 = query_count.get_result(&conn).unwrap(); //查询总条数
    let data_null: Vec<LawsuitAutocar> = Vec::new();

    if count <= 0 {
        return (count, data_null);
    }

    //分页
    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始

    //分页,旧的
    // let query = lawsuit_autocar
    //     .filter(show.eq(true))
    //     .order_by(id.desc())
    //     .limit(limit) //取10条数据
    //     .offset(offset); //从第0条开始

    log::debug!(
        "get_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    (
        count,
        query.load::<LawsuitAutocar>(&conn).unwrap_or_else(|_op| {
            return data_null;
        }),
    )
}

pub fn get_id(primary_key: i32) -> Option<LawsuitAutocar> {
    let query = lawsuit_autocar.find(primary_key);
    log::debug!(
        "get_id查询SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let conn = get_connection();
    let result = query.first::<LawsuitAutocar>(&conn);

    match result {
        Ok(data) => Some(data),
        Err(error) => {
            match error {
                diesel::result::Error::NotFound => {
                    log::debug!("表lawsuit_autocar查无ID（{}）数据", primary_key);
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

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_autocar"]
pub struct NewLawsuitAutocar {
    pub title: String,                      //标题
    pub summary: String,                    //车摘要
    pub list_img: Option<String>,           //封面图-列表图
    pub license: Option<String>,            //车牌号
    pub violating: Option<String>,          //违章
    pub universal_model: Option<String>,    //通用车型号
    pub gearbox: Option<String>,            //变速箱(手动6档,自动档)
    pub fuel_type: Option<String>,          //燃料:汽油,柴油,纯电,油电混合,氢能电池,氢能
    pub kilometer: Option<i32>,             //已行驶公里数
    pub registration: Option<NaiveDate>,    //注册登记日期
    pub production_date: Option<NaiveDate>, //生产日期
    pub autocar_model: Option<String>,      //厂家车型
    pub vim: Option<String>,                //车架号
    pub engine_number: Option<String>,      //发动机号
    pub emission: Option<String>,           //排放阶段
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

// 更新数据结构体
#[derive(AsChangeset, Identifiable)]
#[table_name = "lawsuit_autocar"]
pub struct LawsuitAutocarForm {
    pub id: i32,                            //主键ID
    pub title: String,                      //标题
}

pub fn update_test(pkey:i32,tit:String){
    log::warn!("测试用结构体更新表数据！！！！");

    let up_data=LawsuitAutocarForm{
        id:pkey,
        title:tit,
    };

    let connection = get_connection();
    use crate::db::_my_pg_connection;
    let k=_my_pg_connection();
    // let change_data=up_data.save_changes(&connection);
}