use crate::{model::datamodel::{
    DataError, Datamodel, Expense, Group, GroupId, GroupMembership, User, UserId,
}, DbBackend};

use rusqlite::{Connection, Result};
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
}
impl Datamodel for SqliteBackend {
    async fn add_user(&self, user: User) -> Result<(), DataError> {
        let write_lock = self.rw_lock.write().await;
        let connection = self.get_new_connection()?;
        connection.execute(
            "INSERT INTO User VALUES (?1 ?2 ?3) ",
            (user.user_id, user.username, user.name),
        )?;
        Result::Ok(())
    }

    async fn add_group(&self, group: Group) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn add_user_to_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn add_expense(&self, expense: Expense) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn get_user(&self, user_id: UserId) -> std::prelude::v1::Result<User, DataError> {
        todo!()
    }

    async fn get_group(&self, group_id: GroupId) -> std::prelude::v1::Result<Group, DataError> {
        todo!()
    }

    async fn get_group_members(
        &self,
        group_id: GroupId,
    ) -> std::prelude::v1::Result<Vec<User>, DataError> {
        todo!()
    }

    async fn get_expenses(
        &self,
        group_id: GroupId,
    ) -> std::prelude::v1::Result<Vec<Expense>, DataError> {
        todo!()
    }

    async fn remove_user_from_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn delete_group(&self, group_id: GroupId) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn delete_user(&self, user_id: UserId) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn delete_expense(&self, expense_id: u32) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn get_membership(
        &self,
        user_id: UserId,
    ) -> std::prelude::v1::Result<GroupMembership, DataError> {
        todo!()
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
