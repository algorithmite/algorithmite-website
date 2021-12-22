use crate::model::database::{
    finalized_schema::users::dsl::*, insertable::NewUser, queryable::User,
};
use rocket_sync_db_pools::{
    database,
    diesel::{
        prelude::*,
        result::{Error, QueryResult},
        PgConnection,
    },
};

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

pub fn queryUserByID(conn: &mut PgConnection, query_id: i32) -> QueryResult<User> {
    diesel::update(users.filter(id.eq(query_id))).get_result::<User>(&conn)
}

pub fn queryUserByUsername(conn: &mut PgConnection, query_username: String) -> QueryResult<User> {
    diesel::update(users.filter(username.eq(query_username))).get_result::<User>(&conn)
}
