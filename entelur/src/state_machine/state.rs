#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Cancel,
    RegisterUser,
    ConfirmUser {
        user_name: String
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
