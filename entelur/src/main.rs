// Temporarily disable some warnings while the basic structure is being built
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod model;
mod state_machine;

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use state_machine::state::State;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok(); // Load env variable for development machines from .env file

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
