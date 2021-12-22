use crate::model::database::{
    insertable::NewModerationAction, model_enums::ModerationActionTypes,
    queryable::ModerationAction,
};

/*
#[derive(Queryable, Identifiable)]
pub struct ModerationAction {
    pub id: i32,
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
    pub created_at: NaiveDateTime
}
 *
#[derive(Insertable)]
#[table_name = "moderation_actions"]
pub struct NewModerationAction {
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
}
 */

//pub fn queryAll(conn: &mut PgConnection){}

//pub fn
