use crate::model::database::{
    finalized_schema::users::dsl::*, insertable::NewUser, queryable::User,
};
use argon2::{self, Config, ThreadMode};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

/*
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_role: i32,
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
 *
#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub user_role: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime
}
 */

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

fn userExists(test_user: QueryResult<User>) -> UserExistsStatus {
    match test_user {
        Ok(query_user) => match query_user.deleted_at {
            Some(_) => return UserExistsStatus::Deleted(query_user),
            None => return UserExistsStatus::Exists(query_user),
        },
        _ => return UserExistsStatus::DoesNotExist,
    };
}

fn usersExist(test_users: QueryResult<Vec<User>>) -> Vec<UserExistsStatus> {
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

pub fn queryAllID(conn: &PgConnection, query_id: i32) -> UserExistsStatus {
    userExists(users.filter(id.eq(query_id)).first(conn))
}

pub fn queryActiveUsername(conn: &PgConnection, query_username: String) -> UserExistsStatus {
    userExists(
        users
            .filter(deleted_at.is_null())
            .filter(username.eq(query_username))
            .first(conn),
    )
}

pub fn queryActiveEmail(conn: &PgConnection, query_email: String) -> UserExistsStatus {
    userExists(
        users
            .filter(deleted_at.is_null())
            .filter(email.eq(query_email))
            .first(conn),
    )
}

pub fn queryAllEmail(conn: &PgConnection, query_email: String) -> Vec<UserExistsStatus> {
    usersExist(users.filter(email.eq(query_email)).get_results(conn))
}

fn encodePassword(input_password: String) -> String {
    let mut config = Config::default();
    config.lanes = 4;
    config.thread_mode = ThreadMode::Parallel;
    argon2::hash_encoded(input_password.as_bytes(), DEFAULT_SALT, &config).unwrap()
}

pub fn testPassword(
    conn: &PgConnection,
    query_username: String,
    query_password: String,
) -> PasswordStatus {
    let user_status = queryActiveUsername(conn, query_username);
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

pub fn createUser(
    conn: &PgConnection,
    input_username: String,
    input_email: String,
    input_password: String,
) -> UserCreationStatus {
    match queryActiveUsername(conn, input_username.to_owned()) {
        UserExistsStatus::Exists(_) => return UserCreationStatus::UsernameExists,
        _ => (),
    };
    match queryActiveEmail(conn, input_email.to_owned()) {
        UserExistsStatus::Exists(_) => return UserCreationStatus::EmailExists,
        _ => (),
    };
    let new_user: NewUser = NewUser {
        //TODO Get Default Role ID
        user_role: 0,
        username: &input_username,
        email: &input_email,
        password_hash: &encodePassword(input_password),
    };
    UserCreationStatus::Success(userExists(
        insert_into(users).values(&new_user).get_result(conn),
    ))
}
