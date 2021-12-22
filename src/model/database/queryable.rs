use crate::model::database::{
    finalized_schema::{
        comments, moderation_actions, pages, posts, roles, routes, user_actions, users,
    },
    model_enums::{ModerationActionTypes, UserActionTypes},
};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
pub struct UserAction {
    pub id: i32,
    pub actor: i32,
    pub ip: String,
    pub user_action: UserActionTypes,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct ModerationAction {
    pub id: i32,
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct Route {
    pub id: i32,
    pub parent: i32,
    pub url_slug: String,
}

#[derive(Queryable, Identifiable)]
pub struct Role {
    pub id: i32,
    pub role_name: String,
    pub role_level: i32,
    pub can_edit_pages: bool,
    pub can_edit_posts: bool,
    pub can_edit_roles: bool,
    pub can_moderate_roles: bool,
    pub can_moderate_comments: bool,
    pub can_embed: bool,
    pub can_comment: bool,
    pub comments_visible: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub user_role: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct Post {
    pub id: i32,
    pub url_route: i32,
    pub author: i32,
    pub title: String,
    pub content: String,
    pub tab_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct Page {
    pub id: i32,
    pub url_route: i32,
    pub template_location: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
pub struct Comment {
    pub id: i32,
    pub commenting_user: i32,
    pub commented_post: i32,
    pub commented_comment: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}
