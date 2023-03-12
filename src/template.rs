use handlebars::{to_json, Handlebars};
use serde_json;
use serde_json::value::{Map, Value as Json};

// 后台专用视图展示
pub fn view(name: &str, mut data: Map<String, Json>, session: crate::session::Session) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame_base.html", "src/views/frame_base.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame_base.html")); //必传,这个是插入父级的html
                                                                   // let current = crate::models::menus_model::get_current(session.url_path);
    data.insert(
        "current".to_string(),
        to_json(crate::models::menus_model::get_current(session.url_path)),
    );

    data.insert(
        "left_menus".to_string(),
        to_json(crate::common::get_menus_cache(
            session.admin.role.unwrap_or(1),
        )),
    );
    data.insert("admin".to_string(), to_json(session.admin));

    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    let html = handlebars.render(name, &data).unwrap();
    html
}

/* 基础版嵌入式页 */
pub fn to_html_base(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame_base.html", "src/views/frame_base.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame_base.html")); //必传,这个是插入父级的html
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    let html = handlebars.render(name, &data).unwrap();
    html
}

/* 单面 */
pub fn to_html_single(tpl_name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板文件 */
    handlebars
        .register_template_file(tpl_name, "src/views/".to_owned() + tpl_name)
        .expect("handlebars注册模板出错");

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    let html = handlebars.render(tpl_name, &data).expect("注册模板出错");
    html
}
