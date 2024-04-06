use teloxide::{
    dispatching::{dialogue::{self, InMemStorage}, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::{self, BotCommands},
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveProductChoice {
        full_name: String,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
    #[command(description = "Create a group")]
    CreateGroup,
    #[command(description = "ModifyGroup")]
    ModifyGroup,
    #[command(description = "Add Expense")]
    AddExpense,
    #[command(description = "Show pending settlements")]
    ShowPending,
    #[command(description = "Settle the group")]
    Settle,
    #[command(description = "Show summary so far")]
    ShowSummary,
    #[command(description = "Show statement for past n months")]
    ShowStatement{
        months:u32
    }
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
    .branch(case![Command::Help]).endpoint(help)
    .branch(case![Command::Cancel]).endpoint(cancel)
    .branch(case![Command::CreateGroup]).endpoint(create_group)
    .branch(case![Command::ModifyGroup]).endpoint(modify_group)
    .branch(case![Command::AddExpense]).endpoint(add_expense)
    .branch(case![Command::ShowPending]).endpoint(show_pending)
    .branch(case![Command::Settle]).endpoint(settle)
    .branch(case![Command::ShowSummary]).endpoint(show_summary)
    .branch(case![Command::ShowStatement {months}]).endpoint(show_statement);

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(command_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn create_group(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn modify_group(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn add_expense(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn show_pending(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn settle(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn show_summary(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

async fn show_statement(bot: Bot, msg: Message) -> HandlerResult {
    Ok(())
}

/*

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