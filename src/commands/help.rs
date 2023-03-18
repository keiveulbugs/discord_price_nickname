use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| b.description(
            "This bot is an example bot and sings a beautiful song for you!
            beep boop beep boop lorem ipsum lalalala"
            ).title("help").colour(Colour::BLITZ_BLUE))
            .ephemeral(true)
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| {
                        b.label("Discord.com")
                            .url("https://discord.com/")
                            .style(serenit::ButtonStyle::Link)                        
                    })                                   
                })
            })          
    })
    .await?;
    //When the message is sent in your private channel, return the option to deregister the bot.
    // Change the channelid to your id
    if ctx.channel_id() == ChannelId(123456789123456789) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}