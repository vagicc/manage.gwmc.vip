-- 站点介绍类统一文章表 
CREATE TABLE site_introduction(
    "id" SERIAL PRIMARY KEY,
    "title" CHARACTER VARYING(180) NOT NULL,
  
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(255) DEFAULT NULL,

    "content" TEXT,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_site_introduction_title ON site_introduction (title);

COMMENT ON TABLE site_introduction IS '站点介绍类统一文章表';
COMMENT ON COLUMN site_introduction.id IS '主键ID';

COMMENT ON COLUMN site_introduction.title IS '标题';

COMMENT ON COLUMN site_introduction.seo_title IS 'SEO标题';
COMMENT ON COLUMN site_introduction.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN site_introduction.seo_description IS 'SEO描述';

COMMENT ON COLUMN site_introduction.content IS '内容';
COMMENT ON COLUMN site_introduction.last_time IS '最后修改时间';