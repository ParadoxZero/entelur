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

use chrono::DateTime;
use rusqlite::{params, Connection, Result, Row};
use std::path::{Iter, Path, PathBuf};

use crate::model::DataError;
use crate::model::migrations::{Migration, MigrationData};
use crate::model::sqlite::backend::SqliteBackend;

const C_MIGRATION_TABLE_NAME: &str = "migrations";

static C_MIGRATION_LIST: [Migration;1] = [
    Migration {
        version: 1,
        sql_statements: "
        CREATE TABLE USER(user_id STRING PRIMARY KEY, name STRING, username STRING);
        CREATE TABLE EXPENSE_GROUP(group_id INTEGER PRIMARY KEY AUTOINCREMENT, name STRING, description STRING, created_by STRING);
        CREATE TABLE GROUP_MEMBERSHIP(user_id STRING, group_id STRING);
        CREATE TABLE EXPENSE(id INTEGER PRIMARY KEY AUTOINCREMENT,added_by STRING, group_id STRING, amount INTEGER, title STRING, description STRING, split_type INTEGER);
        CREATE TABLE USER_EXPENSES(user_id STRING, expense_id INTEGER, split INTEGER);
        "
    }
];

impl SqliteBackend {
    pub async fn migrate_database(&mut self) -> Result<(), DataError> {
        let lock = self.rw_lock.write().await;
        let mut connection = self.get_new_connection()?;
        if !check_if_migration_table_exists(&mut connection) {
            create_migrations_table(&mut connection)?;
        }
        let latest_migration = get_latest_migration(&mut connection)?;
        let last_version = match latest_migration {
            Some(migration) => migration.version,
            None => 0,
        };
        let available_migrations = Vec::from(C_MIGRATION_LIST);
        let mut migrations_to_apply = available_migrations
            .iter()
            .filter(|migration| migration.version > last_version);

        apply_migrations(&mut migrations_to_apply, &mut connection)?;
        
        Result::Ok(())
    }
}

fn apply_migrations(
    migrations_to_apply: &mut dyn Iterator<Item = &Migration>,
    connection: &mut Connection,
) -> Result<(), rusqlite::Error> {
    let tx = connection.transaction()?;
    for migration in migrations_to_apply.into_iter() {
        tx.execute_batch(migration.sql_statements)?;
        let current_time = chrono::offset::Utc::now();
        tx.execute(
            "INSERT INTO migrations(version, migration_time) VALUES (?1, CURRENT_TIMESTAMP)",
            params![migration.version],
        )?;
    }

    tx.commit()?;
    Ok(())
}

fn get_latest_migration(
    connection: &mut Connection,
) -> Result<Option<MigrationData>, rusqlite::Error> {
    match connection.query_row(
        "SELECT * FROM migrations WHERE version=(SELECT MAX(version) FROM migrations);",
        [],
        |row| {
            Ok(MigrationData {
                version: row.get(0)?,
                last_migration_time: row.get(1)?,
            })
        },
    ) {
        Ok(migration) => Ok(Some(migration)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

fn create_migrations_table(connection: &mut Connection) -> Result<(), rusqlite::Error> {
    let tx = connection.transaction()?;
    tx.execute("CREATE TABLE IF NOT EXISTS migrations (version INTEGER PRIMARY KEY, migration_time DATETIME);",[])?;
    tx.commit()?;
    Result::Ok(())
}

fn check_if_migration_table_exists(connection: &mut Connection) -> bool {
    let query_result: Result<String> = connection.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name=?1;",
        [C_MIGRATION_TABLE_NAME],
        |row: &Row| row.get(0),
    );
    match query_result {
        Ok(_) => true,
        Err(rusqlite::Error::QueryReturnedNoRows) => false,
        Err(err) => panic!(
            "Unknown fatal failure while checking for migrations {}",
            err
        ),
    }
}
