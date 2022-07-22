use crate::db::get_connection;
use crate::schema::lawsuit_autocar_photo;
use crate::schema::lawsuit_autocar_photo::dsl::*;
use diesel::prelude::*;

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
        let connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_autocar_photo)
            .values(self)
            .returning(lapid)
            .get_result::<i32>(&connection)
            .unwrap_or(0);
        insert_id
    }
}