use crate::common::get_env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

mod common;
mod constants;
mod db;
mod filters;
mod format_logger;
mod handlers;
mod http;
mod json_value;
mod models;
mod pager;
mod parse;
mod routes;
mod schema;
mod session;
mod template;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    log::info!("跟我买车:manage.gwmc.vip!");
    // // 最基础warp
    // let hello = warp::path!("hello" / String).map(|name| format!("你好，{}！欢迎来到warp", name));
    // warp::serve(hello).run(([127, 0, 0, 1], 1358)).await;

    // env_logger::init();
    let log_level = crate::format_logger::get_log_level();
    // 自定义日志输出格式
    env_logger::Builder::new()
        .format(crate::format_logger::formatlog)
        .filter(None, log_level)
        .target(env_logger::Target::Stdout) //添加这行可以重定向日志
        .init();

    log::info!("info日志");
    log::warn!("警告日志");
    log::error!("错误日志");
    log::debug!("调试日志");
    log::trace!("trace跟踪日志");
    log::info!("warp框架web站点：{}", get_env("BASE_URL"));

    let routes = filters::all_routes();

    //取得https证书等
    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    // let ip_addr = get_env("ip_address");
    // let socket_addr: SocketAddr = ip_addr.as_str().parse().unwrap();

    let port = get_env("PORT").parse::<u16>().expect("端口字符转整形出错");
    let is_ipv6 = get_env("IS_IPV6")
        .parse::<bool>()
        .expect("监听IPv6或监听IPv4=>BOOL出错");

    if is_ipv6 {
        warp::serve(routes)
            // .tls()   //https
            // .cert_path(cert_path) //https
            // .key_path(key_path) //https
            // .run(socket_addr)
            .run(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                port,
            ))
            .await;
    } else {
        warp::serve(routes)
            // .tls()   //https
            // .cert_path(cert_path) //https
            // .key_path(key_path) //https
            // .run(socket_addr)
            .run(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
            .await;
    }
}
