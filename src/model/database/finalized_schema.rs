table! {
    comments (comment_key) {
        comment_key -> Int4,
        commenting_user -> Int4,
        commented_post -> Int4,
        commented_comment -> Nullable<Int4>,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

// Add imports and proper enum type
table! {
    use crate::model::database::model_enums::ModerationActionTypesMapping;
    use diesel::sql_types::{Int4, Timestamp};
    moderation_actions (moderation_action_key) {
        moderation_action_key -> Int4,
        moderator -> Int4,
        actor -> Int4,
        pre_action_role -> Int4,
        moderation_action -> ModerationActionTypesMapping,
        created_at -> Timestamp,
    }
}

table! {
    pages (page_key) {
        page_key -> Int4,
        url_route -> Int4,
        template_location -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    posts (post_key) {
        post_key -> Int4,
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
    roles (role_key) {
        role_key -> Int4,
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
    routes (route_key) {
        route_key -> Int4,
        parent -> Nullable<Int4>,
        url_slug -> Nullable<Text>,
    }
}

// Add imports and proper enum type
table! {
    use crate::model::database::model_enums::UserActionTypesMapping;
    use diesel::sql_types::{Int4, Inet, Timestamp, Nullable};
    user_actions (user_action_key) {
        user_action_key -> Int4,
        actor -> Int4,
        ip -> Nullable<Inet>,
        user_action -> UserActionTypesMapping,
        created_at -> Timestamp,
    }
}

table! {
    users (user_key) {
        user_key -> Int4,
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
