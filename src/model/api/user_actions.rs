use crate::model::database::{
    finalized_schema::user_actions::dsl::*, insertable::NewUserAction,
    model_enums::UserActionTypes, queryable::UserAction,
};
use ipnetwork::IpNetwork;
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

//Create

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

//Read

pub fn read_user_action(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_actor: Option<i32>,
    option_ip: Option<Option<IpNetwork>>,
    option_user_action: Option<UserActionTypes>,
) -> QueryResult<UserAction> {
    let mut query = user_actions.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_actor {
        Some(query_actor) => query = query.filter(actor.eq(query_actor)),
        _ => (),
    }
    match option_ip {
        Some(query_ip) => query = query.filter(ip.eq(query_ip)),
        _ => (),
    }
    match option_user_action {
        Some(query_user_action) => query = query.filter(user_action.eq(query_user_action)),
        _ => (),
    }
    query.first(conn)
}

pub fn read_user_actions(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_actor: Option<i32>,
    option_ip: Option<Option<IpNetwork>>,
    option_user_action: Option<UserActionTypes>,
) -> QueryResult<Vec<UserAction>> {
    let mut query = user_actions.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_actor {
        Some(query_actor) => query = query.filter(actor.eq(query_actor)),
        _ => (),
    }
    match option_ip {
        Some(query_ip) => query = query.filter(ip.eq(query_ip)),
        _ => (),
    }
    match option_user_action {
        Some(query_user_action) => query = query.filter(user_action.eq(query_user_action)),
        _ => (),
    }
    query.get_results(conn)
}

//Update

pub fn update_user_action(
    conn: &PgConnection,
    user_action_id: i32,
    option_actor: Option<i32>,
    option_ip: Option<Option<IpNetwork>>,
    option_user_action: Option<UserActionTypes>,
) -> Option<QueryResult<usize>> {
    match read_user_action(conn, Some(user_action_id), None, None, None) {
        Ok(query_role) => {
            let mut final_actor: i32 = query_role.actor;
            let mut final_ip: Option<IpNetwork> = query_role.ip;
            let mut final_user_action: UserActionTypes = query_role.user_action.clone();

            match option_actor {
                Some(query_actor) => final_actor = query_actor,
                _ => (),
            }
            match option_ip {
                Some(query_ip) => final_ip = query_ip,
                _ => (),
            }
            match option_user_action {
                Some(query_user_action) => final_user_action = query_user_action,
                _ => (),
            }

            Some(
                diesel::update(&query_role)
                    .set((
                        actor.eq(final_actor),
                        ip.eq(final_ip),
                        user_action.eq(final_user_action),
                    ))
                    .execute(conn),
            )
        }
        _ => None,
    }
}

//Delete

pub fn delete_user_action(conn: &PgConnection, user_action_id: i32) -> QueryResult<UserAction> {
    match read_user_action(conn, Some(user_action_id), None, None, None) {
        Ok(query_user_action) => diesel::delete(&query_user_action).get_result(conn),
        Err(e) => Err(e),
    }
}
