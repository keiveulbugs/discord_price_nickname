use crate::Error;
use poise::serenity_prelude::ChannelId;
use serenity::utils::Colour;

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
    //When the message is sent in your private channel, return the option to deregister the bot.
    // Change the channelid to your id
    if ctx.channel_id() == ChannelId(1086782678826762241) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}
