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

use std::sync::Arc;

use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::{self, BotCommands},
};

use crate::model::{datamodel::{Datamodel, User}, sqlite::backend::{self, SqliteBackend}};

use super::state::{State, UserData};

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn user_schemas()-> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>{
    use dptree::case;

    case![State::RegisterUser].endpoint(register_name)
}

pub fn user_callback_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    case![State::ConfirmUser { data }].endpoint(confirm_user)
}

async fn register_name(bot: Bot, msg: Message, dialogue: BotDialogue) -> HandlerResult {
    let Some(name) = msg.text() else {
        bot.send_message(msg.chat.id, "Please enter a name.")
            .await?;
        return Ok(());
    };

    let Some(user_name) = msg.chat.username() else {
        bot.send_message(msg.chat.id, "Username not found.").await?;
        return Ok(());
    };

    let options =
        ["Accept", "Edit", "Cancel"].map(|option| InlineKeyboardButton::callback(option, option));

    bot.send_message(
        msg.chat.id,
        format!("Please confirm your details:\nName: {name}\nUserId: {user_name}"),
    )
    .reply_markup(InlineKeyboardMarkup::new([options]))
    .await?;

    dialogue
        .update(State::ConfirmUser {
            data: UserData {
                name: name.to_string(),
                username: user_name.to_string(),
            },
        })
        .await?;
    Ok(())
}

async fn confirm_user(
    bot: Bot,
    dialogue: BotDialogue,
    data: UserData,
    query: CallbackQuery,
    backend: Arc<SqliteBackend>
) -> HandlerResult {
    if let Some(option) = query.data {
        match option.as_str() {
            "Edit" => {
                bot.send_message(dialogue.chat_id(), "Please enter your name.")
                    .await?;
                dialogue.update(State::RegisterUser).await?;
            }
            "Accept" => {
                bot.send_message(dialogue.chat_id(), "Thank you for confirming your details.")
                    .await?;
                let user = User {
                    name: data.username,
                    user_id: dialogue.chat_id().to_string(),
                    username: data.name
                };
                match backend.as_ref().add_user(user).await {
                    Ok(_) => {
                        dialogue.update(State::Start).await?;
                        bot.send_message(dialogue.chat_id(), "Successfully registered")
                        .await?;
                    },
                    Err(e) => {
                        bot.send_message(dialogue.chat_id(), format!("Failed to register.")).await?;
                    }
                }
            }
            "Cancel" => {
                bot.send_message(dialogue.chat_id(), "Canceled registration.")
                    .await?;
                dialogue.update(State::Start).await?;
            }
            _ => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Invalid input. Please try again or use /cancel to go back to main menu.",
                )
                .await?;
                dialogue.update(State::RegisterUser).await?;
            }
        }
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Callback query not found. Please try again. Or /cancel to go back to main menu.",
        )
        .await?;
    }
    Ok(())
}
