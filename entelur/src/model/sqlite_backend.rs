use super::datamodel::{DataError, GroupId, UserId};

use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub struct SqliteBackend {
    file_path: PathBuf,
    rw_lock: RwLock<u32>,
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

impl super::datamodel::Datamodel for SqliteBackend {
    async fn add_user(
        &self,
        user: super::datamodel::User,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn add_group(
        &self,
        group: super::datamodel::Group,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn add_user_to_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn add_expense(
        &self,
        expense: super::datamodel::Expense,
    ) -> std::prelude::v1::Result<(), DataError> {
        todo!()
    }

    async fn get_user(
        &self,
        user_id: UserId,
    ) -> std::prelude::v1::Result<super::datamodel::User, DataError> {
        todo!()
    }

    async fn get_group(
        &self,
        group_id: GroupId,
    ) -> std::prelude::v1::Result<super::datamodel::Group, DataError> {
        todo!()
    }

    async fn get_group_members(
        &self,
        group_id: GroupId,
    ) -> std::prelude::v1::Result<Vec<super::datamodel::User>, DataError> {
        todo!()
    }

    async fn get_expenses(
        &self,
        group_id: GroupId,
    ) -> std::prelude::v1::Result<Vec<super::datamodel::Expense>, DataError> {
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
    ) -> std::prelude::v1::Result<super::datamodel::GroupMembership, DataError> {
        todo!()
    }

    async fn get_user_expenses(
        &self,
        user_id: UserId,
    ) -> std::prelude::v1::Result<Vec<super::datamodel::Expense>, DataError> {
        todo!()
    }
}
