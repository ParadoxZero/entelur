#[derive(Debug, Clone, Copy)]
pub struct MigrationData {
    version: u32,
    last_migration_time: u64,
}

pub trait Migration {
    async fn get_version() -> u32;
    async fn process<T>(connection: T) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait MigrationHandler {
    async fn get_latest_migration_details() -> Result<MigrationData, Box<dyn std::error::Error>>;
    async fn apply_migration() -> Result<(), Box<dyn std::error::Error>>;
}
