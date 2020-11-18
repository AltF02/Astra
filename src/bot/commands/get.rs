use serenity::framework::standard::Args;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::services::database::get_launch_database;
use crate::services::ConnectionPool;
use crate::bot::utils::{check_msg, truncate, truncate_string};
use serenity::model::prelude::ReactionType::Unicode;

#[group()]
#[prefixes("get", "search")]
#[commands(rocket, upcoming)]
pub struct Get;

#[command]
async fn rocket(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // let rocket = match args.remains() {
    //     Some(rocket) => rocket,
    //     None => {
    //         msg.reply(&ctx, "Please provide a rocket to look up in the database")
    //             .await?;
    //         return Ok(());
    //     }
    // };
    // let rockets = match get_rocket(rocket).await {
    //     Ok(res) => res,
    //     Err(e) => {
    //         error!("{:?}", e);
    //         return Ok(());
    //     }
    // };
    //
    // if rockets.count == 0 {
    //     check_msg(msg.reply(&ctx, "Unable to locate that rocket").await);
    //     return Ok(());
    // }
    //
    // let rocket = &rockets.rockets[0];
    //
    // let agencies = match get_agency(&rocket.family.as_ref().unwrap().agencies).await {
    //     Ok(res) => res,
    //     Err(e) => {
    //         error!("{:?}", e);
    //         return Ok(());
    //     }
    // };
    // let agency: &Agency = &agencies.agencies[0];
    //
    // check_msg(
    //     msg.channel_id
    //         .send_message(&ctx.http, |m| {
    //             m.embed(|e| {
    //                 e.title(&rocket.name)
    //                     .url(&rocket.wiki_url)
    //                     .image(&rocket.image_url)
    //                     .fields(vec![(
    //                         "Agency",
    //                         format!(
    //                             "\
    //                             ➤ Name: **{}**\n\
    //                             ➤ Country: **{}**\n\
    //                             ",
    //                             &agency.name, &agency.country_code
    //                         ),
    //                         false,
    //                     )])
    //                     .colour(0x00adf8)
    //             })
    //         })
    //         .await,
    // );
    msg.reply(
        &ctx,
        "Sorry not implemented yet due to switching to api 2.0",
    )
    .await?;
    Ok(())
}

#[command]
#[aliases("company")]
async fn agency(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[aliases("upcoming_launch", "next", "launch")]
async fn upcoming(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<ConnectionPool>().unwrap().clone()
    };
    let next_launches = get_launch_database(&pool, false).await;
    let next_launch = match next_launches.get(0) {
        Some(launch) => launch,
        None => {
            msg.reply(&ctx, "Unable to find any in the near future :(").await?;
            return Ok(());
        }
    };

    let mut description = next_launch.description.as_ref().unwrap_or(&"No description found...".to_string()).clone();
    truncate_string(&mut description, 2000);

    check_msg(msg.channel_id.send_message(&ctx.http, |m| { m
        .embed(|e| {e
            .color(0x00adf8)
            .image(&next_launch.image_url.as_ref().unwrap_or(&" ".to_string()))
            .title(&next_launch.name)
            .description(&description)
            .footer(|f| { f
                .text(&next_launch.launch_id)
            })
        })
        .reactions(vec![Unicode("🔔".to_string())])
    }).await);

    Ok(())
}
