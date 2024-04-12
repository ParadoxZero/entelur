use crate::{model::datamodel::{
    DataError, Datamodel, Expense, Group, GroupId, GroupMembership, User, UserId,
}, DbBackend};

use rusqlite::{params, Connection, Result};
use std::{path::{Path, PathBuf}, rc::Rc};
use tokio::sync::RwLock;

pub struct SqliteBackend {
    file_path: PathBuf,
    pub(super) rw_lock: RwLock<u32>,
}

impl SqliteBackend {
    pub fn new(file_path: std::path::PathBuf, readers: u32) -> SqliteBackend {
        SqliteBackend {
            file_path,
            rw_lock: RwLock::new(readers),
        }
    }

    pub fn get_new_connection(&self) -> Result<Connection> {
        Connection::open(self.file_path.clone())
    }
    
    async fn ensure_group_exists(&self, connection: &Connection, group_id: u32) -> Result<(), DataError> {
        let read_lock = self.rw_lock.read().await;
        connection.query_row("SELECT group_id FROM EXPENSE_GROUP WHERE group_id = ?", [group_id], |row| {
        Ok(())
    })?;
        Ok(())
    }
}
impl Datamodel for SqliteBackend {
    async fn add_user(&self, user: User) -> Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let connection = self.get_new_connection()?;
        connection.execute(
            "INSERT INTO User(user_id, username, name) VALUES (?1 ?2 ?3) ",
            (user.user_id, user.username, user.name),
        )?;
        Result::Ok(())
    }

    async fn add_group(&self, group: Group) -> std::prelude::v1::Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let connection = self.get_new_connection()?;
        connection.execute(
            "INSERT INTO ExpenseGroup(group_id, name, description, created_by) VALUES (?1 ?2 ?3 ?4) ",
            (group.group_id, group.name, group.description, group.created_by),
        )?;
        Result::Ok(())
    }

    async fn add_user_to_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> std::prelude::v1::Result<(), DataError> {
        let connection = self.get_new_connection()?;

        self.ensure_group_exists(&connection, group_id).await?;
        let write_lock = self.rw_lock.write().await;
        connection.execute(
            "INSERT INTO GroupMembership(user_id, group_id) VALUES (?1 ?2) ",
            (user_id, group_id)
        )?;

        Result::Ok(())
    }

    async fn add_expense(&self, expense: Expense) -> std::prelude::v1::Result<(), DataError> {
        let connection = self.get_new_connection()?;
        self.ensure_group_exists(&connection, expense.group).await?;
        let write_lock = self.rw_lock.write().await;
        connection.execute(
            "INSERT INTO Expense(added_by, group, amount, title, description) VALUES (?1 ?2 ?3 ?4 ?5) ",
            (expense.added_by, expense.group, expense.amount, expense.title, expense.description),
        )?;

        Result::Ok(())
    }

    async fn get_user(&self, user_id: UserId) -> std::prelude::v1::Result<User, DataError> {
        let read_lock = self.rw_lock.read().await;
        let connection = self.get_new_connection()?;
        let user = connection.query_row("SELECT user_id, username, name FROM User WHERE user_id = ?", [user_id], |row| {
            Ok(User {
                user_id: row.get(0)?,
                username: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        Ok(user)
    }

    async fn get_group(&self, group_id: GroupId) -> std::prelude::v1::Result<Group, DataError> {
        let read_lock = self.rw_lock.read().await;
        let connection = self.get_new_connection()?;
        let group = connection.query_row("SELECT group_id, name, description, created_by FROM ExpenseGroup WHERE group_id = ?", [group_id], |row| {
            Ok(Group {
                group_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_by: row.get(3)?,
            })
        })?;

        Ok(group)
    }

    async fn get_group_members(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<User>, DataError> {
        let read_lock = self.rw_lock.read().await;
        let connection = self.get_new_connection()?;
        let mut members_query = connection.prepare("SELECT user_id, username, name FROM User WHERE user_id IN (SELECT user_id FROM GroupMembership WHERE group_id = ?)")?;
        let members_query_result = members_query.query_map([group_id], |row| {
            Ok(User {
                user_id: row.get(0)?,
                username: row.get(1)?,
                name: row.get(2)?,
            })
        })?;
        let mut members_list:Vec<User> = Vec::new();
        for member_encap in members_query_result {
            members_list.push(member_encap?);
        }
        Result::Ok(members_list)
    }

    async fn get_expenses(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<Expense>, DataError> {
        let read_lock = self.rw_lock.read().await;
        let connection = self.get_new_connection()?;

        let mut expenses_query = connection.prepare("SELECT id, added_by, group, amount, title, description FROM Expense WHERE group = ?")?;
        let expenses_query_result = expenses_query.query_map([group_id], |row| {
            Ok(Expense {
                id: row.get(0)?,
                added_by: row.get(1)?,
                group: row.get(2)?,
                amount: row.get(3)?,
                title: row.get(4)?,
                description: row.get(5)?,
            })
        })?;
        let mut expenses_list:Vec<Expense> = Vec::new();
        for expense_encap in expenses_query_result {
            expenses_list.push(expense_encap?);
        }
        Result::Ok(expenses_list)
    }

    async fn remove_user_from_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> std::prelude::v1::Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let connection = self.get_new_connection()?;
        connection.execute("DELETE FROM GroupMembership WHERE group_id = ?1 AND user_id = ?2", params![group_id, user_id])?;
        Ok(())
    }

    async fn delete_group(&self, group_id: GroupId) -> std::prelude::v1::Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let mut connection = self.get_new_connection()?;
        let tx = connection.transaction()?;
        tx.execute("DELETE FROM GroupMembership WHERE group_id = ?1", params![group_id])?;
        tx.execute("DELETE FROM ExpenseGroup WHERE group_id = ?1", params![group_id])?;
        tx.commit()?;
        Ok(())
    }

    async fn delete_user(&self, user_id: UserId) -> std::prelude::v1::Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let mut connection = self.get_new_connection()?;
        let tx = connection.transaction()?;
        tx.execute("DELETE FROM User WHERE user_id = ?1", params![user_id])?;
        tx.commit()?;
        Ok(())
    }

    async fn delete_expense(&self, expense_id: u32) -> std::prelude::v1::Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let mut connection = self.get_new_connection()?;
        let tx = connection.transaction()?;
        tx.execute("DELETE FROM Expense WHERE id = ?1", params![expense_id])?;
        tx.commit()?;
        Ok(())
    }

    async fn get_membership(
        &self,
        user_id: UserId,
    ) -> std::prelude::v1::Result<Vec<GroupMembership>, DataError> {
        let read_lock = self.rw_lock.read().await;
        let connection = self.get_new_connection()?;
        let mut membership_query = connection.prepare("SELECT group_id FROM GroupMembership WHERE user_id = ?")?;
        let membership_query_result = membership_query.query_map([user_id], |row| {
            Ok(GroupMembership {
                group_id: row.get(0)?,
                user_id: row.get(1)?,
            })
        })?;
        let mut membership_list:Vec<GroupMembership> = Vec::new();
        for member_encap in membership_query_result {
            membership_list.push(member_encap?);
        }
        Ok(membership_list)
    }

    async fn get_user_expenses(
        &self,
        user_id: UserId,
    ) -> std::prelude::v1::Result<Vec<Expense>, DataError> {
        todo!()
    }
}

impl From<rusqlite::Error> for DataError {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::SqliteFailure(_, _) => DataError::DatabaseError,
            rusqlite::Error::SqliteSingleThreadedMode => DataError::DatabaseError,
            rusqlite::Error::FromSqlConversionFailure(_, _, _) => DataError::FromSqlConversionFailure,
            rusqlite::Error::IntegralValueOutOfRange(_, _) => DataError::IntegralValueOutOfRange,
            rusqlite::Error::Utf8Error(_) => DataError::Utf8Error,
            rusqlite::Error::NulError(_) => DataError::NulError,
            rusqlite::Error::InvalidParameterName(_) => DataError::InvalidParameterName,
            rusqlite::Error::InvalidPath(_) => DataError::DatabaseError,
            rusqlite::Error::ExecuteReturnedResults => DataError::ExecuteReturnedResults,
            rusqlite::Error::QueryReturnedNoRows => DataError::QueryReturnedNoRows,
            rusqlite::Error::InvalidColumnIndex(_) => DataError::InvalidColumnIndex,
            rusqlite::Error::InvalidColumnName(_) => Self::InvalidColumnName,
            rusqlite::Error::InvalidColumnType(_, _, _) => DataError::InvalidColumnType,
            rusqlite::Error::StatementChangedRows(_) => DataError::StatementChangedRows,
            rusqlite::Error::ToSqlConversionFailure(_) => DataError::ToSqlConversionFailure,
            rusqlite::Error::InvalidQuery => DataError::InvalidQuery,
            rusqlite::Error::MultipleStatement => DataError::MultipleStatement,
            rusqlite::Error::InvalidParameterCount(_, _) => DataError::InvalidParameterCount,
            rusqlite::Error::SqlInputError {
                error,
                msg,
                sql,
                offset,
            } => panic!("{} {} {} {}", error, msg, sql, offset),
            _ => DataError::UnknownError,
        }
    }
}
