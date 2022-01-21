use crate::model::database::{
    finalized_schema::moderation_actions::dsl::*, insertable::NewModerationAction,
    model_enums::ModerationActionTypes, queryable::ModerationAction,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

pub fn query_id(conn: &PgConnection, query_id: i32) -> QueryResult<ModerationAction> {
    moderation_actions.filter(id.eq(query_id)).first(conn)
}

pub fn query_all_actor(
    conn: &PgConnection,
    query_actor_id: i32,
) -> QueryResult<Vec<ModerationAction>> {
    moderation_actions
        .filter(actor.eq(query_actor_id))
        .get_results(conn)
}

pub fn query_all_moderator(
    conn: &PgConnection,
    query_moderator_id: i32,
) -> QueryResult<Vec<ModerationAction>> {
    moderation_actions
        .filter(moderator.eq(query_moderator_id))
        .get_results(conn)
}

pub fn create_moderation_action(
    conn: &PgConnection,
    input_moderator: i32,
    input_actor: i32,
    input_pre_action_role: i32,
    input_moderation_action: ModerationActionTypes,
) -> QueryResult<ModerationAction> {
    let new_role: NewModerationAction = NewModerationAction {
        moderator: input_moderator,
        actor: input_actor,
        pre_action_role: input_pre_action_role,
        moderation_action: input_moderation_action,
    };
    insert_into(moderation_actions)
        .values(&new_role)
        .get_result(conn)
}
