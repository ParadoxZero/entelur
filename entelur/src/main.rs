// Temporarily disable some warnings while the basic structure is being built
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod model;
mod state_machine;

use std::{fmt, str::FromStr};

use clap::Parser;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use state_machine::state::State;

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

    #[arg(short, long, value_enum)]
    backend: DbBackend,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok(); // Load env variable for development machines from .env file

    let args = Cli::parse();
    pretty_env_logger::init();
    log::info!("Starting purchase bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, state_machine::schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
