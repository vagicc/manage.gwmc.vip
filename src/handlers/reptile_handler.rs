use crate::models::lawsuit_reptile_model;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// GET查询条件
#[derive(Debug, Deserialize, Serialize)]
pub struct GetQuery {
    pub title: String, //爬虫抓到的标题
    pub push: bool,    //是否已推送
}

pub async fn list_page(
    page: u32,
    get: Option<GetQuery>,
    session: Session,
) -> ResultWarp<impl Reply> {
    log::debug!("抓到要推荐的车列表-分页");
    log::warn!("GET查询条件：{:#?}", get);

    let per: u32 = 8; //每页总数
    let (count, list, pages) = lawsuit_reptile_model::list_page(Some(page), Some(per), get);

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    // let html = to_html_single("reptile_list.html", data);
    let html = view("reptile/list.html", data, session);

    Ok(warp::reply::html(html))
}

// 列表无分页-不用了
pub async fn list() -> ResultWarp<impl Reply> {
    log::info!("抓到要推荐的车列表");

    let list = lawsuit_reptile_model::get_list();

    let mut data = Map::new();

    data.insert("list_len".to_string(), to_json(list.len())); //
                                                              // data.insert("list".to_string(), to_json(list)); //
    let list_json = to_json(list);
    // log::debug!("list_json:{:#?}", list_json);
    data.insert("list".to_string(), list_json); //

    let html = to_html_single("reptile_list.html", data);

    let id = 0;

    // 判断用户是否登录过
    if id == 0 {
        log::info!("输出正常登录");
        Ok(warp::reply::html(html))
    } else {
        log::info!("输出404");

        Err(warp::reject::not_found()) //返回404页面
    }
}

pub async fn new_html(session: Session) -> ResultWarp<impl Reply> {
    log::debug!("[调试信息]访问了“/reptile/new”");
    let html = "欢迎访问<跟我买车>后台首页(Hi Luck)";
    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("reptile/new.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewReptile {
    pub paimai_id: String,   //拍卖ID
    pub belong: String,      //所属平台（1.淘宝、2.京东）
    pub html_string: String, //
}
impl NewReptile {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if !self.belong.eq("1") && !self.belong.eq("2") {
            return Err("所属平台（1.淘宝、2.京东）");
        }
        // if self.paimai_id < 0 {
        //     return Err("拍卖ID为大于0");
        // }

        //淘宝改为直接输入html
        if self.belong.eq("1") && self.html_string.is_empty() {
            return Err("淘宝平台请输入html");
        }

        Ok(self.clone())
    }
}

pub async fn new_reptile(form: NewReptile, session: Session) -> ResultWarp<impl Reply> {
    // log::warn!("post数据： {:#?}", form);
    let mut html = "后台命令去抓取法拍车".to_string();

    match form.validate() {
        Ok(post) => {
            log::info!("表彰：{:#?}", post);
            //所属平台（1.淘宝、2.京东）
            if post.belong.eq("1") {
                let url =
                    "https://sf-item.taobao.com/sf_item/{}.htm".replace("{}", &post.paimai_id);
                let url = url.as_str();

                let html = &post.html_string.clone();
                let html = html.as_str();
                // let html = &post.html_string.as_ref().as_str();
                let data = crate::parse::taobao_select(html).await;
                if data.is_none() {
                    log::error!("解析HTML得不到数据");
                }
                let data = data.unwrap();
                insert_table(data, url); //插入到表
            } else {
                //京东，直走以前的抓取
                // let program = "./target/debug/reptile";
                // let dir = "/home/luck/Code/跟我买车/reptile";

                let program = crate::common::get_env("reptile");
                let dir = crate::common::get_env("reptile_dir");
                let mut output = std::process::Command::new(program)
                    .current_dir(dir)
                    .arg(post.paimai_id)
                    .arg(post.belong)
                    .output()
                    .expect("执行不了命令");

                log::debug!("执行抓取法拍车命令结果：{:#?}", output);

                html = String::from_utf8(output.stdout).expect("命令执行无标准输出！");
            }
        }
        Err(e) => {
            log::warn!("表单认证不通过：{}", e);
            html = e.to_string();
        }
    }

    Ok(warp::reply::html(html)) //直接返回html
}

