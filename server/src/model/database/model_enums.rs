#[derive(Clone, DbEnum, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UserActionTypes {
    Create,
    Login,
    Logout,
    ResetPasswordEmailSent,
    ResetPasswordFromEmail,
    ResetPasswordLoggedIn,
    Delete,
}

#[derive(Clone, DbEnum, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ModerationActionTypes {
    ChangeRole,
    DeleteComment,
    UndoDeleteComment,
}
