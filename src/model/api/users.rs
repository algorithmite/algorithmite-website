use crate::model::{
    api::password::{encode_password, test_hash},
    database::{finalized_schema::users::dsl::*, insertable::NewUser, queryable::User},
};
use chrono::Utc;
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

//CRUD

//Create

pub fn create_user(
    conn: &PgConnection,
    input_username: String,
    input_email: String,
    input_bio: String,
    input_user_role: i32,
    input_password: String,
) -> Option<QueryResult<User>> {
    match read_user(
        conn,
        false,
        None,
        Some(input_username.to_owned()),
        None,
        None,
        None,
    ) {
        Ok(_) => return None,
        _ => (),
    }
    match read_user(conn, false, None, None, Some(input_email.to_owned()), None, None) {
        Ok(_) => return None,
        _ => (),
    }
    let new_user: NewUser = NewUser {
        user_role: input_user_role,
        username: &input_username,
        email: &input_email,
        bio: &input_bio,
        password_hash: &encode_password(input_password),
    };
    Some(insert_into(users).values(&new_user).get_result(conn))
}

//Read

pub fn read_user(
    conn: &PgConnection,
    query_deleted: bool,
    option_id: Option<i32>,
    option_username: Option<String>,
    option_email: Option<String>,
    option_bio: Option<String>,
    option_user_role: Option<i32>,
) -> QueryResult<User> {
    let mut query = users.into_boxed();
    if !query_deleted {
        query = query.filter(deleted_at.is_null())
    }
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_username {
        Some(query_username) => query = query.filter(username.eq(query_username)),
        _ => (),
    }
    match option_email {
        Some(query_email) => query = query.filter(email.eq(query_email)),
        _ => (),
    }
    match option_bio {
        Some(query_bio) => query = query.filter(bio.eq(query_bio)),
        _ => (),
    }
    match option_user_role {
        Some(query_user_role) => query = query.filter(user_role.eq(query_user_role)),
        _ => (),
    }
    query.first(conn)
}

pub fn read_users(
    conn: &PgConnection,
    query_deleted: bool,
    option_id: Option<i32>,
    option_username: Option<String>,
    option_email: Option<String>,
    option_bio: Option<String>,
    option_user_role: Option<i32>,
) -> QueryResult<Vec<User>> {
    let mut query = users.into_boxed();
    if !query_deleted {
        query = query.filter(deleted_at.is_null())
    }
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_username {
        Some(query_username) => query = query.filter(username.eq(query_username)),
        _ => (),
    }
    match option_email {
        Some(query_email) => query = query.filter(email.eq(query_email)),
        _ => (),
    }
    match option_bio {
        Some(query_bio) => query = query.filter(bio.eq(query_bio)),
        _ => (),
    }
    match option_user_role {
        Some(query_user_role) => query = query.filter(user_role.eq(query_user_role)),
        _ => (),
    }
    query.get_results(conn)
}

//Update

pub fn update_username(conn: &PgConnection, user_id: i32, new_username: String) -> Option<String> {
    match read_user(conn, false, None, Some(new_username.clone()), None, None, None) {
        Ok(_) => None,
        _ => match read_user(conn, false, Some(user_id), None, None, None, None) {
            Ok(query_user) => {
                let old_username = query_user.username.clone();
                match diesel::update(&query_user)
                    .set(username.eq(new_username))
                    .execute(conn)
                {
                    Ok(_) => Some(old_username),
                    _ => None,
                }
            }
            _ => None,
        },
    }
}

pub fn update_email(conn: &PgConnection, user_id: i32, new_email: String) -> Option<String> {
    match read_user(conn, false, None, None, Some(new_email.clone()), None, None) {
        Ok(_) => None,
        _ => match read_user(conn, false, Some(user_id), None, None, None, None) {
            Ok(query_user) => {
                let old_email = query_user.email.clone();
                match diesel::update(&query_user)
                    .set(email.eq(new_email))
                    .execute(conn)
                {
                    Ok(_) => Some(old_email),
                    _ => None,
                }
            }
            _ => None,
        },
    }
}

pub fn update_user_role(conn: &PgConnection, user_id: i32, new_user_role: i32) -> Option<i32> {
    match read_user(conn, false, Some(user_id), None, None, None, None) {
        Ok(query_user) => {
            let old_user_role = query_user.user_role;
            match diesel::update(&query_user)
                .set(user_role.eq(new_user_role))
                .execute(conn)
            {
                Ok(_) => Some(old_user_role),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn update_bio(conn: &PgConnection, user_id: i32, new_bio: String) -> Option<String> {
    match read_user(conn, false, Some(user_id), None, None, None, None) {
        Ok(query_user) => {
            let old_bio = query_user.bio.clone();
            match diesel::update(&query_user)
                .set(bio.eq(new_bio))
                .execute(conn)
            {
                Ok(_) => Some(old_bio),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn test_password(
    conn: &PgConnection,
    query_id: Option<i32>,
    query_username: Option<String>,
    query_email: Option<String>,
    query_password: String,
) -> bool {
    let mut query_user = match query_id {
        Some(_) => match read_user(conn, false, query_id, None, None, None, None) {
            Ok(user) => Some(user),
            _ => None,
        },
        _ => None,
    };

    query_user = match query_user {
        Some(user) => Some(user),
        None => match query_username {
            Some(_) => match read_user(conn, false, None, query_username, None, None, None) {
                Ok(user) => Some(user),
                _ => None,
            },
            _ => None,
        },
    };

    query_user = match query_user {
        Some(user) => Some(user),
        None => match query_email {
            Some(_) => match read_user(conn, false, None, None, query_email, None, None) {
                Ok(user) => Some(user),
                _ => None,
            },
            _ => None,
        },
    };

    match query_user {
        Some(user) => test_hash(user.password_hash, query_password),
        _ => false,
    }
}

pub fn update_password(
    conn: &PgConnection,
    user_id: i32,
    old_password: String,
    new_password: String,
) -> bool {
    match read_user(conn, false, Some(user_id), None, None, None, None) {
        Ok(query_user) => {
            if test_hash(query_user.password_hash.clone(), old_password) {
                match diesel::update(&query_user)
                    .set(password_hash.eq(&encode_password(new_password)))
                    .execute(conn)
                {
                    Ok(_) => true,
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

//Delete

pub fn delete_user(conn: &PgConnection, user_id: i32) -> bool {
    match read_user(conn, false, Some(user_id), None, None, None, None) {
        Ok(query_user) => match diesel::update(&query_user)
            .set(deleted_at.eq(Utc::now().naive_utc()))
            .execute(conn)
        {
            Ok(_) => true,
            _ => false,
        },
        _ => true,
    }
}
