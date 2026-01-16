-- 会话表 start
CREATE TABLE "ci_sessions" (
    "id" varchar(128) NOT NULL PRIMARY KEY,
    "ip_address" inet NOT NULL,
    "timestamp" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "data" bytea DEFAULT '' NOT NULL
);

CREATE INDEX "ci_sessions_timestamp" ON "ci_sessions" ("timestamp");
CREATE INDEX "ci_sessions_id_ip_address" ON "ci_sessions" (id, ip_address);
-- 当 sessionMatchIP = TRUE 时
-- ALTER TABLE ci_sessions ADD PRIMARY KEY (id, ip_address);
-- 当 sessionMatchIP = FALSE 时
-- ALTER TABLE ci_sessions ADD PRIMARY KEY (id);
-- 删除先前创建的主键（在更改设置时使用）
-- ALTER TABLE ci_sessions DROP PRIMARY KEY;

-- 会话表 end

-- 后台管理用户表 start
CREATE TABLE admins (
    id serial primary key,
    username CHARACTER VARYING(16) NOT NULL,
    "password" CHARACTER VARYING(40) NOT NULL,
    salt CHARACTER(10) NOT NULL,
    "email" CHARACTER VARYING(100) DEFAULT NULL,
    "mobile" CHARACTER(11) DEFAULT NULL,
    "role" integer DEFAULT 0,
    "status" bigint DEFAULT 0,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    --不带时区
    -- create_time TIMESTAMP(6) WITH TIME ZONE DEFAULT clock_timestamp(), --带时区
    "last_login" TIMESTAMP WITHOUT time ZONE DEFAULT NULL
);

CREATE INDEX idx_admins_username ON admins (username);

CREATE INDEX idx_admins_email ON admins (email);

CREATE INDEX idx_admins_role ON admins (role);

COMMENT ON TABLE admins IS '后台管理用户表';

COMMENT ON COLUMN admins.id IS '自增主键ID';

COMMENT ON COLUMN admins.username IS '登录名';

COMMENT ON COLUMN admins.password IS '登录密码';

COMMENT ON COLUMN admins.salt IS '混淆码';

COMMENT ON COLUMN admins.email IS '邮箱';

COMMENT ON COLUMN admins.mobile IS '电话';

COMMENT ON COLUMN admins.role IS '角色组ID';

COMMENT ON COLUMN admins.status IS '是否冻结：0=正常，1=永久冻结，冻结时间';

COMMENT ON COLUMN admins.create_time IS '创建时间(不带时区)';

COMMENT ON COLUMN admins.last_login IS '最后登录时间(不带时区)';

INSERT INTO
    admins (
        "id",
        "username",
        "password",
        "salt",
        "email",
        "mobile",
        "role",
        "status",
        "create_time",
        "last_login"
    )
VALUES
    (
        1,
        'luck',
        'ea1f6a32e3683412c0f75f45f8c10c82b56a5359',
        '25ee364a54',
        'luck@xust.net',
        '18514791358',
        1,
        0,
        '2008-08-18 18:58:13',
        '2018-08-18 18:58:18'
    );
SELECT pg_catalog.setval('public.admins_id_seq', 1, true);
-- 后台管理用户表 end

-- 后台角色表 start
CREATE TABLE roles (
    id serial primary key,
    name CHARACTER VARYING(20) NOT NULL,
    "rights" integer ARRAY,
    "default" CHARACTER VARYING(50) DEFAULT NULL
);

-- 添加name索引
CREATE INDEX idx_roles_name ON roles (name);

COMMENT ON TABLE roles IS '后台角色表';
COMMENT ON COLUMN roles.id IS '自增主键';
COMMENT ON COLUMN roles.name IS '角色组名称';
COMMENT ON COLUMN roles.rights IS '角色组权限';
COMMENT ON COLUMN roles.default IS '角色组默认登录页';

INSERT INTO
    roles (id, name, rights, "default")
VALUES
    (1, 'root=>超级管理组', '{}', 'luck/index');
SELECT pg_catalog.setval('public.roles_id_seq', 1, true);