fn insert_table(data: crate::parse::Reptile, url: &str) {
    // 开始插入到表
    use crate::models::lawsuit_reptile_model::NewLawsuitReptile;
    use crate::models::lawsuit_reptile_photo_model::NewLawsuitReptilePhoto;
    use diesel::data_types::Cents; //i64

    // 两种字符转数字的方法 ￥241.00  这里单位为分，要×100;
    // let current_price = data.current_price.parse::<i64>().expect("字符串转i64出错");
    use std::str::FromStr;
    let current_price = f64::from_str(data.current_price.as_str())
        .map_err(|_| "i64转换失败？？")
        .unwrap();
    let current_price = (current_price * 100.) as i64;
    let price_base = (data.price_base * 100.) as i64;
    let assess_price = (data.assess_price * 100.) as i64;
    let margin = (data.margin * 100.) as i64;

    //毫秒和秒相差1000,但这样转换,不知道为何少了8个小时,所以主动加上去 8*3600
    let start_time =
        chrono::prelude::NaiveDateTime::from_timestamp(data.start_time / 1000 + 8 * 3600, 0);
    let end_time =
        chrono::prelude::NaiveDateTime::from_timestamp(data.end_time / 1000 + 8 * 3600, 0);
    let now_date_time = crate::common::now_naive_date_time();

    let new_data = NewLawsuitReptile {
        title: data.title,
        price_base: Cents(price_base),       //起拍价
        current_price: Cents(current_price), //当前价
        assess_price: Cents(assess_price),   //评估价
        margin: Cents(margin),               //保证金
        start_time: Some(start_time),
        end_time: Some(end_time),
        address: Some(data.address),             //标地物详细地址
        disposal_unit: Some(data.disposal_unit), //处置单位:所属法院
        external_url: Some(url.to_string()),
        belong: Some(data.belong), //所属平台（1.淘宝、2.京东）
        stage: Some(data.stage),   //拍卖阶段（一拍、二拍、变卖、撤回）
        status: 1,                 //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
        create_time: Some(now_date_time),
    };

    let insert_id = new_data.insert();
    println!("插入ID:{}", insert_id);

    let photos = data.photos;
    if !photos.is_empty() {
        let mut first = 1;
        let mut front_cover = true;
        for photo in photos {
            if first != 1 {
                front_cover = false;
            }
            let insert_photo = NewLawsuitReptilePhoto {
                lrid: insert_id,
                external_small: Some(photo.external_small),
                external_middle: Some(photo.external_middle),
                external_original: Some(photo.external_original),
                front_cover: Some(front_cover),
            };
            insert_photo.insert();
            first += 1;
        }
    }
}

