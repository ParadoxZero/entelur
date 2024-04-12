// Temporarily disable some warnings while the basic structure is being built
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod model;
mod state_machine;

use std::{fmt, rc::Rc, str::FromStr, sync::Arc};

use clap::Parser;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use state_machine::state::State;
use model::sqlite::backend::SqliteBackend;
use model::sqlite::migrations;

use crate::model::datamodel::{self, Datamodel};

#[derive(Debug, Clone, Copy)]
enum DbBackend {
    Sqlite,
    InMemory,
}

impl clap::ValueEnum for DbBackend {
    fn value_variants<'a>() -> &'a [Self] {
        &[DbBackend::Sqlite, DbBackend::InMemory]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DbBackend::Sqlite => Option::Some(clap::builder::PossibleValue::new("sqlite")),
            DbBackend::InMemory => Option::Some(clap::builder::PossibleValue::new("inmemory")),
        }
    }
}

// Telegram bot for sharing expenses and settling among friends
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number parallel readers allowed in DB (Only for SQLite)
    #[arg(short, long, default_value_t = 5)]
    parallel_readers: u32,

    // Which database backend to use
    #[arg(short, long, value_enum)]
    backend: DbBackend,

    // Connection string to use
    #[arg(short, long)]
    connection_string: String,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok(); // Load env variable for development machines from .env file

    let args = Cli::parse();
    pretty_env_logger::init();
    log::info!("Starting entelur bot...");

    let bot = Bot::from_env();

    log::info!("Migrating Database...");
    let mut backend = SqliteBackend::new(args.connection_string.into(), args.parallel_readers);
    log::info!("Migration complete.");

    backend.migrate_database().await.expect("Failed to migrate database");

    let datamodel = Arc::new(backend);
    Dispatcher::builder(bot, state_machine::schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(),datamodel])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
