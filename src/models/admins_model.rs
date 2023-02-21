use crate::db::get_connection;
use crate::schema::admins;
use crate::schema::admins::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Admins {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<i32>,
    pub status: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "admins"]
pub struct NewAdmin {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<i32>,
    pub status: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}

impl NewAdmin {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        diesel::insert_into(admins)
            .values(self)
            .returning(id)
            .get_result::<i32>(&mut connection)
            .unwrap_or(0)
    }
}

pub fn edit(pky: i32, post: &crate::handlers::admins_handler::AdminPost) -> Option<Admins> {
    let mut conn = get_connection();

    if post.password.is_empty() {
        let query = diesel::update(admins.find(pky)).set((
            username.eq(post.username.clone()),
            email.eq(post.email.clone()),
            mobile.eq(post.mobile.clone()),
            role.eq(post.role),
            status.eq(post.status),
            // last_login.eq()
        ));
        log::error!(
            "admins表更新数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );

        match query.get_result::<Admins>(&mut conn) {
            Ok(result) => Some(result),
            Err(err) => {
                log::error!("admins表修改数据失败：{}", err);
                None
            }
        }
    } else {
        let new_salt = get_new_salt();
        let passwd = encryption(&post.password, &new_salt);
        let update = NewAdmin {
            username: post.username.clone(),
            password: passwd,
            salt: new_salt,
            email: Some(post.email.clone()),
            mobile: Some(post.mobile.clone()),
            role: Some(post.role),
            status: Some(post.status),
            create_time: None, //这个创建时间是否也改变了，如果是，那这里不合理
            last_login: None,
        };

        modify(pky, &update)
    }
}

pub fn modify(pky: i32, data: &NewAdmin) -> Option<Admins> {
    let query = diesel::update(admins.find(pky)).set(data);
    log::error!(
        "admins表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Admins>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("admins表修改数据失败：{}", err);
            None
        }
    }
}

//删除一条记录
pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(admins.find(pky));
    log::debug!(
        "admins表删除SQL：{:?}",
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
            log::error!("admins表删除数据失败：{}", e);
            0
        }
    }
}

pub fn find_admin(admin_id: i32) -> Option<Admins> {
    let query = admins.find(admin_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_admin查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Admins>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_admin查无数据：{}", err);
            None
        }
    }
}

pub fn get_admin(user_name: String) -> Option<Admins> {
    let query = admins.filter(username.eq(user_name));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_admin查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Admins>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("get_admin查无数据：{}", err);
            None
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: u32  每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list(page: Option<u32>, per: u32) -> (i64, Vec<Admins>, String) {
    let mut limit: i64 = per as i64; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
        //u32是无符号整数,也就是大于0
        // if offset < 0 {
        //     offset = 0;
        // }
    }

    let query_count = admins.count();
    log::debug!(
        "admins_list分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count = query_count
        .get_result::<i64>(&mut conn)
        .expect("admins_list分页数量查询出错");

    let mut pages = String::new();
    let data_null: Vec<Admins> = Vec::new();

    if count <= 0 {
        // return (count, data_null, pages);
    }

    let query = admins.order_by(id.desc()).limit(limit).offset(offset);
    log::debug!(
        "admins_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Admins>(&mut conn).unwrap_or(data_null);
    (count, list, pages)
}

// 生成新的加密盐
pub fn get_new_salt() -> String {
    let new_salt = crate::common::random_key(10);
    new_salt
}

pub fn get_sha1(passwd: &str) -> String {
    let sha1_string = sha1::Sha1::from(passwd).digest().to_string();
    println!("Sha1加密后：{}", sha1_string);
    println!("Sha1加密后长度：{}", sha1_string.len());
    sha1_string
}

// 加密
pub fn encryption(passwd: &String, new_salt: &String) -> String {
    let new_passwd = format!("{}luck{}", passwd, new_salt).to_owned();
    let sha1_passwd = get_sha1(&new_passwd);
    sha1_passwd
}
