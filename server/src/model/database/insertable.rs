use crate::model::database::{
    finalized_schema::{
        comments, moderation_actions, pages, posts, roles, routes, user_actions, users,
    },
    model_enums::{ModerationActionTypes, UserActionTypes},
};
use ipnetwork::IpNetwork;

#[derive(Clone, Debug, Insertable)]
#[table_name = "user_actions"]
pub struct NewUserAction {
    pub actor: i32,
    pub ip: Option<IpNetwork>,
    pub user_action: UserActionTypes,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "moderation_actions"]
pub struct NewModerationAction {
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "routes"]
pub struct NewRoute<'a> {
    pub parent: Option<i32>,
    pub url_slug: &'a str,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "roles"]
pub struct NewRole<'a> {
    pub role_name: &'a str,
    pub role_level: i32,
    pub can_edit_pages: bool,
    pub can_edit_posts: bool,
    pub can_edit_roles: bool,
    pub can_moderate_roles: bool,
    pub can_moderate_comments: bool,
    pub can_embed: bool,
    pub can_comment: bool,
    pub comments_visible: bool,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_role: i32,
    pub username: &'a str,
    pub email: &'a str,
    pub bio: &'a str,
    pub password_hash: &'a str,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub url_route: i32,
    pub author: i32,
    pub title: &'a str,
    pub content: &'a str,
    pub tab_text: &'a str,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "pages"]
pub struct NewPage<'a> {
    pub url_route: i32,
    pub template_location: &'a str,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub commenting_user: i32,
    pub commented_post: i32,
    pub commented_comment: i32,
    pub content: &'a str,
}
