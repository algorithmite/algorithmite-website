use crate::model::database::{
    finalized_schema::moderation_actions::dsl::*, insertable::NewModerationAction,
    model_enums::ModerationActionTypes, queryable::ModerationAction,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

//Create

pub fn create_moderation_action(
    conn: &PgConnection,
    input_moderator: i32,
    input_actor: i32,
    input_pre_action_role: i32,
    input_moderation_action: ModerationActionTypes,
) -> QueryResult<ModerationAction> {
    let new_moderation_action: NewModerationAction = NewModerationAction {
        moderator: input_moderator,
        actor: input_actor,
        pre_action_role: input_pre_action_role,
        moderation_action: input_moderation_action,
    };
    insert_into(moderation_actions)
        .values(&new_moderation_action)
        .get_result(conn)
}

//Read

pub fn read_moderation_action(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_moderator: Option<i32>,
    option_actor: Option<i32>,
    option_pre_action_role: Option<i32>,
    option_moderation_action: Option<ModerationActionTypes>,
) -> QueryResult<ModerationAction> {
    let mut query = moderation_actions.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_moderator {
        Some(query_moderator) => query = query.filter(moderator.eq(query_moderator)),
        _ => (),
    }
    match option_actor {
        Some(query_actor) => query = query.filter(actor.eq(query_actor)),
        _ => (),
    }
    match option_pre_action_role {
        Some(query_pre_action_role) => query = query.filter(pre_action_role.eq(query_pre_action_role)),
        _ => (),
    }
    match option_moderation_action {
        Some(query_moderation_action) => query = query.filter(moderation_action.eq(query_moderation_action)),
        _ => (),
    }
    query.first(conn)
}

pub fn read_moderation_actions(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_moderator: Option<i32>,
    option_actor: Option<i32>,
    option_pre_action_role: Option<i32>,
    option_moderation_action: Option<ModerationActionTypes>,
) -> QueryResult<Vec<ModerationAction>> {
    let mut query = moderation_actions.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_moderator {
        Some(query_moderator) => query = query.filter(moderator.eq(query_moderator)),
        _ => (),
    }
    match option_actor {
        Some(query_actor) => query = query.filter(actor.eq(query_actor)),
        _ => (),
    }
    match option_pre_action_role {
        Some(query_pre_action_role) => query = query.filter(pre_action_role.eq(query_pre_action_role)),
        _ => (),
    }
    match option_moderation_action {
        Some(query_moderation_action) => query = query.filter(moderation_action.eq(query_moderation_action)),
        _ => (),
    }
    query.get_results(conn)
}

//Update

pub fn update_moderation_action(
    conn: &PgConnection,
    moderation_action_id: i32,
    option_moderator: Option<i32>,
    option_actor: Option<i32>,
    option_pre_action_role: Option<i32>,
    option_moderation_action: Option<ModerationActionTypes>,
) -> Option<QueryResult<usize>> {
    match read_moderation_action(conn, Some(moderation_action_id), None, None, None, None) {
        Ok(query_role) => {
            let mut final_moderator: i32 = query_role.moderator;
            let mut final_actor: i32 = query_role.actor;
            let mut final_pre_action_role: i32 = query_role.pre_action_role;
            let mut final_moderation_action: ModerationActionTypes = query_role.moderation_action.clone();

            match option_moderator {
                Some(query_moderator) => final_moderator = query_moderator,
                _ => (),
            }
            match option_actor {
                Some(query_actor) => final_actor = query_actor,
                _ => (),
            }
            match option_pre_action_role {
                Some(query_pre_action_role) => final_pre_action_role = query_pre_action_role,
                _ => (),
            }
            match option_moderation_action {
                Some(query_moderation_action) => final_moderation_action = query_moderation_action,
                _ => (),
            }

            Some(
                diesel::update(&query_role)
                    .set((
                        moderator.eq(final_moderator),
                        actor.eq(final_actor),
                        pre_action_role.eq(final_pre_action_role),
                        moderation_action.eq(final_moderation_action),
                    ))
                    .execute(conn),
            )
        }
        _ => None,
    }
}

//Delete

pub fn delete_moderation_action(conn: &PgConnection, moderation_action_id: i32) -> QueryResult<ModerationAction> {
    match read_moderation_action(conn, Some(moderation_action_id), None, None, None, None) {
        Ok(query_moderation_action) => diesel::delete(&query_moderation_action).get_result(conn),
        Err(e) => Err(e),
    }
}
