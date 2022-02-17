mod channel;
mod info;
mod set;

use crate::services::config::Config as BotConfig;
use crate::services::database::guild::DBGuild;

use self::channel::CHANNEL_COMMAND;
use self::info::CONFIG_INFO_COMMAND;
use self::set::SET_COMMAND;

use crate::extensions::context::ClientContextExt;
use crate::extensions::MessageExt;
use anyhow::Result;
use serenity::{framework::standard::macros::group, model::channel::Message, prelude::*};

pub fn format_setting(setting: bool, name: &str, config: &BotConfig) -> String {
    let emote = if setting {
        &config.emotes.enabled
    } else {
        &config.emotes.disabled
    };
    format!("{} **{}**\n", emote, name)
}

async fn send_settings(guild_db: &DBGuild, msg: &Message, ctx: &Context) -> Result<()> {
    let config = ctx.get_config().await;
    let mut settings: String = "".to_string();

    settings.push_str(format_setting(guild_db.launches, "Launches", &config).as_str());
    settings.push_str(
        format_setting(
            guild_db.apod,
            "APOD (Astronomy Picture of the Day)",
            &config,
        )
        .as_str(),
    );
    settings.push_str(format_setting(guild_db.events, "Events", &config).as_str());

    msg.reply_success(ctx, settings).await?;

    Ok(())
}

#[group()]
#[prefixes("config", "update")]
#[commands(channel, set)]
#[default_command(config_info)]
#[required_permissions(MANAGE_CHANNELS)]
pub struct Config;
