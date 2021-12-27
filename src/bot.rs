use crate::services::{config::Config, database::DB};

use events::Handler;
use log::warn;
use serenity::{
    framework::standard::{
        help_commands,
        macros::help,
        Args,
        CommandGroup,
        CommandResult,
        HelpOptions,
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    prelude::*,
    utils::Colour,
    Client,
};
use std::collections::HashSet;
use std::sync::Arc;

mod commands;
pub mod embeds;
mod events;
mod hooks;
mod loops;
mod utils;

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
            c
        })
        .on_dispatch_error(hooks::dispatch_error_hook)
        .group(&commands::general::COMMANDS_GROUP)
        .group(&commands::nasa::NASA_GROUP)
        .group(&commands::get::GET_GROUP)
        .group(&commands::config::CONFIG_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&config.token)
        .framework(framework)
        .event_handler(Handler {
            run_loops: Mutex::new(true),
        })
        .application_id(config.application_id)
        .await
        .expect("Failed to create a new client");

    let db = DB::new(&config.db_uri)
        .await
        .expect("Failed to initialize database");

    db.run_migrations().await.expect("Failed to run migrations");

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(config));
        data.insert::<DB>(Arc::new(db));
    }

    if let Err(e) = client.start_autosharded().await {
        warn!("Failed to login, is the token correct?\n{}", e);
    }
}
