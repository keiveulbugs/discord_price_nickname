use crate::Error;
use crate::Data;

use poise::serenity_prelude::{self as serenit, ChannelId, Attachment};
use serenity::utils::Colour;
use anyhow::Context as _;

/// About command
#[poise::command(slash_command)]
pub async fn icon(ctx: poise::Context<'_, Data, Error>,
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