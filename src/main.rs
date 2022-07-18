use crate::common::get_env;
use std::net::SocketAddr;

mod common;
mod db;
mod filters;
mod format_logger;
mod handlers;
mod models;
mod routes;
mod schema;
mod template;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    log::info!("跟我买车:manage.gwmc.vip!");

    // env_logger::init();
    let log_level = crate::format_logger::get_log_level();
    // 自定义日志输出格式
    env_logger::Builder::new()
        .format(crate::format_logger::formatlog)
        .filter(None, log_level)
        .target(env_logger::Target::Stdout) //添加这行可以重定向日志
        .init();

    // log::info!("such information");
    // log::warn!("o_O");
    // log::error!("much error");
    // log::debug!("高度");

    let routes = filters::all_routes();

    //取得https证书等
    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    let ip_addr = get_env("ip_address");
    let socket_addr: SocketAddr = ip_addr.as_str().parse().unwrap();

    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(socket_addr)
        .await;
}
