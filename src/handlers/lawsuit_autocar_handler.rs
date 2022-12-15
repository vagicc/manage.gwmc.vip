use crate::models::lawsuit_autocar_model;
use crate::template::to_html_single;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// 直接写在路由上了
// pub async fn list_no_page() -> ResultWarp<impl Reply> {
//     list_page(1).await
// }

// 分页显示
pub async fn list(page: u32) -> ResultWarp<impl Reply> {
    let per: u32 = 8; //每页总数
    let (count, list) = lawsuit_autocar_model::get_list(Some(page), Some(per));
    let pages = crate::pager::default_full("lawsuit/autocar", count, page, per);
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(list.len()));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));

    let html = to_html_single("lawsuit_autocar_list.html", data);
    Ok(warp::reply::html(html))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LawsuitAutocarForm {
    pub title: String,
    pub list_img: String,        //列表图
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
    pub show: bool,              //是否推送
    pub summary: String,         //车摘要
    pub description: String,     //文章内容
}
impl LawsuitAutocarForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.recommended_price < self.price_base {
            return Err("推荐价不能低于起拍价");
        }

        // 推荐文章见容不能为空
        if self.description.is_empty() {
            return Err("法拍车推荐文章见容不能为空");
        }

        Ok(self.clone())
    }
}

/*
接收POST数据
 */
pub async fn edit(id: i32, form: LawsuitAutocarForm) -> ResultWarp<impl Reply> {
    log::warn!("接收到POST");
    match form.validate() {
        Ok(post) => {
            // 先修改文章
            use crate::models::lawsuit_autocar_article_model::update_content;
            use diesel::data_types::Cents; //i64
            update_content(id, post.description);

            // 再修改lawsuit_autocar表
            let current_price = (form.current_price * 100.) as i64;
            let recommended_price = (form.recommended_price * 100.) as i64;
            let update = lawsuit_autocar_model::UpdateLawsuitAutocar {
                title: &post.title[..],
                summary: &post.summary[..],
                license: &post.license[..],
                violating: &post.violating[..],
                universal_model: &post.universal_model[..],
                gearbox: &post.gearbox[..],
                fuel_type: &post.fuel_type[..],
                kilometer: post.kilometer,
                autocar_model: Some(post.autocar_model),
                vim: Some(post.vim),
                engine_number: Some(post.engine_number),
                emission: Some(post.emission),
                current_price: Cents(current_price),
                recommended_price: Cents(recommended_price),
                recommend: post.recommend,
                // address: Some(post.address),
                show: Some(post.show),
            };
            let temp = lawsuit_autocar_model::modify(id, &update);
            log::error!("到这里:{:#?}", temp);
        }
        Err(message) => {
            log::warn!("用户修改数据不合法：{}", message);
        }
    }

    let detail = lawsuit_autocar_model::get_id(id);

    if detail.is_none() {
        log::warn!("查无此数据:lawsuit_autocar表无ID:{}", id);
        // return Err(warp::reject::not_found()); //错误的返回
    }

    use crate::models::lawsuit_autocar_article_model;
    use crate::models::lawsuit_autocar_photo_model;

    let detail = detail.unwrap();
    let article = lawsuit_autocar_article_model::get_article(id);
    let photo = lawsuit_autocar_photo_model::get_autocar_photo(id);

    let mut data = Map::new();
    data.insert("detail".to_string(), to_json(detail));
    data.insert("article".to_string(), to_json(article));
    data.insert("photo".to_string(), to_json(photo));

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

// 输出要修改的html
pub async fn detail(id: i32) -> ResultWarp<impl Reply> {
    log::info!("输出修改推荐");
    let detail = lawsuit_autocar_model::get_id(id);

    if detail.is_none() {
        log::warn!("查无此数据:lawsuit_autocar表无ID:{}", id);
        return Err(warp::reject::not_found()); //错误的返回
    }

    use crate::models::lawsuit_autocar_article_model;
    use crate::models::lawsuit_autocar_photo_model;

    let detail = detail.unwrap();
    let article = lawsuit_autocar_article_model::get_article(id);
    let photo = lawsuit_autocar_photo_model::get_autocar_photo(id);
    let mut data = Map::new();
    data.insert("detail".to_string(), to_json(detail));
    data.insert("article".to_string(), to_json(article));
    data.insert("photo".to_string(), to_json(photo));

    let html = to_html_single("lawsuit_autocar_edit.html", data);
    Ok(warp::reply::html(html))
}
