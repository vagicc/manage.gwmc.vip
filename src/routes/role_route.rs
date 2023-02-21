use crate::handlers::role_handler;
use warp::Filter;

/* 访问站点 / 时的路由 */
pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path!("role" / "index"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(|sesion:crate::session::Session| async {
            role_handler::list(1, sesion).await
        });

    // let test = warp::get()
    //     .and(warp::path("role_test"))
    //     .and(warp::path::end())
    //     .and(crate::session::with_session())
    //     .map(|sesion:crate::session::Session| format!("======luck====="));

    warp::get()
        .and(warp::path("role"))
        .and(warp::path("index"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(role_handler::list)
        .or(first)
        // .or(test)
        .or(add())
        .or(delete())
        .or(edit())
    // .recover(crate::session::no_login) //这个不用了，其它有一处有就行
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("role"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        // .and_then(|_|{})
        .and(crate::session::with_session())
        .and_then(role_handler::do_new);

    use std::collections::HashMap;
    let test_post = warp::post()
        .and(warp::path("role"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::form())
        .map(|simple_map: HashMap<String, String>| {
            println!("post:{:#?}", simple_map);
            "Got a urlencoded body!"
            /*
            post:{
                "id": "",
                "default": "aaaaa",
                "name": "aaaaaa",
                "rights[9]": "9",
                "rights[6]": "6",
                "rights[5]": "5",
            }
            {
                "name": "sad",
                "id": "",
                "default": "asd",
                "rights[]": "5",
            }

            */
        });

    warp::get()
        .and(warp::path!("role" / "create"))
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(role_handler::create_html)
        .or(post)
    // .or(test_post)
}

pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let del_arr = warp::post()
        .and(warp::path("role"))
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(
            |simple_map: std::collections::HashMap<String, String>,
             sesion:crate::session::Session| async move {
                // println!("post:{:#?}", simple_map);
                /*
                post:{
                    "ids": "2",
                    "del_ids": "13,12,11,4,2",
                    "checkbox1": "on",
                } */
                let del = simple_map.get("del_ids");

                let mut ids: Vec<i32> = Vec::new();
                if del.is_some() {
                    ids = del
                        .unwrap()
                        .split(",")
                        .map(|id| id.parse::<i32>().expect("多选删除转ID出错"))
                        .collect();
                }
                role_handler::expurgate(ids, sesion).await
            },
        );

    warp::get()
        .and(warp::path("role"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(role_handler::delete)
        .or(del_arr)
}

pub fn edit() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("role"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and(crate::session::with_session())
        .and_then(role_handler::do_edit);

    warp::get()
        .and(warp::path("role"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(role_handler::edit)
        .or(post)
}
