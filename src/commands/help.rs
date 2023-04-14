use crate::Error;
use serenity::utils::Colour;
use poise::serenity_prelude::UserId;

/// About command
#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| {
            b.description("This bot sets prices as nickname")
                .title("help")
                .colour(Colour::BLITZ_BLUE)
        })
        .ephemeral(true)
    })
    .await?;
    // Change this id to the user that needs permissions to change the id.
    if ctx.author().id == UserId(397118394714816513) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}
