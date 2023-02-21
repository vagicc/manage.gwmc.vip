use crate::constants::SESSION_NAME;
use crate::models::admins_model;
use crate::session;
use crate::template::to_html_single;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

pub async fn show_login_html() -> Result<impl Reply, Rejection> {
    // log::debug!("输出登录表单");
    let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );

    Ok(warp::reply::html(to_html_single("login.html", data)))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub passwd: String,
}
impl LoginForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            Err("登录名不能为空")
        } else if self.passwd.is_empty() {
            Err("密码不能为空")
        } else {
            Ok(self.clone())
        }
    }
}

pub async fn check_login(form: LoginForm) -> Result<impl Reply, Rejection> {
    //错误返回登录页，成功刚跳转到到默认页
    let mut data = Map::new();

    match form.validate() {
        Ok(post) => {
            // 检验密码是否正确
            let admin = admins_model::get_admin(post.username.clone());
            if admin.is_none() {
                log::warn!("查无用户");
                //查无用户,写入提示
                data.insert("jump_url".to_string(), to_json("/login"));
                data.insert("message".to_string(), to_json("查无用户"));
                let html = to_html_single("hint.html", data);

                let resp = warp::http::Response::builder()
                    .status(warp::http::StatusCode::OK)
                    .body(html);
                return Ok(resp);
            }

            let admin = admin.unwrap();
            // 这里还应判断下status 是否冻结：0=正常，1=永久冻结，冻结时间
            let password = admins_model::encryption(&post.passwd, &admin.salt);
            if password != admin.password {
                // ea1f6a32e3683412c0f75f45f8c10c82b56a5359
                log::warn!("密码{}出错:{} != {}", post.passwd, password, admin.password);
                //写入提示
                data.insert("jump_url".to_string(), to_json("/login"));
                data.insert("message".to_string(), to_json("用户密码错误"));
                let html = to_html_single("hint.html", data);

                let resp = warp::http::Response::builder()
                    .status(warp::http::StatusCode::OK)
                    .body(html);
                return Ok(resp);
            }

            // 正确，写入session,并跳转到默认页
            use crate::common::random_key;
            let cookie = session::Cookie {
                id: random_key(34),
                passwd: admin.password.clone(),
                timestamp: random_key(8), //应该为当前时间戳
                userid: admin.id,
            };

            let session_value = serde_json::json!(cookie);
            let cookie_encode = base64::encode(session_value.to_string().as_bytes());

            //设置cookie时不加Path=/时可能出现有时带不了cookie请求
            let cookie = format!(
                "{}={}; Path=/; SameSite=Lax; HttpOpnly",
                SESSION_NAME, cookie_encode
            );

            data.insert("jump_url".to_string(), to_json("/"));
            data.insert("message".to_string(), to_json("登录成功"));
            let html = to_html_single("hint.html", data);

            let resp = warp::http::Response::builder()
                .status(warp::http::StatusCode::OK)
                .header(warp::http::header::SET_COOKIE, cookie)
                .body(html);
            return Ok(resp);
        }
        Err(msg) => {
            log::error!("{}", msg);

            data.insert("jump_url".to_string(), to_json("/login"));
            data.insert("message".to_string(), to_json(msg));
            let html = to_html_single("hint.html", data);

            let resp = warp::http::Response::builder()
                .status(warp::http::StatusCode::OK)
                .body(html);
            return Ok(resp);
        }
    }
}

// // 处理登录
// pub async fn do_login() -> std::result::Result<impl Reply, Rejection> {
//     log::info!("[调试信息]访问了“/login_post”");
//     // log::warn!("[警告信息] warn");
//     // log::info!("[提示信息] info");

//     println!("post登录校验成功，得到用户信息，写入cookie.并跳转到默认后台管理页");
//     println!("post登录校验失败，跳转到登录页");

//     let html = "session login示例：post处理".to_string();
//     let k = warp::reply::html(html.clone());

//     use crate::common::random_key;
//     let session = session::Session {
//         id: random_key(34),
//         timestamp: random_key(8),
//         data: Some(session::SessionData {
//             user_id: 8,
//             username: "kd".to_string(),
//         }),
//     };
//     println!("密码正确，写入session：{:#?}", session);

//     let resp = warp::http::Response::builder()
//         .status(warp::http::StatusCode::OK)
//         .header(
//             warp::http::header::SET_COOKIE,
//             format!(
//                 "{}={}; SameSite=Strict; HttpOpnly",
//                 SESSION_NAME,
//                 serde_json::json!(session)
//             ),
//         )
//         .body(html);
//     Ok(resp)

//     //处理完post数据，跳转到列表页
//     // let k = warp::redirect::see_other(warp::http::Uri::from_static("/demo/redirect/v"));
//     // Ok(k)

//     // let html = "session login示例".to_string();
//     // Ok(warp::reply::html(html)) //直接返回html
//     // Err(warp::reject::not_found())   //错误的返回
// }
