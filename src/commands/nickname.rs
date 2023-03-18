use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

#[poise::command(slash_command)]
pub async fn nickname(ctx: poise::Context<'_, (), Error>,     
    #[description = "Enter the new status"] status: Option<String>,
    #[description = "Enter the new nickname"] nickname: Option<String>,
) -> Result<(), Error> {
 
    if nickname.is_some() {
        guildid.edit_nickname(ctx.discord(), Some(&nickname)).await?;
        ctx.send(format!("Updated the nickname to {}", nickname)).await?
    }
    let status = format!("{}", something);
        guildid.edit_nickname(ctx.discord(), Some(&status)).await?;

    if status.is_some() {
        serenity::prelude::Context::set_activity(ctx.discord(), Activity::watching(status)).await;
        ctx.send(format!("Updated the status to {}", status)).await?
    }
        
    Ok(())
}