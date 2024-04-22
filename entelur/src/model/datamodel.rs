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

use super::DataError;

pub type UserId = String;
pub type GroupId = u32;

pub enum SplitType {
    Equal,
    Percent,
    Amount,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: UserId,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_id: GroupId,
    pub name: String,
    pub description: String,
    pub created_by: UserId,
}

#[derive(Debug, Clone)]
pub struct GroupMembership {
    pub user_id: UserId,
    pub group_id: GroupId,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: Option<u32>,
    pub added_by: UserId,
    pub group: GroupId,
    pub amount: u32,
    pub title: String,
    pub description: String,
    pub split_type: u32
}

#[derive(Debug, Clone)]
pub struct UserExpenses {
    pub user_id: UserId,
    pub expenses_id: u32,
    pub split: u32,
}

impl User {
    pub fn new(user_id: UserId, name: String, username: String) -> User {
        User {
            user_id,
            name,
            username,
        }
    }
}

impl Group {
    pub fn new(group_id: GroupId, name: String, description: String, created_by: UserId) -> Group {
        Group {
            group_id,
            name,
            description,
            created_by,
        }
    }
}

impl TryFrom<u32> for SplitType {
    type Error = DataError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SplitType::Equal),
            1 => Ok(SplitType::Percent),
            2 => Ok(SplitType::Amount),
            _ => Err(DataError::InvalidColumnType),
        }
    }
}

impl TryInto<u32> for SplitType {
    type Error = DataError;
    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            SplitType::Equal => Ok(0),
            SplitType::Percent => Ok(1),
            SplitType::Amount => Ok(2),
        }
    }
}

impl GroupMembership {
    pub fn new(user_id: UserId, group_id: GroupId) -> GroupMembership {
        GroupMembership { user_id, group_id }
    }
}

impl Expense {
    pub fn new(
        added_by: UserId,
        group: GroupId,
        amount: u32,
        title: String,
        description: String,
    ) -> Expense {
        Expense {
            id: Option::None,
            added_by,
            group,
            amount,
            title,
            description,
            split_type: SplitType::Equal.try_into().unwrap()
        }
    }
}

pub trait Datamodel {
    async fn add_user(&self, user: User) -> Result<(), DataError>;
    async fn add_group(&self, group: Group) -> Result<(), DataError>;
    async fn add_user_to_group(&self, group_id: GroupId, user_id: UserId) -> Result<(), DataError>;
    async fn add_expense(&self, expense: Expense) -> Result<(), DataError>;

    async fn get_user(&self, user_id: UserId) -> Result<User, DataError>;
    async fn get_group(&self, group_id: GroupId) -> Result<Group, DataError>;
    async fn get_group_members(&self, group_id: GroupId) -> Result<Vec<User>, DataError>;
    async fn get_expenses(&self, group_id: GroupId) -> Result<Vec<Expense>, DataError>;

    async fn remove_user_from_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> Result<(), DataError>;
    async fn delete_group(&self, group_id: GroupId) -> Result<(), DataError>;
    async fn delete_user(&self, user_id: UserId) -> Result<(), DataError>;
    async fn delete_expense(&self, expense_id: u32) -> Result<(), DataError>;

    async fn get_membership(&self, user_id: UserId) -> Result<Vec<GroupMembership>, DataError>;
    async fn get_user_expenses(&self, user_id: UserId) -> Result<Vec<Expense>, DataError>;
}
