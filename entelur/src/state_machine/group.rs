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

use std::{error::Error, fmt::Debug, sync::Arc};

use teloxide::{
    dispatching::{
        dialogue::{self, GetChatId, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, True},
    utils::command::{self, BotCommands},
};
use tokio::runtime::Handle;

use crate::model::{
    datamodel::{Datamodel, Group, GroupId},
    sqlite::backend::SqliteBackend,
};

use super::state::State;

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type TelHandler<'a> = Handler<
    'a,
    DependencyMap,
    Result<(), Box<dyn Error + Sync + Send>>,
    teloxide::dispatching::DpHandlerDescription,
>;

pub fn create_group_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    dptree::entry()
        .branch(case![State::CreateGroup].endpoint(create_group))
        .branch(case![State::RecieveGroupDescription { group }].endpoint(recieve_group_description))
}

pub fn create_group_callback_schema(
) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    dptree::entry()
        .branch(case![State::ConfirmGroup { group }].endpoint(confirm_group))
        .branch(case![State::RecieveGroupToAddUser].endpoint(recieve_group_user_add))
}

async fn check_group_state(dialog: BotDialogue) -> Option<State> {
    let Ok(state) = dialog.get().await else {
        return None;
    };
    state
}

async fn create_group(bot: Bot, msg: Message, dialogue: BotDialogue) -> HandlerResult {
    if let Some(name) = msg.text() {
        let group = Group {
            name: name.to_string(),
            description: "".to_string(),
            created_by: msg.chat.id.to_string(),
            group_id: 0,
        };
        bot.send_message(msg.chat.id, "Please enter the description of the group.")
            .await?;
        dialogue
            .update(State::RecieveGroupDescription { group })
            .await?;
    } else {
        bot.send_message(msg.chat.id, "Please enter a name for the group.")
            .await?;
    }
    Ok(())
}

async fn recieve_group_description(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue,
    group: Group,
) -> HandlerResult {
    if let Some(description) = msg.text() {
        let mut group = group;
        group.description = description.to_string();

        let options = ["Confirm", "Edit", "Cancel"]
            .map(|option| InlineKeyboardButton::callback(option, option));

        bot.send_message(
            msg.chat.id,
            format!(
                "Confirm your group details:\nName: {}\nDescription: {}",
                group.name, group.description
            ),
        )
        .reply_markup(InlineKeyboardMarkup::new([options]))
        .await?;
        dialogue.update(State::ConfirmGroup { group }).await?;
    } else {
        bot.send_message(msg.chat.id, "Please enter a description for the group.")
            .await?;
    }
    Ok(())
}

async fn confirm_group(
    bot: Bot,
    dialogue: BotDialogue,
    group: Group,
    q: CallbackQuery,
    backend: Arc<SqliteBackend>,
) -> HandlerResult {
    let Some(option) = q.data else {
        bot.send_message(
            dialogue.chat_id(),
            "Callback query not found. Please try again. Or /cancel to go back to main menu.",
        )
        .await?;
        return Ok(());
    };

    match option.as_str() {
        "Confirm" => {
            match backend.as_ref().add_group(group).await {
                Ok(_) => {
                    dialogue.update(State::Start).await?;
                    bot.send_message(dialogue.chat_id(), "Group created successfully.")
                        .await?;
                }
                Err(e) => {
                    dbg!(e);
                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Error creating group. Please try again later."),
                    )
                    .await?;
                }
            }
        }
        "Edit" => {
            dialogue.update(State::CreateGroup).await?;
            bot.send_message(dialogue.chat_id(), "Please enter the name of the group.")
                .await?;
        }
        "Cancel" => {
            dialogue.update(State::Start).await?;
            bot.send_message(dialogue.chat_id(), "Canceled group creation.")
                .await?;
        }
        _ => {}
    };

    Ok(())
}

async fn recieve_group_user_add(
    bot: Bot,
    dialogue: BotDialogue,
    group: Group,
    q: CallbackQuery,
    backend: Arc<SqliteBackend>,
) -> HandlerResult {
    let Some(group_id) = q.data else {
        bot.send_message(
            dialogue.chat_id(),
            "Didn't find group in response. Please try again",
        )
        .await?;
        dialogue.update(State::Start).await?;
        return Ok(());
    };

    let group_id: GroupId = group_id.parse()?;
    let Ok(group) = backend.get_group(group_id).await else {
        bot.send_message(
            dialogue.chat_id(),
            "Didn't find group in database. Please try again",
        )
        .await?;
        dialogue.update(State::Start).await?;
        return Ok(());
    };

    bot.send_message(
        dialogue.chat_id(),
        "Please select a user to add to the group.",
    )
    .await?;
    dialogue.update(State::RecieveUserToAdd { group }).await?;

    Ok(())
}
