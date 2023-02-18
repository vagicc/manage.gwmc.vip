use crate::handlers::login_handler;
// use crate::session;
use warp::Filter;

/// GET:/login and all
pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and_then(login_handler::show_login_html)
        .or(do_login())
        // .or(login_post())
        .or(get_value())
}

pub fn do_login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::path("check"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(login_handler::check_login)
}

// 测试取得头部信息
pub fn get_value() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //取得URL
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };
    let counts = Arc::new(Mutex::new(HashMap::new()));
    // 所有访问网站几次
    let url = warp::get()
        .and(warp::path("login_url"))
        .and(warp::path::full().map(move |path: warp::path::FullPath| {
            let mut counts = counts.lock().unwrap();
            *counts
                .entry(path.as_str().to_string())
                .and_modify(|c| *c += 1)
                .or_insert(0)
        }))
        .map(|count| format!("This is the {}th visit to this URL!", count));

    /// let client_ip = warp::header("x-real-ip")
    ///     .or(warp::header("x-forwarded-for"))
    ///     .unify()
    ///     .map(|ip: SocketAddr| {
    ///         // Get the IP from either header,
    ///         // and unify into the inner type.
    ///     });
    let ip = warp::get()
        .and(warp::path("login_ip"))
        .and(
            //​REMOTE_ADDR HTTP_X_FORWARDED_FOR  原:x-real-ip , x-forwarded-for
            warp::header("x-real-ip")
                .or(warp::header("http-x-forwarded-for"))
                .unify(),
        )
        .map(|ip: std::net::SocketAddr| {
            println!("{:#?}", ip);
            format!("ip: {}", ip)
        });

    let addr = warp::get()
        .and(warp::path("remote_addr"))
        .and(warp::addr::remote())
        .map(|addr: Option<std::net::SocketAddr>| {
            println!("SocketAddr remote address = {:#?}", addr);
            let k=addr.unwrap();
            let ip=k.ip();
            println!("用户的IP = {:#?}", ip);
            println!("用户的IP = {:#?}", ip.to_string());
            println!("IP是否为IPv6: {:#?}", ip.is_ipv6());

            let addr = "10.1.9.32/32".parse::<ipnetwork::IpNetwork>().unwrap();
            //Inet=> ipnetwork  (version = ">=0.12.2, <0.21.0")   ipnetwork = "0.20.0"

            /* 
            SocketAddr remote address = Some(
                [2409:8a56:3218:ffd1:21ac:1dd7:9e64:2985]:45452,
            )
            用户的IP = 2409:8a56:3218:ffd1:21ac:1dd7:9e64:2985
            用户的IP = "2409:8a56:3218:ffd1:21ac:1dd7:9e64:2985"
            IP是否为IPv6: true

            // ip4
            SocketAddr remote address = Some(
                120.231.217.117:5986,
            )
            用户的IP = 120.231.217.117
            用户的IP = "120.231.217.117"
            IP是否为IPv6: false
            */
            "ip:address"
        });

    let log = warp::log("example::api");
    //[2023-02-06 11:08:56 INFO  example::api] 127.0.0.1:43500 "GET /login_ip HTTP/2.0" 200 "-" "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0" 36.223µs
    let log_r = warp::get().map(warp::reply).with(log);

    //取得头信息
    warp::get()
        .and(warp::path("login_h"))
        .and(warp::path::end())
        .and(warp::header::headers_cloned())
        .and(warp::path::full())
        .map(
            |headers: warp::http::HeaderMap, path: warp::path::FullPath| {
                // header count: {
                //     "cache-control": "max-age=0",
                //     "sec-ch-ua": "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"102\", \"Yandex\";v=\"22\"",
                //     "sec-ch-ua-mobile": "?0",
                //     "sec-ch-ua-platform": "\"Linux\"",
                //     "upgrade-insecure-requests": "1",
                //     "user-agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.167 YaBrowser/22.7.5.1036 (beta) Yowser/2.5 Safari/537.36",
                //     "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
                //     "sec-fetch-site": "none",
                //     "sec-fetch-mode": "navigate",
                //     "sec-fetch-user": "?1",
                //     "sec-fetch-dest": "document",
                //     "accept-encoding": "gzip, deflate, br",
                //     "accept-language": "zh,en;q=0.9",
                //     "cookie": "_ga=GA1.1.2097038432.1645540692",
                //     "cookie": "warp-session={\"data\":null,\"id\":\"\",\"timestamp\":\"\"}",
                // }

                format!(
                    "header count: {:#?} <br><hr> fullpath请求的后缀:{:#?}",
                    headers, path
                )
            },
        )
        .or(url)
        .or(ip)
        .or(addr)
    // .map(|| format!("login_h"))
}

//模拟post表单
//设置为已登录session, GET: /login_post
// pub fn login_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::get()
//         .and(warp::path("login_post"))
//         .and(warp::path::end())
//         .and_then(login_handler::do_login)
// }

// GET:/logined
// 正常登录可访问的页面,未登录则自动跳转到登录页
// pub fn logined() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::get()
//         .and(warp::path("logined"))
//         .and(session::with_session())
//         .and(warp::path::end())
//         .map(|session: crate::models::admins_model::Admins| format!("用户{:#?}已登录!!!!", session))
//         .recover(session::no_login)
// }
