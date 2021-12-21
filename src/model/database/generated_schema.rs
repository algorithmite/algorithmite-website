table! {
    comments (id) {
        id -> Int4,
        commenting_user -> Int4,
        commented_post -> Int4,
        commented_comment -> Nullable<Int4>,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    moderation_actions (id) {
        id -> Int4,
        moderator -> Int4,
        actor -> Int4,
        pre_action_role -> Int4,
        moderation_action -> Moderation_action_types,
        created_at -> Timestamp,
    }
}

table! {
    pages (id) {
        id -> Int4,
        url_route -> Int4,
        template_location -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        url_route -> Int4,
        author -> Int4,
        title -> Nullable<Text>,
        content -> Nullable<Text>,
        tab_text -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        role_name -> Varchar,
        role_level -> Int4,
        can_edit_pages -> Bool,
        can_edit_posts -> Bool,
        can_edit_roles -> Bool,
        can_moderate_roles -> Bool,
        can_moderate_comments -> Bool,
        can_embed -> Bool,
        can_comment -> Bool,
        comments_visible -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    routes (id) {
        id -> Int4,
        parent -> Nullable<Int4>,
        url_slug -> Nullable<Text>,
    }
}

table! {
    user_actions (id) {
        id -> Int4,
        actor -> Int4,
        ip -> Nullable<Inet>,
        user_action -> User_action_types,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_role -> Int4,
        username -> Varchar,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(comments -> posts (commented_post));
joinable!(comments -> users (commenting_user));
joinable!(moderation_actions -> roles (pre_action_role));
joinable!(pages -> routes (url_route));
joinable!(posts -> routes (url_route));
joinable!(posts -> users (author));
joinable!(user_actions -> users (actor));
joinable!(users -> roles (user_role));

allow_tables_to_appear_in_same_query!(
    comments,
    moderation_actions,
    pages,
    posts,
    roles,
    routes,
    user_actions,
    users,
);
