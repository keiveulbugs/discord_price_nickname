use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId, Attachment, CacheHttp, Context};
use serenity::utils::Colour;
use std::io;
use std::fs::File;
use std::io::Cursor;
use poise::serenity_prelude::EditProfile;


/// About command
#[poise::command(slash_command)]
pub async fn icon(ctx: poise::Context<'_, (), Error>,
#[description = "Upload a logo"] icon: Attachment,
) -> Result<(), Error> {
    //ctx.send("hello").await?;
   

    let fileurl = &icon.url;
    let response = reqwest::get(fileurl).await?;
    let mut file = std::fs::File::create(&icon.filename)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    let base64 = serenity::utils::read_image(&icon.filename)?;
    let mut user = ctx.cache().unwrap().current_user();
    let _ = user.edit(&ctx, |p| p.avatar(Some(&base64))).await;
    std::fs::remove_file(icon.filename).unwrap();
    
    ctx.say("Succesfully set avatar").await?;

    Ok(())
}