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

pub mod group;
pub mod state;
pub mod user;

use std::{clone, sync::Arc};

use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    dptree::endpoint,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::{self, BotCommands},
};

use state::State;

use crate::{
    model::{
        datamodel::{Datamodel, Group},
        sqlite::backend::{self, SqliteBackend},
        DataError,
    },
    state_machine::{
        group::{create_group_callback_schema, create_group_schema},
        user::{user_callback_schema, user_schemas},
    },
};

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
    #[command(description = "Register user")]
    Register,
    #[command(description = "Create a group")]
    CreateGroup,
    #[command(description = "Modify a Group")]
    ModifyGroup,
    #[command(description = "Add user to Group")]
    AddUser,
    #[command(description = "Add Expense")]
    AddExpense,
    #[command(description = "Show pending settlements")]
    ShowPending,
    #[command(description = "Settle the group")]
    Settle,
    #[command(description = "Show summary so far")]
    ShowSummary,
    #[command(description = "Show statement for past n months")]
    ShowStatement { months: u32 },
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Cancel].endpoint(cancel))
        .branch(
            case![State::Start]
                .branch(case![Command::Register].endpoint(register))
                .branch(case![Command::CreateGroup].endpoint(create_group))
                .branch(case![Command::ModifyGroup].endpoint(modify_group))
                .branch(case![Command::AddUser].endpoint(add_user))
                .branch(case![Command::AddExpense].endpoint(add_expense))
                .branch(case![Command::ShowPending].endpoint(show_pending))
                .branch(case![Command::Settle].endpoint(settle))
                .branch(case![Command::ShowSummary].endpoint(show_summary))
                .branch(case![Command::ShowStatement { months }].endpoint(show_statement)),
        )
        .branch(case![Command::Help].endpoint(help));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .branch(command_handler)
                .branch(user_schemas().chain(create_group_schema())),
        )
        .branch(
            Update::filter_callback_query()
                .branch(user_callback_schema().chain(create_group_callback_schema())),
        )
        .branch(endpoint(invalid_state))
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message, dialogue: BotDialogue) -> HandlerResult {
    dialogue.update(State::Start).await?;
    bot.send_message(msg.chat.id, "Reset successful").await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Invalid state. please try again. Use /cancel to go back to main menu.",
    )
    .await?;
    Ok(())
}

async fn register(bot: Bot, msg: Message, dialogue: BotDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please enter your name.")
        .await?;
    dialogue.update(State::RegisterUser).await?;
    Ok(())
}

async fn create_group(bot: Bot, msg: Message, dialogue: BotDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please enter a name for the group.")
        .await?;
    dialogue.update(State::CreateGroup).await?;
    Ok(())
}

async fn modify_group(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn add_user(
    bot: Bot,
    msg: Message,
    backend: Arc<SqliteBackend>,
    dialogue: BotDialogue,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please select a group.")
        .await?;
    let Ok(memberships) = backend
        .as_ref()
        .get_membership(msg.chat.id.to_string())
        .await
    else {
        bot.send_message(msg.chat.id, format!("Please select a group."))
            .await?;
        return Ok(());
    };

    let mut groups: Vec<Group> = Vec::new();

    for membership in memberships {
        let Ok(group) = backend.get_group(membership.group_id).await else {
            continue;
        };
        groups.push(group);
    }

    let keyboard_buttons = groups
        .iter()
        .map(|g| InlineKeyboardButton::callback(g.name.to_string(), g.group_id.to_string()));
    bot.send_message(msg.chat.id, "Please select a group.")
        .reply_markup(InlineKeyboardMarkup::new([keyboard_buttons]))
        .await?;
    dialogue.update(State::RecieveGroupToAddUser).await?;
    Ok(())
}

async fn add_expense(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn show_pending(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn settle(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn show_summary(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn show_statement(bot: Bot, msg: Message) -> HandlerResult {
    to_do_message(&bot, msg.chat.id).await?;
    Ok(())
}

async fn to_do_message(bot: &Bot, id: ChatId) -> HandlerResult {
    bot.send_message(id, "This command is not suported yet")
        .await?;
    Ok(())
}

impl From<DataError> for () {
    fn from(value: DataError) -> Self {
        ()
    }
}
/*
For reference -

async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(full_name) => {
            let products = ["Apple", "Banana", "Orange", "Potato"]
                .map(|product| InlineKeyboardButton::callback(product, product));

            bot.send_message(msg.chat.id, "Select a product:")
                .reply_markup(InlineKeyboardMarkup::new([products]))
                .await?;
            dialogue.update(State::ReceiveProductChoice { full_name }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
        }
    }

    Ok(())
}

async fn receive_product_selection(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String, // Available from `State::ReceiveProductChoice`.
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(product) = &q.data {
        bot.send_message(
            dialogue.chat_id(),
            format!("{full_name}, product '{product}' has been purchased successfully!"),
        )
        .await?;
        dialogue.exit().await?;
    }

    Ok(())
}
 */
