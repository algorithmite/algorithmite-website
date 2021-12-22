#[derive(DbEnum, Debug)]
pub enum UserActionTypes {
    Login,
    Logout,
    Create,
    Delete,
    ResetPasswordLoggedIn,
    ResetPasswordEmailSent,
    ResetPasswordEmail,
}

#[derive(DbEnum, Debug)]
pub enum ModerationActionTypes {
    ChangeRole,
    DeleteComment,
    UndoDeleteComment,
}
