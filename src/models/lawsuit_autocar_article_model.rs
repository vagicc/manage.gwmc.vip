use crate::db::get_connection;
use crate::schema::lawsuit_autocar_article;
use crate::schema::lawsuit_autocar_article::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体合二为一(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "lawsuit_autocar_article"]
pub struct LawsuitAutocarArticle {
    pub laid: i32,
    pub article_content: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}
impl LawsuitAutocarArticle {
    pub fn insert(&mut self) -> Option<LawsuitAutocarArticle> {
        /* 处理expires有效时间 */
        if self.create_time.is_none() {
            let now_date_time = crate::common::now_naive_date_time();
            self.create_time = Some(now_date_time);
        }

        let connection = get_connection();
        let insert_data = diesel::insert_into(lawsuit_autocar_article)
            .values(self.clone())
            .get_result::<LawsuitAutocarArticle>(&connection); //这里还要处理，不能中断程序

        match insert_data {
            Ok(data) => {
                log::debug!("表(lawsuit_autocar_article)成功插入数据：{:?}", data);
                return Some(data);
            }
            Err(database_error) => {
                // println!("插入出错: {:#?}", database_error); // diesel::result::Error::DatabaseError;
                // crate::common::type_v(database_error);  //查看所属性形
                /*
                Err(
                DatabaseError(
                    UniqueViolation,
                    "duplicate key value violates unique constraint \"oauth_access_tokens_pkey\"",
                )
                */
                match database_error {
                    diesel::result::Error::DatabaseError(error_kind, message) => {
                        log::debug!("表(lawsuit_autocar_article)插入错误：{:#?}", error_kind); //2022-05-10 14:20:53 [DEBUG] - 数据插入错误：UniqueViolation
                        log::debug!("表(lawsuit_autocar_article)插入错误提示：{:#?}", message);
                        //2022-05-10 14:20:53 [DEBUG] - 数据插入错误提示："duplicate key value violates unique constraint \"oauth_access_tokens_pkey\""
                    }
                    _ => {
                        log::error!(
                            "表(lawsuit_autocar_article)插入其它类型错误：{:#?}",
                            database_error
                        );
                    }
                }
                return None;
            }
        }
    }
}

pub fn get_article(id: i32) -> Option<LawsuitAutocarArticle> {
    let query = lawsuit_autocar_article.find(id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("lawsuit_autocar_article查询（get_article）SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.first::<LawsuitAutocarArticle>(&connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("查无数据：{}", e);
            return None;
        }
    }
}

pub fn update_content(id: i32, content: String) {
    let query = diesel::update(lawsuit_autocar_article.find(id)).set(article_content.eq(content));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("update_content=>SQL：{:?}", sql);

    let conn = get_connection();
    let update_result = query.get_result::<LawsuitAutocarArticle>(&conn);
}