-- 后台角色表 end

-- 菜单表 start
CREATE TABLE menus (
    id serial primary key,
    order_by smallint NOT NULL,
    "path_full" CHARACTER VARYING(255) DEFAULT NULL,
    name character varying(20) NOT NULL,
    level smallint DEFAULT 0,
    parent integer DEFAULT 0, 
    icon CHARACTER VARYING(50) DEFAULT NULL,
    department integer DEFAULT NULL,
    is_show boolean NOT NULL DEFAULT TRUE
);
CREATE INDEX idx_menus_parent ON menus (parent);
CREATE INDEX idx_menus_path_full ON menus (path_full);
CREATE INDEX idx_menus_level ON menus (level);

COMMENT ON TABLE menus IS '后台菜单表';
comment on column menus.id is 'ID自增主键';
comment on column menus.order_by is '排序';
comment on column menus.path_full is 'url全路径';
comment on column menus.name is '菜单名字';
COMMENT ON COLUMN menus.level IS '菜单层级';
COMMENT ON COLUMN menus.parent IS '菜单父级';
COMMENT ON COLUMN menus.icon IS '菜单左侧小图标';
COMMENT ON COLUMN menus.department IS '菜单所属顶级';
COMMENT ON COLUMN menus.is_show IS '菜单是否显示：默认true显示，false不显示';

INSERT INTO public.menus VALUES (3, 1, 'menus/index', '菜单管理', 2, 2, '', 2, true);
INSERT INTO public.menus VALUES (2, 8, '', '系统设置', 1, 0, 'fa-cogs', 0, true);
INSERT INTO public.menus VALUES (4, 2, 'admins/index', '后台用户', 2, 2, '', 2, true);
INSERT INTO public.menus VALUES (1, 1, '', '后台首页', 1, 0, 'fa-home', 0, true);
INSERT INTO public.menus VALUES (5, 3, 'role/index', '角色管理', 2, 2, '', 2, true);
INSERT INTO public.menus VALUES (6, 5, 'record/index', '操作日志', 2, 2, '', 2, true);
INSERT INTO public.menus VALUES (7, 4, 'rights/index', '权限列表', 2, 2, '', 2, true);
SELECT pg_catalog.setval('public.menus_id_seq', 7, true);
-- 菜单表 end

-- 权限表 start
CREATE TABLE rights (
    "right_id" SERIAL PRIMARY KEY,
    "right_name" CHARACTER VARYING(30) DEFAULT NULL,
    "path_full" CHARACTER VARYING(255) NOT NULL,
    "right_detail" CHARACTER VARYING(30) DEFAULT NULL
);
CREATE INDEX idx_rights_path_full ON rights (path_full);
COMMENT ON TABLE rights IS '权限表';
comment on column rights.right_name is '权限名称,暂无使用';
comment on column rights.path_full is '权限全路径';
comment on column rights.right_detail is '权限详细说明,暂无使用';
-- 权限表 end

-- 日志表 start
-- 内置分区表 (record) ,每月要新添加分区表。（或者每年也是行的。）
CREATE TABLE record(
    "id" SERIAL,
    "table_id" integer NOT NULL,
    "table_name" CHARACTER VARYING(180) NOT NULL,
    "user_id" integer NOT NULL,
    "username" CHARACTER VARYING(18) NOT NULL,
    "action" CHARACTER VARYING(180) NOT NULL,
    "ip" inet NOT NULL,
    "record_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp() PRIMARY KEY
) PARTITION BY RANGE(record_time);

COMMENT ON TABLE record IS '操作记录表-采用了内置分区表，按月分表';

COMMENT ON COLUMN record.table_id IS '操作表ID';

COMMENT ON COLUMN record.table_name IS '操作表名';

COMMENT ON COLUMN record.user_id IS '操作用户ID';

COMMENT ON COLUMN record.username IS '操作用户名';

COMMENT ON COLUMN record.action IS '操作动作';

COMMENT ON COLUMN record.ip IS '操作IP';

COMMENT ON COLUMN record.record_time IS '操作时间';
 
-- 日志表 end
