use chrono::NaiveDateTime;

#[derive(DbEnum, Debug)]
pub enum UserActionTypes {
    login,
    logout,
    create,
    delete,
    reset_password_logged_in,
    reset_password_email_sent,
    reset_password_email
}

table! {
    use super::UserActionTypesMapping;
    use diesel::types::{Int4, Inet, Timestamp, Nullable};
    user_actions (user_action_key) {
        user_action_key -> Int4,
        actor -> Int4,
        ip -> Nullable<Inet>,
        user_action -> UserActionTypesMapping,
        created_at -> Timestamp,
    }
}

#[derive(Queryable)]
pub struct UserActions {
    pub user_action_key: i32,
    pub actor: i32,
    pub ip: String,
    pub user_action: UserActionTypes,
    pub created_at: NaiveDateTime
}


#[derive(DbEnum, Debug)]
pub enum ModerationActionTypes {
    change_role,
    delete_comment,
    undo_delete_comment
}

#[derive(Queryable)]
pub struct ModerationActions {
    pub moderation_action_key: i32,
    pub moderator: i32,
    pub actor: i32,
    pub pre_action_role: i32,
    pub moderation_action: ModerationActionTypes,
    pub created_at: NaiveDateTime
}

table! {
    use super::ModerationActionTypesMapping;
    use diesel::types::{Int4, Inet, Timestamp};
    moderation_actions (moderation_action_key) {
        moderation_action_key -> Int4,
        moderator -> Int4,
        actor -> Int4,
        pre_action_role -> Int4,
        moderation_action -> ModerationActionTypesMapping,
        created_at -> Timestamp,
    }
}

