#[derive(Clone, Default)]
pub struct UserData {
    pub(crate) username: String,
    pub(crate) name: String
}


#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Cancel,
    RegisterUser,
    ConfirmUser {
        data: UserData
    },
    CreateGroup,
    RecieveGroupName,
    RecieveUserToAdd,
    ModifyGroup,
    RecieveGroupNameToModify,
    RecieveModifyGroupAction,
    RecieveModifyGroupUserToAdd,
    RecieveUserToRemove,
    RecieveNewDescription,
    AddExpense,
    RecieveAddExpenseType,
    RecieveAddExpenseUser,
    RecieveAddExpenseGroup,
    RecieveAddExpenseAmountEqualSplit,
    RecieveAddExpenseAmountCustomSplit,
}
