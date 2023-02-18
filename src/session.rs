use crate::constants::SESSION_NAME;
use crate::models::admins_model;
use serde::{Deserialize, Serialize};
use warp::http::Response;
use warp::Filter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub admin: admins_model::Admins,
    pub ip: Option<std::net::SocketAddr>,
    pub url_path: String, //处理后相对URL路径,不带参数与页数
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cookie {
    pub id: String,
    pub passwd: String,
    // ip_address -> Inet,
    pub timestamp: String,
    pub userid: i32,
}

impl Cookie {
    pub(crate) fn new() -> Cookie {
        Cookie {
            id: String::new(),
            passwd: String::new(),
            timestamp: String::new(),
            userid: 0,
        }
    }
}

//自定义错误:错误信息应该还要整理
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseError {
    pub error: String,
    pub error_description: String,
}
impl warp::reject::Reject for ResponseError {
    // into_response  //warp:src/reject.rs   448行
}

impl warp::Reply for ResponseError {
    fn into_response(self) -> warp::reply::Response {
        Response::new(
            format!(
                "出错提示message: {} =>description:{}",
                self.error, self.error_description
            )
            .into(),
        )
    }
}

///无效用户ID
#[derive(Debug)]
pub struct InvalidUserID;
impl warp::reject::Reject for InvalidUserID {}

///已退出登录
#[derive(Debug)]
pub struct Logout;
impl warp::reject::Reject for Logout {}

///未登录
#[derive(Debug)]
pub struct NoLogin;
impl warp::reject::Reject for NoLogin {}

///没权限
#[derive(Debug)]
pub struct NoRight;
impl warp::reject::Reject for NoRight {}

///未登录的全跳转到此处理no_login改 inaccessible  ,除了单单跳转到登录,还可以跳转到提示页面
pub async fn inaccessible(reject: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    log::warn!("用户未登录,直接跳转到登录页");

    // let ka = warp::reject::not_found(); //找不页面  missing_cookie
    // let ka=warp::reject::missing_cookie(); //找不页面  missing_cookie
    if reject.is_not_found() {
        log::warn!("找不到页面----!!");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else if let Some(e) = reject.find::<ResponseError>() {
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else if let Some(e) = reject.find::<NoLogin>() {
        //未登录
        log::debug!("未登录");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else if let Some(e) = reject.find::<InvalidUserID>() {
        //无效用户ID
        log::debug!("无效用户ID");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else if let Some(e) = reject.find::<Logout>() {
        //已退出登录
        log::debug!("已退出登录");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else if let Some(e) = reject.find::<NoRight>() {
        //没权限
        log::debug!("没权限");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    } else {
        log::warn!("其他情况^^^^^");
        Ok(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )))
    }

    // let k = warp::redirect::see_other(warp::http::Uri::from_static("/login"));
    // Ok(k)

    // if reject.is_not_found() {
    //     println!("handle_not_found => Ok ");

    //     Ok(warp::hyper::StatusCode::NOT_FOUND)
    // } else {
    //     println!("handle_not_found => Err ");
    //     Err(reject)
    // }
}

/// 带上session
pub fn with_session() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    //先检查是否登录（有无cookie，是否为有效登录），再检查用户对当前访问页是否有权限
    let cookie = warp::filters::cookie::optional(SESSION_NAME)
        // .map(move |cookie: Option<String>| {
        //     log::error!("11111===111:{:#?}", cookie);
        //     cookie
        // })
        .and_then(check_cookie)
        .and(warp::path::full())
        // .map(move |session: Session, path: warp::path::FullPath| {
        //     println!("请求的路径：{:?}", path);
        //     session
        // });
        .and(warp::addr::remote()) //客户来源IP地址
        .and_then(check_right);
    cookie
}

// 检查用户是否有权限
async fn check_right(
    admin: admins_model::Admins,
    path: warp::path::FullPath,
    ip: Option<std::net::SocketAddr>,
) -> Result<Session, warp::Rejection> {
    // log::warn!("请求的路径：{:?}", path);
    let url_path = check_full_path(path.as_str().to_string()); //处理过后的url相对路径
    if crate::models::rights_model::permit(url_path.clone(), &admin) {
        let s = Session {
            admin: admin,
            ip: ip,
            url_path: url_path,
        };
        Ok(s)
    } else {
        // 这里无权限
        //错误信息应该还要整理
        // let my = ResponseError {
        //     error: "无权限".to_string(),
        //     error_description: "用户无权限".to_string(),
        // };
        // let emy = Err(warp::reject::custom(my));

        // emy
        Err(warp::reject::custom(NoRight))
    }
}

//处理路径:只要两段路径,去掉第三段的页数
fn check_full_path(full_path: String) -> String {
    // 路径为根时，也就是访问首页时
    if full_path.len() == 1 {
        // log::warn!("处理后返回的PATH：{}", full_path);
        return full_path;
    }

    let path_vec: Vec<&str> = full_path.trim_matches('/').split("/").collect();
    // log::warn!("原来的path: {}", full_path);
    // log::warn!("path_vec: {:#?}", path_vec);
    // log::warn!("path_vec长度: {}", path_vec.len());

    if path_vec.len() < 2 {
        // log::debug!("原来的path: {}", full_path);
        // log::debug!("path_vec: {:#?}", path_vec);
        // log::debug!("path_vec长度: {}", path_vec.len());
        // log::warn!("处理后返回的PATH：{}", path_vec[0]);
        return path_vec[0].to_string();
    }

    // log::debug!(
    //     "处理后返回的PATH：{}",
    //     format!("{}/{}", path_vec[0], path_vec[1])
    // );

    // 直接只返回前两断。一般第三断为分页的页数
    format!("{}/{}", path_vec[0], path_vec[1])
}

async fn check_cookie(cookie: Option<String>) -> Result<admins_model::Admins, warp::Rejection> {
    let admin = match cookie {
        Some(cookie_encode) => {
            log::debug!("cookie_encode:{:#?}", cookie_encode);

            // 这三行如果出错,则cookie是伪造的
            let cookie_decode = base64::decode(cookie_encode).expect("解cookie时出错");
            let cookie_string = String::from_utf8(cookie_decode).expect("cookie转string出错");
            let s = serde_json::from_str::<Cookie>(cookie_string.as_str()).unwrap();
            log::debug!("带有cookien: {:#?}", s);

            // 这里还要判断时间戳是否过期 timestamp

            //是否有userid,表里是否有用户,用户密码是否正确
            if s.userid < 1 {
                //用户id小于1表示已退出
                Err(warp::reject::custom(Logout))
            } else {
                let option_admin = admins_model::find_admin(s.userid);

                if option_admin.is_none() {
                    //无效的用户ID
                    return Err(warp::reject::custom(InvalidUserID));
                }

                let admin = option_admin.unwrap();
                if admin.password != s.passwd {
                    let my = ResponseError {
                        error: "用户已更改过密码".to_string(),
                        error_description: "用户已更改过密码".to_string(),
                    };
                    let emy = Err(warp::reject::custom(my));
                    return emy;
                }

                Ok(admin)
            }
        }
        None => {
            log::warn!("头无token，无权限,未登录");
            Err(warp::reject::custom(NoLogin))
        }
    };
    admin
}
