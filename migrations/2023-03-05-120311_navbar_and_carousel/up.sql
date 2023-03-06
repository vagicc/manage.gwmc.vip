-- 导航栏
CREATE TABLE navbar(
    "id" SERIAL PRIMARY KEY,
    "menu" CHARACTER VARYING(18) NOT NULL,
    "link" CHARACTER VARYING(255) NOT NULL,
    "show"  BOOLEAN DEFAULT TRUE,
    "sort_order" SMALLINT NOT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_navbar_show ON navbar (show);
CREATE INDEX idx_navbar_sort_order ON navbar (sort_order);

COMMENT ON TABLE navbar IS '导航栏表';
COMMENT ON COLUMN navbar.id IS '主键ID';
COMMENT ON COLUMN navbar.menu IS '导航栏按纽';
COMMENT ON COLUMN navbar.link IS '导航栏链接';
COMMENT ON COLUMN navbar.show IS '是否显示';
COMMENT ON COLUMN navbar.sort_order IS '排序：小前大后';
COMMENT ON COLUMN navbar.last_time IS '最后修改时间';

-- 首页轮播
CREATE TABLE carousel(
    "id" SERIAL PRIMARY KEY,
    "subhead" CHARACTER VARYING(38) DEFAULT NULL,
    "title" CHARACTER VARYING(189) NOT NULL,
    "summary" CHARACTER VARYING(255) DEFAULT NULL,
    "link" CHARACTER VARYING(255) NOT NULL,
    "path" CHARACTER VARYING(255) DEFAULT NULL,
    "show"  BOOLEAN DEFAULT TRUE,
    "sort_order" SMALLINT NOT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_carousel_title ON carousel (title);
CREATE INDEX idx_carousel_show ON carousel (show);
CREATE INDEX idx_carousel_sort_order ON carousel (sort_order);

COMMENT ON TABLE carousel IS '首页焦点图表';
COMMENT ON COLUMN carousel.id IS '主键ID';
COMMENT ON COLUMN carousel.subhead IS '短标题';
COMMENT ON COLUMN carousel.title IS '标题';
COMMENT ON COLUMN carousel.summary IS '摘要';
COMMENT ON COLUMN carousel.link IS '指向链接';
COMMENT ON COLUMN carousel.path IS '图片URL';
COMMENT ON COLUMN carousel.show IS '是否显示';
COMMENT ON COLUMN carousel.sort_order IS '排序：小前大后';
COMMENT ON COLUMN carousel.last_time IS '最后修改时间';