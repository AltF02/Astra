use crate::extensions::*;
use serenity::framework::standard::{macros::hook, DispatchError, Reason};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt;

struct DispatchWrapper(DispatchError);

#[hook]
pub async fn dispatch_error_hook(
    ctx: &Context,
    msg: &Message,
    err: DispatchError,
    _command_name: &str,
) {
    let _ = msg.reply_error(ctx, DispatchWrapper(err)).await;
}

impl fmt::Display for DispatchWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_display = match &self.0 {
            DispatchError::CheckFailed(_required, reason) => match reason {
                Reason::User(reason)
                | Reason::UserAndLog {
                    user: reason,
                    log: _,
                } => reason.to_string(),
                _ => "You're not allowed to run this command".to_string(),
            },
            DispatchError::Ratelimited(i) => format!("Got rate-limited: {:?}", i),
            DispatchError::CommandDisabled => "This command is disabled".to_string(),
            DispatchError::BlockedUser => "User is not permitted to use this bot".to_string(),
            DispatchError::BlockedGuild => "Guild is blocked by this bot".to_string(),
            DispatchError::BlockedChannel => "Channel is blocked by this bot".to_string(),
            DispatchError::OnlyForDM => "Command may only be used in DMs".to_string(),
            DispatchError::OnlyForGuilds => "Command may only be used in guilds".to_string(),
            DispatchError::OnlyForOwners => "Command may only be used by owners".to_string(),
            DispatchError::LackingRole => "Missing a required role".to_string(),
            DispatchError::LackingPermissions(p) => {
                format!("User is missing permissions, required permissions is {}", p)
            }
            DispatchError::NotEnoughArguments { min, given } => format!(
                "Missing required arguments, expected {} but got {}",
                given, min
            ),
            DispatchError::TooManyArguments { max, given } => format!(
                "Received too many arguments, expected {} but got {}",
                given, max
            ),
            _ => {
                log::warn!("Unhandled dispatch error: {:?}", self.0);
                format!(
                    "Something went wrong, `{:?}`.\nThe owner has been notified",
                    self.0
                )
            }
        };

        write!(f, "{}", error_display)
    }
}
