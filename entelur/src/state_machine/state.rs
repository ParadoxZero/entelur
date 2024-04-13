/*
This file is part of Entelur (https://github.com/ParadoxZero/entelur/).
Copyright (c) 2024 Sidhin S Thomas.

Entelur is free software: you can redistribute it and/or modify it under the terms of the 
GNU General Public License as published by the Free Software Foundation, either version 3 
of the License, or (at your option) any later version.

Entelur is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. 
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Foobar. 
If not, see <https://www.gnu.org/licenses/>.
*/

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
