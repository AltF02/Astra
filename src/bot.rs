use crate::services::{config::Config, database};

use events::Handler;
use log::warn;
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::{framework::standard::StandardFramework, prelude::TypeMapKey, Client};
use std::collections::HashSet;

mod commands;
mod events;
mod loops;
mod utils;

impl TypeMapKey for Config {
    type Value = Config;
}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough commands`~~ are unavailable because the bot is unable to run them."]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough commands`~~ are unavailable because the bot is unable to run them."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
#[wrong_channel = "Strike"]
#[group_prefix = "Prefix commands"]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut ho = help_options.clone();
    ho.embed_error_colour = Colour::from_rgb(255, 30, 30);
    ho.embed_success_colour = Colour::from(0x00adf8);

    let _ = help_commands::with_embeds(ctx, msg, args, &ho, groups, owners).await;
    Ok(())
}

pub async fn start(config: Config) {
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&config.prefix);
            c.allow_dm(true);
            c.case_insensitivity(true);
            c.ignore_bots(false);
            return c;
        })
        .group(&commands::general::COMMANDS_GROUP)
        .group(&commands::nasa::NASA_GROUP)
        .group(&commands::get::GET_GROUP)
        .group(&commands::config::CONFIG_GROUP)
        .help(&MY_HELP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler {
            run_loops: Mutex::new(true),
        })
        .await
        .expect("Failed to create a new client");

    let pool = database::connect(&config.db_uri).await.unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
        data.insert::<database::ConnectionPool>(pool);
    }

    if let Err(e) = client.start_autosharded().await {
        warn!("Failed to login, is the token correct?\n{}", e);
    }
}
