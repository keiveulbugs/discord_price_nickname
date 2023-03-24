use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId, Attachment};
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn icon(ctx: poise::Context<'_, (), Error>,
#[description = "Enter the smart contract address of the pair"] icon: Attachment,
) -> Result<(), Error> {
    //ctx.send("hello").await?;

    let file = &icon.url;
    let bitsandbytes = reqwest::get(file).await?;
    let bitsies = bitsandbytes.bytes().await?;
    println!("{}", file);
    println!("{:#?}", icon);

    Ok(())
}