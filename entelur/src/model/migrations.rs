use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Copy)]
pub(super) struct MigrationData {
    pub(super) version: u32,
    pub(super) last_migration_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Migration{
    pub(super) version: u32,
    pub(super) sql_statements: &'static str
}