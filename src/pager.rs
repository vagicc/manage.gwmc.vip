
/// 输出默认分页html
pub fn default_full(path: &str, count: i64, page: u32, per: u32) -> String {
    let base_url =  crate::common::get_env("BASE_URL");
    let page_url = format!("{}{}", base_url, path);

    let count_page = ((count as f32) / (per as f32)).ceil() as u32; //总页数

    /*
    <ul class="pagination pull-right">
        <li><a href="https://www.gust.cn/fortune/record/index/"
                data-ci-pagination-page="1" rel="start">«</a></li>
        <li class="pre-page"><a href="https://www.gust.cn/fortune/record/index/26"
                data-ci-pagination-page="3" rel="prev">上一页</a></li>
        <li><a href="https://www.gust.cn/fortune/record/index/13"
                data-ci-pagination-page="2">2</a></li>
        <li><a href="https://www.gust.cn/fortune/record/index/26"
                data-ci-pagination-page="3">3</a></li>
        <li class="active"><a href="javascript:void(0)" class="paging">4</a></li>
        <li><a href="https://www.gust.cn/fortune/record/index/52"
                data-ci-pagination-page="5">5</a></li>
        <li class="next-page"><a href="https://www.gust.cn/fortune/record/index/52"
                data-ci-pagination-page="5" rel="next">下一页</a></li>
        <li><a href="https://www.gust.cn/fortune/record/index/52"
                                                data-ci-pagination-page="5">»</a></li>
    </ul>
     */
    let mut show_left = 2; //左边显示的数字数
    let mut show_right = 2; //右边显示的数字数
    let mut page_html = String::new();

    //首页
    if page > 2 && page - show_left > 0 {
        page_html = format!(
            r#"
        <li><a href="{}/"
            data-ci-pagination-page="1" rel="start">«</a></li>
            "#,
            page_url
        );
    }

    // 是否有上一页
    if page > 1 {
        // 当前页的上一页,非最前字字上一页
        page_html = format!(
            r#"{}
            <li class="pre-page">
                <a href="{}/{}"
                data-ci-pagination-page="{2}" rel="prev">上一页</a>
            </li>
            "#,
            page_html,
            page_url,
            page - 1
        );

        // 左边的页数数字,如左边少页数,则在右边补
        if page - show_left <= 0 {
            show_right += show_left;
            show_left = page - 1;
            show_right -= show_left;
        }
        while show_left > 0 {
            page_html = format!(
                r#"
            {}
            <li>
                <a href="{}/{}" data-ci-pagination-page="{2}">{2}</a>
            </li>
            "#,
                page_html,
                page_url,
                page - show_left
            );
            show_left -= 1;
        }
    }

    // 当前页
    page_html = format!(
        r#"
    {}
    <li class="active"><a href="javascript:void(0)" class="paging">{}</a></li>
    "#,
        page_html, page
    );

    //是否有下一页
    if page < count_page {
        // 输出右边数字
        let mut t = 1;
        loop {
            if page + show_right > count_page {
                show_right = count_page - page;
            }

            if t > show_right {
                break;
            }

            page_html = format!(
                r#"
            {}
            <li>
                <a href="{}/{}" data-ci-pagination-page="{2}">{2}</a>
            </li>
            "#,
                page_html,
                page_url,
                page + t
            );

            t += 1;
            // show_right -= 1;
            // break;
        }

        // 下一页
        page_html = format!(
            r#"{}
            <li class="next-page">
                <a href="{}/{}"
                    data-ci-pagination-page="{2}" rel="next">下一页</a>
            </li>
            "#,
            page_html,
            page_url,
            page + 1
        );
    }

    //末页,无符号整形的减法会溢出
    // log::warn!("总页数：{}， 右边页数:{}", count_page, show_right);
    // // let k = count_page - show_right;
    // let k = count_page.wrapping_sub(show_right);
    // println!("wrapping_sub:{:?}", k); //总页数：2， 右边页数:3  wrapping_sub:4294967295
    // if page < count_page - show_right {
    if page + show_right < count_page {
        // if page < count_page.wrapping_sub(show_right) {
        page_html = format!(
            r#"{}
            <li>
            <a href="{}/{}"
                        data-ci-pagination-page="{2}">»</a>
            </li>
            "#,
            page_html, page_url, count_page
        );
    }

    page_html
}