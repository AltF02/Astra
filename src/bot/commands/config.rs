mod channel;
mod info;
mod set;

use crate::bot::utils::Utils;
use crate::services::config::Config as BotConfig;
use crate::services::database::DBGuild;

use self::channel::CHANNEL_COMMAND;
use self::info::CONFIG_INFO_COMMAND;
use self::set::SET_COMMAND;

use serenity::model::prelude::*;
use serenity::{framework::standard::macros::group, model::channel::Message, prelude::*};

pub fn format_setting(setting: bool, name: &str, config: &BotConfig) -> String {
    let emote = if setting {
        &config.emotes.enabled
    } else {
        &config.emotes.disabled
    };
    format!("{} **{}**\n", emote, name)
}

async fn send_settings(guild_db: &DBGuild, msg: &Message, ctx: &Context, guild: &Guild) {
    let data = ctx.data.read().await;
    let config = data.get::<BotConfig>().unwrap();
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

    Utils::check_msg(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Guild settings")
                        .description(settings)
                        .footer(|f| {
                            f.text(&guild.name)
                                .icon_url(&guild.icon_url().unwrap_or_else(|| " ".to_string()))
                        })
                        .color(0x00adf8)
                })
            })
            .await,
    );
}

#[group()]
#[prefixes("config", "update")]
#[commands(channel, set)]
#[default_command(config_info)]
#[required_permissions(MANAGE_CHANNELS)]
pub struct Config;
