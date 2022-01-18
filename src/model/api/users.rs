use crate::model::database::{
    finalized_schema::users::dsl::*, insertable::NewUser, queryable::User,
};
use argon2::{self, Config, ThreadMode};
use chrono::Utc;
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

const DEFAULT_SALT: &[u8] = b"fDzMWxV9RYwZ60ZzG0b4AejOEho/mVeFmnzwswpmUnEw";

pub enum PasswordStatus {
    UserDoesNotExist,
    UserDeleted,
    NoMatch,
    Match,
}

pub enum UserCreationStatus {
    Success(UserExistsStatus),
    UsernameExists,
    EmailExists,
}

pub enum UserExistsStatus {
    Exists(User),
    Deleted(User),
    DoesNotExist,
}

pub enum UserUpdateStatus<T> {
    Success(T),
    Failure(T),
    ValueOverlap,
    DoesNotExist,
}

fn user_exists(test_user: QueryResult<User>) -> UserExistsStatus {
    match test_user {
        Ok(query_user) => match query_user.deleted_at {
            Some(_) => return UserExistsStatus::Deleted(query_user),
            None => return UserExistsStatus::Exists(query_user),
        },
        _ => return UserExistsStatus::DoesNotExist,
    };
}

fn users_exist(test_users: QueryResult<Vec<User>>) -> Vec<UserExistsStatus> {
    let mut output = Vec::new();
    match test_users {
        Ok(test_users) => {
            for query_user in test_users.iter() {
                output.push(match query_user.deleted_at {
                    Some(_) => UserExistsStatus::Deleted(query_user.clone()),
                    None => UserExistsStatus::Exists(query_user.clone()),
                })
            }
        }
        _ => (),
    }
    output
}

pub fn query_all_id(conn: &PgConnection, query_id: i32) -> UserExistsStatus {
    user_exists(users.filter(id.eq(query_id)).first(conn))
}

pub fn query_active_username(conn: &PgConnection, query_username: String) -> UserExistsStatus {
    user_exists(
        users
            .filter(deleted_at.is_null())
            .filter(username.eq(query_username))
            .first(conn),
    )
}

pub fn query_active_email(conn: &PgConnection, query_email: String) -> UserExistsStatus {
    user_exists(
        users
            .filter(deleted_at.is_null())
            .filter(email.eq(query_email))
            .first(conn),
    )
}

pub fn query_all_email(conn: &PgConnection, query_email: String) -> Vec<UserExistsStatus> {
    users_exist(users.filter(email.eq(query_email)).get_results(conn))
}

fn encode_password(input_password: String) -> String {
    let mut config = Config::default();
    config.lanes = 4;
    config.thread_mode = ThreadMode::Parallel;
    argon2::hash_encoded(input_password.as_bytes(), DEFAULT_SALT, &config).unwrap()
}

pub fn test_password(
    conn: &PgConnection,
    query_username: String,
    query_password: String,
) -> PasswordStatus {
    let user_status = query_active_username(conn, query_username);
    let query_user = match user_status {
        UserExistsStatus::DoesNotExist => return PasswordStatus::UserDoesNotExist,
        UserExistsStatus::Deleted(_) => return PasswordStatus::UserDeleted,
        UserExistsStatus::Exists(query_user) => query_user,
    };
    if argon2::verify_encoded(&query_user.password_hash, query_password.as_bytes()).unwrap() {
        PasswordStatus::Match
    } else {
        PasswordStatus::NoMatch
    }
}

pub fn create_user(
    conn: &PgConnection,
    input_username: String,
    input_email: String,
    input_user_role: i32,
    input_password: String,
) -> UserCreationStatus {
    match query_active_username(conn, input_username.to_owned()) {
        UserExistsStatus::Exists(_) => return UserCreationStatus::UsernameExists,
        _ => (),
    };
    match query_active_email(conn, input_email.to_owned()) {
        UserExistsStatus::Exists(_) => return UserCreationStatus::EmailExists,
        _ => (),
    };
    let new_user: NewUser = NewUser {
        user_role: input_user_role,
        username: &input_username,
        email: &input_email,
        password_hash: &encode_password(input_password),
    };
    UserCreationStatus::Success(user_exists(
        insert_into(users).values(&new_user).get_result(conn),
    ))
}

