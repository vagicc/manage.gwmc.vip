use crate::multipart_form::{get_input, upload_file};
use crate::session::Session;
use crate::template::view;
use futures_util::TryStreamExt;
use handlebars::{to_json, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::value::Map;
use std::collections::HashMap;
use warp::multipart::{FormData, Part};
use warp::{Rejection, Reply};

// 输出要修改的html
pub async fn upload_html(session: Session) -> Result<impl Reply, Rejection> {
    log::info!("输出修改推荐");

    let mut data = Map::new();

    let html = view("summernote.html", data, session);
    Ok(warp::reply::html(html))
}

pub async fn summernote(form: FormData, _session: Session) -> Result<impl Reply, Rejection> {
    // println!("上传的表单：{:#?}", post);
    // if post.is_empty() {
    //     //408请求超时
    //     return crate::common::response_json(
    //         warp::http::StatusCode::REQUEST_TIMEOUT,
    //         None,
    //         Some("408请求超时".to_owned()),
    //     );
    // }

    /* 处理文件上传表单（method="post" enctype="multipart/form-data"） */
    let mut parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("上传文件表单出错form error: {}", e);
        warp::reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    if parts.is_empty() {
        return crate::common::response_json(
            warp::http::StatusCode::REQUEST_TIMEOUT,
            None,
            Some("没有选择文件上传".to_owned()),
        );
    }

    let part = parts.pop().unwrap(); // let mut part=parts[0];
    let (name, filename) = upload_file(part).await;
    if name != "file" || filename.is_empty() {
        //408请求超时
        return crate::common::response_json(
            warp::http::StatusCode::REQUEST_TIMEOUT,
            None,
            Some("408请求超时".to_owned()),
        );
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct ImageUpload {
        image: String,
    }

    let upload_return = ImageUpload {
        image: format!("{}{}", crate::common::get_env("IMAGE_URL"), filename),
    };
    // upload_return.image = filename; //返回上传后的图片 ，这里是相对路径
    //{"status":200,"message":null,"data":{"image":"static/uploads/20232/2023225165513g3cEp.jpg"}}
    return crate::common::response_json(
        warp::http::StatusCode::OK,
        Some(&upload_return),
        Some("图片上传成功".to_owned()),
    );
}

#[derive(Debug)]
struct ServerError {
    message: String,
}
impl warp::reject::Reject for ServerError {}

// 输出html
pub async fn demo_html(session: Session) -> Result<impl Reply, Rejection> {
    log::info!("输出修改推荐");

    let mut data = Map::new();

    let html = view("upload_demo.html", data, session);
    Ok(warp::reply::html(html))
}

pub async fn upload_demo(form: FormData, session: Session) -> Result<impl Reply, Rejection> {
    /* 处理文件上传表单（method="post" enctype="multipart/form-data"） */
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("上传文件表单出错form error: {}", e);
        warp::reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    let mut post: HashMap<String, String> = HashMap::new();

    for mut part in parts {
        let input_name = part.name();
        if input_name == "image" {
            log::debug!("文件上传处理");
            let (name, filename) = upload_file(part).await;
            // println!("表单{}上传的文件{}", name, filename);
            post.insert(name, filename);
        } else {
            log::debug!("普通表单处理");
            let (name, value) = get_input(&mut part).await;
            // println!("表单{} => {}", name, value);
            post.insert(name, value);
        }
    }

    println!("上传的表单：{:#?}", post);

    Ok("文件上传成功!")
}
