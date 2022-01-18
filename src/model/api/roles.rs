use crate::model::database::{
    finalized_schema::roles::dsl::*, insertable::NewRole, queryable::Role,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

pub enum RoleCreationStatus {
    Success(QueryResult<Role>),
    RoleNameExists,
}

pub enum RoleUpdateStatus<T> {
    Success(T),
    Failure(T),
    ValueOverlap,
    DoesNotExist,
}

pub fn query_id(conn: &PgConnection, query_id: i32) -> QueryResult<Role> {
    roles.filter(id.eq(query_id)).first(conn)
}

pub fn query_role_name(conn: &PgConnection, query_role_name: String) -> QueryResult<Role> {
    roles.filter(role_name.eq(query_role_name)).first(conn)
}

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
) -> RoleCreationStatus {
    match query_role_name(conn, input_role_name.to_owned()) {
        Ok(_) => return RoleCreationStatus::RoleNameExists,
        _ => (),
    };
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
    RoleCreationStatus::Success(insert_into(roles).values(&new_role).get_result(conn))
}

pub fn update_role_name(
    conn: &PgConnection,
    role_id: i32,
    new_role_name: String,
) -> RoleUpdateStatus<String> {
    match query_role_name(conn, new_role_name.to_owned()) {
        Ok(_) => return RoleUpdateStatus::ValueOverlap,
        _ => (),
    };
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_role_name = query_role.role_name.to_owned();
            match diesel::update(&query_role)
                .set(role_name.eq(&new_role_name.to_owned()))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_role_name),
                Err(_) => RoleUpdateStatus::Failure(old_role_name),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_role_level(
    conn: &PgConnection,
    role_id: i32,
    new_role_level: i32,
) -> RoleUpdateStatus<i32> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_role_level = query_role.role_level;
            match diesel::update(&query_role)
                .set(role_level.eq(new_role_level))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_role_level),
                Err(_) => RoleUpdateStatus::Failure(old_role_level),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_edit_pages(
    conn: &PgConnection,
    role_id: i32,
    new_can_edit_pages: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_edit_pages = query_role.can_edit_pages;
            match diesel::update(&query_role)
                .set(can_edit_pages.eq(new_can_edit_pages))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_edit_pages),
                Err(_) => RoleUpdateStatus::Failure(old_can_edit_pages),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_edit_posts(
    conn: &PgConnection,
    role_id: i32,
    new_can_edit_posts: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_edit_posts = query_role.can_edit_posts;
            match diesel::update(&query_role)
                .set(can_edit_posts.eq(new_can_edit_posts))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_edit_posts),
                Err(_) => RoleUpdateStatus::Failure(old_can_edit_posts),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_edit_roles(
    conn: &PgConnection,
    role_id: i32,
    new_can_edit_roles: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_edit_roles = query_role.can_edit_roles;
            match diesel::update(&query_role)
                .set(can_edit_roles.eq(new_can_edit_roles))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_edit_roles),
                Err(_) => RoleUpdateStatus::Failure(old_can_edit_roles),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_moderate_roles(
    conn: &PgConnection,
    role_id: i32,
    new_can_moderate_roles: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_moderate_roles = query_role.can_moderate_roles;
            match diesel::update(&query_role)
                .set(can_moderate_roles.eq(new_can_moderate_roles))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_moderate_roles),
                Err(_) => RoleUpdateStatus::Failure(old_can_moderate_roles),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_moderate_comments(
    conn: &PgConnection,
    role_id: i32,
    new_can_moderate_comments: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_moderate_comments = query_role.can_moderate_comments;
            match diesel::update(&query_role)
                .set(can_moderate_comments.eq(new_can_moderate_comments))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_moderate_comments),
                Err(_) => RoleUpdateStatus::Failure(old_can_moderate_comments),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_embed(
    conn: &PgConnection,
    role_id: i32,
    new_can_embed: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_embed = query_role.can_embed;
            match diesel::update(&query_role)
                .set(can_embed.eq(new_can_embed))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_embed),
                Err(_) => RoleUpdateStatus::Failure(old_can_embed),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_can_comment(
    conn: &PgConnection,
    role_id: i32,
    new_can_comment: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_can_comment = query_role.can_comment;
            match diesel::update(&query_role)
                .set(can_comment.eq(new_can_comment))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_can_comment),
                Err(_) => RoleUpdateStatus::Failure(old_can_comment),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn update_comments_visible(
    conn: &PgConnection,
    role_id: i32,
    new_comments_visible: bool,
) -> RoleUpdateStatus<bool> {
    match query_id(conn, role_id) {
        Ok(query_role) => {
            let old_comments_visible = query_role.comments_visible;
            match diesel::update(&query_role)
                .set(comments_visible.eq(new_comments_visible))
                .execute(conn)
            {
                Ok(_) => RoleUpdateStatus::Success(old_comments_visible),
                Err(_) => RoleUpdateStatus::Failure(old_comments_visible),
            }
        }
        Err(_) => RoleUpdateStatus::DoesNotExist,
    }
}

pub fn delete_role(conn: &PgConnection, role_id: i32) -> QueryResult<Role> {
    match query_id(conn, role_id) {
        Ok(query_role) => diesel::delete(&query_role).get_result(conn),
        Err(e) => Err(e),
    }
}
