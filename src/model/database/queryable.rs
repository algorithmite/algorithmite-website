use crate::model::database::{
    finalized_schema::{
        comments, moderation_actions, pages, posts, roles, routes, user_actions, users,
    },
    model_enums::{ModerationActionTypes, UserActionTypes},
};
use chrono::NaiveDateTime;
use ipnetwork::IpNetwork;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct UserAction {
    pub id: i32,
    pub actor: i32,
    pub ip: Option<IpNetwork>,
    pub user_action: UserActionTypes,
    pub created_at: NaiveDateTime,
}
impl Ord for UserAction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for UserAction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct ModerationAction {
    pub id: i32,
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
    pub created_at: NaiveDateTime,
}
impl Ord for ModerationAction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for ModerationAction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Route {
    pub id: i32,
    pub parent: Option<i32>,
    pub url_slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
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
impl Ord for Role {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct User {
    pub id: i32,
    pub user_role: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Post {
    pub id: i32,
    pub url_route: i32,
    pub author: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tab_text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Page {
    pub id: i32,
    pub url_route: i32,
    pub template_location: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Comment {
    pub id: i32,
    pub commenting_user: i32,
    pub commented_post: i32,
    pub commented_comment: Option<i32>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
impl Ord for Comment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
