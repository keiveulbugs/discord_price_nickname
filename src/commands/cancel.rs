use crate::Error;
use anyhow::Context as _;

#[poise::command(slash_command)]
pub async fn cancel(ctx: poise::Context<'_>) -> Result<(), Error> {
    let filename = format!("bot{}", ctx.framework().bot_id);
    
    if std::path::Path::new(&filename).exists() {
        std::fs::remove_file(&filename)?;
        ctx.send(|b| b.content("**Deleted the config file. The bot will stop updating its name at the next iteration.**").ephemeral(true)).await?;

    } else {
        ctx.send(|b| b.content("**There doesn't seem to be a config file for the bot.**").ephemeral(true)).await?;
    };
    Ok(())
}