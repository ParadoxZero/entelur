use teloxide::{
    dispatching::{dialogue::{self, InMemStorage}, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::{self, BotCommands},
};

use super::state::State;

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

// pub fn create_group_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//     use dptree::case;

//     let handler = Update::filter_message().branch(case![State::])
// }