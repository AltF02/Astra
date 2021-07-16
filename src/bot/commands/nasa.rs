mod daily;

use serenity::framework::standard::macros::group;

use self::daily::DAILY_COMMAND;

#[group()]
#[commands(daily)]
pub struct Nasa;
