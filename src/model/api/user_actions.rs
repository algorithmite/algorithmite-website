use crate::model::database::{
    finalized_schema::user_actions::dsl::*, insertable::NewUserAction,
    model_enums::UserActionTypes, queryable::UserAction,
};
use ipnetwork::IpNetwork;
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

pub fn query_id(conn: &PgConnection, query_id: i32) -> QueryResult<UserAction> {
    user_actions.filter(id.eq(query_id)).first(conn)
}

pub fn query_all_user(conn: &PgConnection, query_user_id: i32) -> QueryResult<Vec<UserAction>> {
    user_actions
        .filter(actor.eq(query_user_id))
        .get_results(conn)
}

pub fn create_user_action(
    conn: &PgConnection,
    input_actor: i32,
    input_ip: Option<IpNetwork>,
    input_user_action: UserActionTypes,
) -> QueryResult<UserAction> {
    let new_user_action: NewUserAction = NewUserAction {
        actor: input_actor,
        ip: input_ip,
        user_action: input_user_action,
    };
    insert_into(user_actions)
        .values(&new_user_action)
        .get_result(conn)
}
