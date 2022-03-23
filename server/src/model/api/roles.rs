use crate::model::database::{
    finalized_schema::roles::dsl::*, insertable::NewRole, queryable::Role,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

//Create

pub fn create_role(
    conn: &PgConnection,
    input_role_name: String,
    input_role_level: i32,
    input_can_edit_pages: bool,
    input_can_edit_posts: bool,
    input_can_edit_roles: bool,
    input_can_moderate_roles: bool,
    input_can_moderate_comments: bool,
    input_can_embed: bool,
    input_can_comment: bool,
    input_comments_visible: bool,
) -> QueryResult<Role> {
    let new_role: NewRole = NewRole {
        role_name: &input_role_name,
        role_level: input_role_level,
        can_edit_pages: input_can_edit_pages,
        can_edit_posts: input_can_edit_posts,
        can_edit_roles: input_can_edit_roles,
        can_moderate_roles: input_can_moderate_roles,
        can_moderate_comments: input_can_moderate_comments,
        can_embed: input_can_embed,
        can_comment: input_can_comment,
        comments_visible: input_comments_visible,
    };
    insert_into(roles).values(&new_role).get_result(conn)
}

//Read

pub fn read_role(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_role_name: Option<String>,
    option_role_level: Option<i32>,
    option_can_edit_pages: Option<bool>,
    option_can_edit_posts: Option<bool>,
    option_can_edit_roles: Option<bool>,
    option_can_moderate_roles: Option<bool>,
    option_can_moderate_comments: Option<bool>,
    option_can_embed: Option<bool>,
    option_can_comment: Option<bool>,
    option_comments_visible: Option<bool>,
) -> QueryResult<Role> {
    let mut query = roles.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_role_name {
        Some(query_role_name) => query = query.filter(role_name.eq(query_role_name)),
        _ => (),
    }
    match option_role_level {
        Some(query_role_level) => query = query.filter(role_level.eq(query_role_level)),
        _ => (),
    }
    match option_can_edit_pages {
        Some(query_can_edit_pages) => query = query.filter(can_edit_pages.eq(query_can_edit_pages)),
        _ => (),
    }
    match option_can_edit_posts {
        Some(query_can_edit_posts) => query = query.filter(can_edit_posts.eq(query_can_edit_posts)),
        _ => (),
    }
    match option_can_edit_roles {
        Some(query_can_edit_roles) => query = query.filter(can_edit_roles.eq(query_can_edit_roles)),
        _ => (),
    }
    match option_can_moderate_roles {
        Some(query_can_moderate_roles) => {
            query = query.filter(can_moderate_roles.eq(query_can_moderate_roles))
        }
        _ => (),
    }
    match option_can_moderate_comments {
        Some(query_can_moderate_comments) => {
            query = query.filter(can_moderate_comments.eq(query_can_moderate_comments))
        }
        _ => (),
    }
    match option_can_embed {
        Some(query_can_embed) => query = query.filter(can_embed.eq(query_can_embed)),
        _ => (),
    }
    match option_can_comment {
        Some(query_can_comment) => query = query.filter(can_comment.eq(query_can_comment)),
        _ => (),
    }
    match option_comments_visible {
        Some(query_comments_visible) => {
            query = query.filter(comments_visible.eq(query_comments_visible))
        }
        _ => (),
    }
    query.first(conn)
}

pub fn read_roles(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_role_name: Option<String>,
    option_role_level: Option<i32>,
    option_can_edit_pages: Option<bool>,
    option_can_edit_posts: Option<bool>,
    option_can_edit_roles: Option<bool>,
    option_can_moderate_roles: Option<bool>,
    option_can_moderate_comments: Option<bool>,
    option_can_embed: Option<bool>,
    option_can_comment: Option<bool>,
    option_comments_visible: Option<bool>,
) -> QueryResult<Vec<Role>> {
    let mut query = roles.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_role_name {
        Some(query_role_name) => query = query.filter(role_name.eq(query_role_name)),
        _ => (),
    }
    match option_role_level {
        Some(query_role_level) => query = query.filter(role_level.eq(query_role_level)),
        _ => (),
    }
    match option_can_edit_pages {
        Some(query_can_edit_pages) => query = query.filter(can_edit_pages.eq(query_can_edit_pages)),
        _ => (),
    }
    match option_can_edit_posts {
        Some(query_can_edit_posts) => query = query.filter(can_edit_posts.eq(query_can_edit_posts)),
        _ => (),
    }
    match option_can_edit_roles {
        Some(query_can_edit_roles) => query = query.filter(can_edit_roles.eq(query_can_edit_roles)),
        _ => (),
    }
    match option_can_moderate_roles {
        Some(query_can_moderate_roles) => {
            query = query.filter(can_moderate_roles.eq(query_can_moderate_roles))
        }
        _ => (),
    }
    match option_can_moderate_comments {
        Some(query_can_moderate_comments) => {
            query = query.filter(can_moderate_comments.eq(query_can_moderate_comments))
        }
        _ => (),
    }
    match option_can_embed {
        Some(query_can_embed) => query = query.filter(can_embed.eq(query_can_embed)),
        _ => (),
    }
    match option_can_comment {
        Some(query_can_comment) => query = query.filter(can_comment.eq(query_can_comment)),
        _ => (),
    }
    match option_comments_visible {
        Some(query_comments_visible) => {
            query = query.filter(comments_visible.eq(query_comments_visible))
        }
        _ => (),
    }
    query.get_results(conn)
}

pub fn read_role_id(conn: &PgConnection, query_id: i32) -> QueryResult<Role> {
    read_role(
        conn,
        Some(query_id),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn read_role_name(conn: &PgConnection, query_role_name: String) -> QueryResult<Role> {
    read_role(
        conn,
        None,
        Some(query_role_name),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

//Update

pub fn update_role(
    conn: &PgConnection,
    role_id: i32,
    option_role_name: Option<String>,
    option_role_level: Option<i32>,
    option_can_edit_pages: Option<bool>,
    option_can_edit_posts: Option<bool>,
    option_can_edit_roles: Option<bool>,
    option_can_moderate_roles: Option<bool>,
    option_can_moderate_comments: Option<bool>,
    option_can_embed: Option<bool>,
    option_can_comment: Option<bool>,
    option_comments_visible: Option<bool>,
) -> Option<QueryResult<usize>> {
    match read_role_id(conn, role_id) {
        Ok(query_role) => {
            let mut final_role_name: String = query_role.role_name.clone();
            let mut final_role_level: i32 = query_role.role_level;
            let mut final_can_edit_pages: bool = query_role.can_edit_pages;
            let mut final_can_edit_posts: bool = query_role.can_edit_posts;
            let mut final_can_edit_roles: bool = query_role.can_edit_roles;
            let mut final_can_moderate_roles: bool = query_role.can_moderate_roles;
            let mut final_can_moderate_comments: bool = query_role.can_moderate_comments;
            let mut final_can_embed: bool = query_role.can_embed;
            let mut final_can_comment: bool = query_role.can_comment;
            let mut final_comments_visible: bool = query_role.comments_visible;

            match option_role_name {
                Some(query_role_name) => final_role_name = query_role_name,
                _ => (),
            }
            match option_role_level {
                Some(query_role_level) => final_role_level = query_role_level,
                _ => (),
            }
            match option_can_edit_pages {
                Some(query_can_edit_pages) => final_can_edit_pages = query_can_edit_pages,
                _ => (),
            }
            match option_can_edit_posts {
                Some(query_can_edit_posts) => final_can_edit_posts = query_can_edit_posts,
                _ => (),
            }
            match option_can_edit_roles {
                Some(query_can_edit_roles) => final_can_edit_roles = query_can_edit_roles,
                _ => (),
            }
            match option_can_moderate_roles {
                Some(query_can_moderate_roles) => {
                    final_can_moderate_roles = query_can_moderate_roles
                }
                _ => (),
            }
            match option_can_moderate_comments {
                Some(query_can_moderate_comments) => {
                    final_can_moderate_comments = query_can_moderate_comments
                }
                _ => (),
            }
            match option_can_embed {
                Some(query_can_embed) => final_can_embed = query_can_embed,
                _ => (),
            }
            match option_can_comment {
                Some(query_can_comment) => final_can_comment = query_can_comment,
                _ => (),
            }
            match option_comments_visible {
                Some(query_comments_visible) => final_comments_visible = query_comments_visible,
                _ => (),
            }

            Some(
                diesel::update(&query_role)
                    .set((
                        role_name.eq(final_role_name),
                        role_level.eq(final_role_level),
                        can_edit_pages.eq(final_can_edit_pages),
                        can_edit_posts.eq(final_can_edit_posts),
                        can_edit_roles.eq(final_can_edit_roles),
                        can_moderate_roles.eq(final_can_moderate_roles),
                        can_moderate_comments.eq(final_can_moderate_comments),
                        can_embed.eq(final_can_embed),
                        can_comment.eq(final_can_comment),
                        comments_visible.eq(final_comments_visible),
                    ))
                    .execute(conn),
            )
        }
        _ => None,
    }
}

//Delete

pub fn delete_role(conn: &PgConnection, role_id: i32) -> QueryResult<Role> {
    match read_role_id(conn, role_id) {
        Ok(query_role) => diesel::delete(&query_role).get_result(conn),
        Err(e) => Err(e),
    }
}
