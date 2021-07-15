mod guilds;
mod leave;
mod ping;
mod prefix;

use serenity::framework::standard::macros::group;

use self::guilds::GUILDS_COMMAND;
use self::leave::LEAVE_COMMAND;
use self::ping::PING_COMMAND;
use self::prefix::PREFIX_COMMAND;

#[group()]
#[commands(ping, prefix, guilds, leave)]
pub struct Commands;
