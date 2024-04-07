#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Cancel,
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
