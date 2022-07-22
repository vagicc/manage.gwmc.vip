use crate::models::lawsuit_reptile_model;
use crate::template::to_html_single;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// 登录表单
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

pub async fn detail(id: i32) -> std::result::Result<impl Reply, Rejection> {
    match lawsuit_reptile_model::get_id(id) {
        Some(autocar) => {
            let mut data = Map::new();
            data.insert("autocar".to_string(), to_json(autocar));
            let html = to_html_single("reptile_detail.html", data);
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
    pub price_base: f64,        //起拍价
    pub current_price: f64,     //当前价
    pub assess_price: f64,      //评估价
    pub margin: f64,            //保证金
    pub recommended_price: f64, //最高推荐价
    pub start_time: String,     //开拍时间
    pub end_time: String,       //结束时间/
    pub recommend: i16,         //推荐星数1-10
    pub address: String,        //标的物地址
    pub disposal_unit: String,  //处置单位,法院
    pub external_url: String,   //拍卖详情URL
    pub belong: i16,            //所属平台（1.淘宝、2.京东）
    pub stage: String,          //拍卖阶段（一拍、二拍、变卖、撤回）
    pub push: bool,             //是否推送
    pub summary: String,        //车摘要
    pub description: String,    //文章内容
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
) -> std::result::Result<impl Reply, Rejection> {
    println!("把数据写到lawsuit_reptile表 {:#?}", form);

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

            let mut data = NewLawsuitAutocar {
                acid: None,                                      //车辆分类表ID
                title: form.title.clone().trim().to_string(),    //标题
                summary: form.summary.clone(),                   //车摘要
                list_img: None,                                  //封面图-列表图
                visit: 0,                                        //浏览次数
                price_base: Cents(price_base),                   //起拍价
                current_price: Cents(current_price),             //当前价
                assess_price: Cents(assess_price),               //评估价
                margin: Cents(margin),                           //保证金
                recommended_price: Cents(recommended_price),     //最高推荐价
                start_time: None,                                //开拍时间
                end_time: None,                                  //结束时间
                recommend: form.recommend,                       //推荐星数1-10
                address: Some(form.address.clone()),             //标地物详细地址
                disposal_unit: Some(form.disposal_unit.clone()), //处置单位:所属法院
                external_url: Some(form.external_url.clone()),   //拍卖详情URL
                belong: Some(form.belong),                       //所属平台（1.淘宝、2.京东）
                stage: Some(form.stage.clone()), //拍卖阶段（一拍、二拍、变卖、撤回）
                status: 1, //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
                show: Some(form.push), //是否展示
                create_time: None,
            };
            let laid = data.insert();

            let content = form.description;
            if !content.is_empty() {
                use crate::models::lawsuit_autocar_article_model::LawsuitAutocarArticle;
                let mut new_article = LawsuitAutocarArticle {
                    laid: laid,
                    article_content: Some(content),
                    create_time: None,
                };
                let new_article = new_article.insert();
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