pub async fn detail(id: i32, session: Session) -> std::result::Result<impl Reply, Rejection> {
    match lawsuit_reptile_model::get_id(id) {
        Some(autocar) => {
            let mut data = Map::new();
            data.insert("autocar".to_string(), to_json(autocar));
            // 去相册取得默认列表图
            use crate::models::lawsuit_reptile_photo_model;
            let photo = lawsuit_reptile_photo_model::get_front_cover(id);
            data.insert("photo".to_string(), to_json(photo));

            // let html = to_html_single("reptile_detail.html", data);
            let html = view("reptile/detail.html", data, session);
            log::info!("输出详情");
            Ok(warp::reply::html(html))
        }
        None => {
            log::info!("查无数据，输出404");

            Err(warp::reject::not_found()) //返回404页面
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LawsuitAutocarForm {
    pub title: String,
    pub list_img: String,        //
    pub price_base: f64,         //起拍价
    pub current_price: f64,      //当前价
    pub assess_price: f64,       //评估价
    pub margin: f64,             //保证金
    pub recommended_price: f64,  //最高推荐价
    pub start_time: String,      //开拍时间
    pub end_time: String,        //结束时间/
    pub recommend: i16,          //推荐星数1-10
    pub license: String,         //车牌号
    pub violating: String,       //是否有违章
    pub universal_model: String, //通用车型号
    pub gearbox: String,         //变速箱(手动6档,自动档)
    pub fuel_type: String,       //燃料:汽油,柴油,纯电,油电混合,氢能电池,氢能
    pub kilometer: i32,          //已行驶公里数
    pub registration: String,    //注册登记日期
    pub production_date: String, //生产日期
    pub autocar_model: String,   //厂家车型
    pub vim: String,             //车架号
    pub engine_number: String,   //发动机号
    pub emission: String,        //排放阶段
    pub address: String,         //标的物地址
    pub disposal_unit: String,   //处置单位,法院
    pub external_url: String,    //拍卖详情URL
    pub belong: i16,             //所属平台（1.淘宝、2.京东）
    pub stage: String,           //拍卖阶段（一拍、二拍、变卖、撤回）
    pub push: bool,              //是否推送
    pub summary: String,         //车摘要
    pub description: String,     //文章内容
}
impl LawsuitAutocarForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.recommended_price < self.price_base {
            return Err("推荐价不能低于起拍价");
        }

        Ok(self.clone())
    }
}

// 把数据写到lawsuit_reptile表
pub async fn push_lawsuit_autocar(
    id: i32,
    form: LawsuitAutocarForm,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("把数据写到lawsuit_reptile表 {:#?}", form);

    let input = form.validate();
    match input {
        Ok(form) => {
            log::debug!("把数据写到lawsuit_reptile表 {:#?}", form);
            let k = form.title.clone().trim().to_string();
            // 插入lawsuit_autocar,lawsuit_autocar_photo,lawsuit_autocar_article
            use crate::models::lawsuit_autocar_model::NewLawsuitAutocar;
            use diesel::data_types::Cents; //i64

            let price_base = (form.price_base * 100.) as i64;
            let assess_price = (form.assess_price * 100.) as i64;
            let current_price = (form.current_price * 100.) as i64;
            let margin = (form.margin * 100.) as i64;
            let recommended_price = (form.recommended_price * 100.) as i64;

            //处理开拍时间:2022-07-26T10:00:00
            let mut start_time: Option<NaiveDateTime> = None;
            if !form.start_time.is_empty() {
                let date_time = chrono::prelude::NaiveDateTime::parse_from_str(
                    form.start_time.clone().as_str(),
                    "%Y-%m-%dT%H:%M:%S",
                );
                start_time = match date_time {
                    Ok(date) => Some(date),
                    Err(err) => {
                        log::error!("开拍时间转换出错:{}", err);
                        None
                    }
                };
            }
            let mut end_time: Option<NaiveDateTime> = None;
            if !form.end_time.is_empty() {
                let date_time = chrono::prelude::NaiveDateTime::parse_from_str(
                    form.end_time.clone().as_str(),
                    "%Y-%m-%dT%H:%M:%S",
                );
                end_time = match date_time {
                    Ok(date) => Some(date),
                    Err(err) => {
                        log::error!("开拍时间转换出错:{}", err);
                        None
                    }
                };
            }

            //
            use chrono::{NaiveDate, NaiveDateTime};
            let mut registration: Option<NaiveDate> = None;
            if !form.registration.is_empty() {
                let tem = chrono::prelude::NaiveDate::parse_from_str(
                    form.registration.clone().as_str(),
                    "%Y-%m-%d",
                );
                registration = match tem {
                    Ok(date) => Some(date),
                    Err(err) => {
                        log::error!("车辆注册登记日期格式转换出错:{}", err);
                        None
                    }
                };
            }
            let mut production_date: Option<NaiveDate> = None;
            if !form.production_date.is_empty() {
                let tem = chrono::prelude::NaiveDate::parse_from_str(
                    form.production_date.clone().as_str(),
                    "%Y-%m-%d",
                );
                production_date = match tem {
                    Ok(date) => Some(date),
                    Err(err) => {
                        log::error!("车辆生产日期格式转换出错:{}", err);
                        None
                    }
                };
            }
            //处理公里数
            let mut kilometer: Option<i32> = None;
            if form.kilometer > 0 {
                kilometer = Some(form.kilometer);
            }

            let mut data = NewLawsuitAutocar {
                title: form.title.clone().trim().to_string(), //标题
                summary: form.summary.clone(),                //车摘要
                list_img: Some(form.list_img.clone()),        //封面图-列表图
                license: Some(form.license.clone()),          //车牌号
                violating: Some(form.violating.clone()),      //违章
                universal_model: Some(form.universal_model.clone()), //通用车型号
                gearbox: Some(form.gearbox.clone()),          //变速箱(手动6档,自动档)
                fuel_type: Some(form.fuel_type.clone()), //燃料:汽油,柴油,纯电,油电混合,氢能电池,氢能
                kilometer: kilometer,                    //已行驶公里数
                registration: registration,              //注册登记日期
                production_date: production_date,        //生产日期
                autocar_model: Some(form.autocar_model.clone()), //厂家车型
                vim: Some(form.vim.clone()),             //车架号
                engine_number: Some(form.engine_number.clone()), //发动机号
                emission: Some(form.emission.clone()), //排放阶段                         //排放阶段
                price_base: Cents(price_base),         //起拍价
                current_price: Cents(current_price),   //当前价
                assess_price: Cents(assess_price),     //评估价
                margin: Cents(margin),                 //保证金
                recommended_price: Cents(recommended_price), //最高推荐价
                start_time: start_time,                //开拍时间
                end_time: end_time,                    //结束时间
                recommend: form.recommend,             //推荐星数1-10
                address: Some(form.address.clone()),   //标地物详细地址
                disposal_unit: Some(form.disposal_unit.clone()), //处置单位:所属法院
                external_url: Some(form.external_url.clone()), //拍卖详情URL
                belong: Some(form.belong),             //所属平台（1.淘宝、2.京东）
                stage: Some(form.stage.clone()),       //拍卖阶段（一拍、二拍、变卖、撤回）
                status: 1, //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
                show: Some(form.push), //是否展示
                create_time: None,
            };
            let laid = data.insert();
            if laid == 0 {
                log::error!("法拍车推荐插入表出错,要插入的数据：{:#?}", data);
            }

            let content = form.description;
            if !content.is_empty() {
                use crate::models::lawsuit_autocar_article_model::LawsuitAutocarArticle;
                let mut new_article = LawsuitAutocarArticle {
                    laid: laid,
                    article_content: Some(content),
                    create_time: None,
                };
                let new_article = new_article.insert();
                if new_article.is_none() {
                    log::error!("法拍车文章插入出错,文章内容：{:#?}", new_article);
                }
            }

            // 处理相册
            use crate::models::lawsuit_reptile_photo_model::get_reptile_photo;
            let photos = get_reptile_photo(id);
            if !photos.is_none() {
                use crate::models::lawsuit_autocar_photo_model::NewLawsuitAutocarPhoto;
                for photo in photos.unwrap() {
                    // let external_small = photo.external_small.unwrap();
                    // let external_middle = photo.external_middle.unwrap();
                    // let external_original = photo.external_original.unwrap();
                    // let front_cover = photo.front_cover;
                    let insert_photo = NewLawsuitAutocarPhoto {
                        laid: laid,                                 //司法拍卖机动车表(lawsuit_autocar)ID
                        external_small: photo.external_small,       //外链小图
                        external_middle: photo.external_middle,     //外链中图
                        external_original: photo.external_original, //外链原图
                        front_cover: photo.front_cover,             //是否为封面图
                    };
                    let insertid = insert_photo.insert();
                    //这里可以处理添加列表图,也可以在前面
                }
            }

            //修改此条法拍爬虫为已推送过文章
            lawsuit_reptile_model::update_push(id, true);
        }
        Err(message) => {
            log::error!("用户输入错误:{}", message);
        }
    }

    let id = 0;

    // 判断用户是否登录过
    if id == 0 {
        log::info!("输出正常登录");
        let html = "成功".to_string();
        Ok(warp::reply::html(html))
    } else {
        log::info!("输出404");

        Err(warp::reject::not_found()) //返回404页面
    }
}
