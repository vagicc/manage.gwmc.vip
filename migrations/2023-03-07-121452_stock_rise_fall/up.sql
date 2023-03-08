
-- 股票涨跌家数  
CREATE TABLE stock_rise_fall(
    "id" SERIAL PRIMARY KEY,
    "record_date" DATE NOT NULL,
    "week" CHARACTER VARYING(6) DEFAULT NULL,
    "m_rise" INTEGER DEFAULT NULL,
    "m_fall" INTEGER DEFAULT NULL,
    "m_rise_limit" INTEGER DEFAULT NULL,  
    "m_limit_drop" INTEGER DEFAULT NULL,

    "n_rise" INTEGER DEFAULT NULL,
    "n_fall" INTEGER DEFAULT NULL,
    "n_rise_limit" INTEGER DEFAULT NULL,  
    "n_limit_drop" INTEGER DEFAULT NULL,

    "e_rise" INTEGER DEFAULT NULL,
    "e_fall" INTEGER DEFAULT NULL,
    "e_rise_limit" INTEGER DEFAULT NULL,  
    "e_limit_drop" INTEGER DEFAULT NULL,

    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_stock_rise_fall_record_date ON stock_rise_fall (record_date);

COMMENT ON TABLE stock_rise_fall IS '股票涨跌家数表';
COMMENT ON COLUMN stock_rise_fall.id IS '主键ID';
COMMENT ON COLUMN stock_rise_fall.record_date IS '记录日期';
COMMENT ON COLUMN stock_rise_fall.week IS '星期几';

COMMENT ON COLUMN stock_rise_fall.m_rise IS '早盘上涨家数';
COMMENT ON COLUMN stock_rise_fall.m_fall IS '早盘下跌家数';
COMMENT ON COLUMN stock_rise_fall.m_rise_limit IS '早盘涨停数';
COMMENT ON COLUMN stock_rise_fall.m_limit_drop IS '早盘跌停数';

COMMENT ON COLUMN stock_rise_fall.n_rise IS '午盘上涨家数';
COMMENT ON COLUMN stock_rise_fall.n_fall IS '午盘下跌家数';
COMMENT ON COLUMN stock_rise_fall.n_rise_limit IS '午盘涨停数';
COMMENT ON COLUMN stock_rise_fall.n_limit_drop IS '午盘跌停数';

COMMENT ON COLUMN stock_rise_fall.e_rise IS '收盘上涨家数';
COMMENT ON COLUMN stock_rise_fall.e_fall IS '收盘下跌家数';
COMMENT ON COLUMN stock_rise_fall.e_rise_limit IS '收盘涨停数';
COMMENT ON COLUMN stock_rise_fall.e_limit_drop IS '收盘跌停数';

COMMENT ON COLUMN stock_rise_fall.create_time IS '创建时间';
COMMENT ON COLUMN stock_rise_fall.last_time IS '最后修改时间';