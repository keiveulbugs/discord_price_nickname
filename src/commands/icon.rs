use crate::Error;
use poise::serenity_prelude::{Attachment, CacheHttp};
use std::io::Cursor;

#[poise::command(
    slash_command,
    guild_only = true,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn icon(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Upload a logo"] icon: Attachment,
) -> Result<(), Error> {
    let fileurl = &icon.url;
    let response = reqwest::get(fileurl).await?;
    let mut file = std::fs::File::create(&icon.filename)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    let base64 = serenity::utils::read_image(&icon.filename)?;
    let mut user = ctx.cache().unwrap().current_user();
    let _ = user.edit(&ctx, |p| p.avatar(Some(&base64))).await;
    std::fs::remove_file(icon.filename).unwrap();

    ctx.say("Succesfully set avatar").await?;

    Ok(())
}
