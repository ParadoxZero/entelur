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

use model::sqlite::backend::SqliteBackend;
use model::sqlite::migrations;
use state_machine::state::State;

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

    backend
        .migrate_database()
        .await
        .expect("Failed to migrate database");

    Dispatcher::builder(bot, state_machine::schema())
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            Arc::new(backend)
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
