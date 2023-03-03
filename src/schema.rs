// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Bpchar,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Bpchar>,
        role -> Nullable<Int4>,
        status -> Nullable<Int8>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ci_sessions (id) {
        id -> Varchar,
        ip_address -> Inet,
        timestamp -> Timestamptz,
        data -> Bytea,
    }
}

diesel::table! {
    demo (id) {
        id -> Int4,
        name -> Varchar,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lawsuit_autocar (id) {
        id -> Int4,
        title -> Varchar,
        summary -> Varchar,
        list_img -> Nullable<Varchar>,
        license -> Nullable<Varchar>,
        violating -> Nullable<Varchar>,
        universal_model -> Nullable<Varchar>,
        gearbox -> Nullable<Varchar>,
        fuel_type -> Nullable<Varchar>,
        kilometer -> Nullable<Int4>,
        registration -> Nullable<Date>,
        production_date -> Nullable<Date>,
        autocar_model -> Nullable<Varchar>,
        vim -> Nullable<Varchar>,
        engine_number -> Nullable<Varchar>,
        emission -> Nullable<Varchar>,
        price_base -> Money,
        current_price -> Money,
        assess_price -> Money,
        margin -> Money,
        recommended_price -> Money,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        recommend -> Int2,
        address -> Nullable<Varchar>,
        disposal_unit -> Nullable<Varchar>,
        external_url -> Nullable<Varchar>,
        belong -> Nullable<Int2>,
        stage -> Nullable<Varchar>,
        status -> Int2,
        show -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lawsuit_autocar_article (laid) {
        laid -> Int4,
        article_content -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lawsuit_autocar_category (acid) {
        acid -> Int4,
        cname -> Nullable<Varchar>,
        parent_id -> Int4,
        level -> Nullable<Int2>,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        order_by -> Int2,
        is_show -> Nullable<Bool>,
    }
}

diesel::table! {
    lawsuit_autocar_photo (lapid) {
        lapid -> Int4,
        laid -> Int4,
        external_small -> Nullable<Varchar>,
        external_middle -> Nullable<Varchar>,
        external_original -> Nullable<Varchar>,
        small -> Nullable<Varchar>,
        middle -> Nullable<Varchar>,
        original -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        front_cover -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lawsuit_reptile (id) {
        id -> Int4,
        title -> Varchar,
        list_img -> Nullable<Varchar>,
        price_base -> Money,
        current_price -> Money,
        assess_price -> Money,
        margin -> Money,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        lng -> Nullable<Numeric>,
        lat -> Nullable<Numeric>,
        address -> Nullable<Varchar>,
        disposal_unit -> Nullable<Varchar>,
        external_url -> Nullable<Varchar>,
        belong -> Nullable<Int2>,
        stage -> Nullable<Varchar>,
        status -> Int2,
        push -> Nullable<Bool>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lawsuit_reptile_photo (lrpid) {
        lrpid -> Int4,
        lrid -> Int4,
        external_small -> Nullable<Varchar>,
        external_middle -> Nullable<Varchar>,
        external_original -> Nullable<Varchar>,
        front_cover -> Nullable<Bool>,
    }
}

diesel::table! {
    menus (id) {
        id -> Int4,
        order_by -> Int2,
        path_full -> Nullable<Varchar>,
        name -> Varchar,
        level -> Nullable<Int2>,
        parent -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
        department -> Nullable<Int4>,
        is_show -> Bool,
    }
}

diesel::table! {
    oauth_access_tokens (access_token) {
        access_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

diesel::table! {
    oauth_authorization_codes (authorization_code) {
        authorization_code -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        redirect_uri -> Nullable<Varchar>,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
        id_token -> Nullable<Varchar>,
    }
}

diesel::table! {
    oauth_clients (client_id) {
        client_id -> Bpchar,
        client_secret -> Nullable<Varchar>,
        redirect_uri -> Nullable<Varchar>,
        grant_types -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    oauth_jwt (client_id) {
        client_id -> Bpchar,
        subject -> Nullable<Varchar>,
        public_key -> Nullable<Varchar>,
    }
}

diesel::table! {
    oauth_refresh_tokens (refresh_token) {
        refresh_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

diesel::table! {
    oauth_scopes (scope) {
        scope -> Bpchar,
        is_default -> Nullable<Int2>,
    }
}

diesel::table! {
    oauth_users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Nullable<Bpchar>,
        scope -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    record (record_time) {
        id -> Int4,
        table_id -> Int4,
        table_name -> Varchar,
        user_id -> Int4,
        username -> Varchar,
        action -> Varchar,
        ip -> Inet,
        record_time -> Timestamp,
    }
}

diesel::table! {
    rights (right_id) {
        right_id -> Int4,
        right_name -> Nullable<Varchar>,
        path_full -> Varchar,
        right_detail -> Nullable<Varchar>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        rights -> Nullable<Array<Nullable<Int4>>>,
        default -> Nullable<Varchar>,
    }
}

diesel::table! {
    site_introduction (id) {
        id -> Int4,
        title -> Varchar,
        seo_title -> Nullable<Varchar>,
        seo_keywords -> Nullable<Varchar>,
        seo_description -> Nullable<Varchar>,
        content -> Nullable<Text>,
        last_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    ci_sessions,
    demo,
    lawsuit_autocar,
    lawsuit_autocar_article,
    lawsuit_autocar_category,
    lawsuit_autocar_photo,
    lawsuit_reptile,
    lawsuit_reptile_photo,
    menus,
    oauth_access_tokens,
    oauth_authorization_codes,
    oauth_clients,
    oauth_jwt,
    oauth_refresh_tokens,
    oauth_scopes,
    oauth_users,
    record,
    rights,
    roles,
    site_introduction,
);
