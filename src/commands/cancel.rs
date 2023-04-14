use crate::{Error, STOPBOOL};
use std::sync::atomic::Ordering::Relaxed;

// This command stops the bribe checking
#[poise::command(
    slash_command,
    guild_only = true,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn cancel(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    STOPBOOL.swap(true, Relaxed);
    ctx.send(|b| b.content("Stopping the nickname-bot. If everything goes well, you should soon see a message that the bot was stopped originating from the other command.").ephemeral(true)).await?;
    Ok(())
}