pub fn update_user_role(
    conn: &PgConnection,
    user_id: i32,
    new_user_role: i32,
) -> UserUpdateStatus<i32> {
    match query_all_id(conn, user_id) {
        UserExistsStatus::Exists(query_user) | UserExistsStatus::Deleted(query_user) => {
            let old_user_role = query_user.user_role;
            match diesel::update(&query_user)
                .set(user_role.eq(new_user_role))
                .execute(conn)
            {
                Ok(_) => UserUpdateStatus::Success(old_user_role),
                Err(_) => UserUpdateStatus::Failure(old_user_role),
            }
        }
        UserExistsStatus::DoesNotExist => UserUpdateStatus::DoesNotExist,
    }
}

pub fn update_username(
    conn: &PgConnection,
    user_id: i32,
    new_username: String,
) -> UserUpdateStatus<String> {
    match query_active_username(conn, new_username.clone()) {
        UserExistsStatus::Exists(_) => UserUpdateStatus::ValueOverlap,
        UserExistsStatus::Deleted(_) | UserExistsStatus::DoesNotExist => {
            match query_all_id(conn, user_id) {
                UserExistsStatus::Exists(query_user) | UserExistsStatus::Deleted(query_user) => {
                    let old_username = query_user.username.clone();
                    match diesel::update(&query_user)
                        .set(username.eq(new_username))
                        .execute(conn)
                    {
                        Ok(_) => UserUpdateStatus::Success(old_username),
                        Err(_) => UserUpdateStatus::Failure(old_username),
                    }
                }
                UserExistsStatus::DoesNotExist => UserUpdateStatus::DoesNotExist,
            }
        }
    }
}

pub fn update_email(
    conn: &PgConnection,
    user_id: i32,
    new_email: String,
) -> UserUpdateStatus<String> {
    match query_active_email(conn, new_email.clone()) {
        UserExistsStatus::Exists(_) => UserUpdateStatus::ValueOverlap,
        UserExistsStatus::Deleted(_) | UserExistsStatus::DoesNotExist => {
            match query_all_id(conn, user_id) {
                UserExistsStatus::Exists(query_user) | UserExistsStatus::Deleted(query_user) => {
                    let old_email = query_user.email.clone();
                    match diesel::update(&query_user)
                        .set(email.eq(new_email))
                        .execute(conn)
                    {
                        Ok(_) => UserUpdateStatus::Success(old_email),
                        Err(_) => UserUpdateStatus::Failure(old_email),
                    }
                }
                UserExistsStatus::DoesNotExist => UserUpdateStatus::DoesNotExist,
            }
        }
    }
}

pub fn update_password(
    conn: &PgConnection,
    user_id: i32,
    old_password: String,
    new_password: String,
) -> UserUpdateStatus<String> {
    match query_all_id(conn, user_id) {
        UserExistsStatus::Exists(query_user) | UserExistsStatus::Deleted(query_user) => {
            match argon2::verify_encoded(&query_user.password_hash, old_password.as_bytes()) {
                Ok(_) => match diesel::update(&query_user)
                    .set(password_hash.eq(encode_password(new_password)))
                    .execute(conn)
                {
                    Ok(_) => UserUpdateStatus::Success(old_password),
                    Err(_) => UserUpdateStatus::Failure(old_password),
                },
                Err(_) => UserUpdateStatus::Failure(old_password),
            }
        }
        UserExistsStatus::DoesNotExist => UserUpdateStatus::DoesNotExist,
    }
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> UserExistsStatus {
    match query_all_id(conn, user_id) {
        UserExistsStatus::Exists(query_user) => user_exists(
            diesel::update(&query_user)
                .set(deleted_at.eq(Utc::now().naive_utc()))
                .get_result(conn),
        ),
        UserExistsStatus::Deleted(query_user) => UserExistsStatus::Deleted(query_user),
        UserExistsStatus::DoesNotExist => UserExistsStatus::DoesNotExist,
    }
}
