use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::{self, BotCommands},
};

use super::state::State;

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn user_schemas()-> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>{
    use dptree::case;

    case![State::RegisterUser].endpoint(register_name)
}

pub fn user_callback_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    case![State::ConfirmUser { user_name }].endpoint(confirm_user)
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
            user_name: user_name.to_string(),
        })
        .await?;
    Ok(())
}

async fn confirm_user(
    bot: Bot,
    dialogue: BotDialogue,
    user_name: String,
    query: CallbackQuery,
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
                dialogue.update(State::Start).await?;
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
