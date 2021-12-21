use rocket_sync_db_pools::{diesel::PgConnection, database};
use crate::model::database::{insertable::NewUser, queryable::User};

pub fn queryUserByID(conn: &mut PgConnection) {}
