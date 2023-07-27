mod commands;
use poise::serenity_prelude::GuildId;
use poise::serenity_prelude::{self as serenity, ChannelId};
use std::io::BufWriter;
use std::io::Write;
use std::sync::atomic::AtomicBool;
type Error = Box<dyn std::error::Error + Send + Sync>;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;
use serde::Deserialize;
use ron::de::from_reader;
use tracing::Level;

use crate::commands::nickname::vectorinfo;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    address: String,
    server: GuildId,
    channel: u64,
    chain : String,
}


#[macro_use]
//.env variables
extern crate dotenv_codegen;

pub static STOPBOOL: AtomicBool = AtomicBool::new(false);

//Constants
// Your Bot token
const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
// If you want to have commands specific to only a specific guild, set this as your guild_id.
const PRIVATEGUILDID: serenity::GuildId = serenity::GuildId(1234567891234567891);

async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {
    // To announce that the bot is online.
    println!("{} is connected!", ready.user.name);
    let filename = format!("{}.ron", ready.user.name);


    if std::path::Path::exists(std::path::Path::new(&filename)) {
        let f = std::fs::File::open(filename).expect("Failed opening file");
        let config: Config = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
    
                std::process::exit(1);
            }
        };
        let url = format!("https://api.dexscreener.com/latest/dex/pairs/{}/{}", config.chain, config.address);


        let chanid = ChannelId(config.channel);
        let guild = config.server;
        chanid.send_message(ctx.http.clone(), |f| {f.content("*The bot restarted*")}).await?;

        'outer: loop {
            if STOPBOOL.load(std::sync::atomic::Ordering::Relaxed) {
                chanid.send_message(ctx.clone().http, |f| {f.content("Stopped updating the nickname of the bot")}).await?;
                
                
                // ("Stopped updating the nickname of the bot").await?;
                break 'outer;
            };
    
            let resultresult = vectorinfo(&url).await;
            match resultresult {
                Ok(result) => {
                    let nickname = format!("{} | {}", result.usd, result.change);
                    // serenity::prelude::Context::set_activity(ctx.serenity_context(), Activity::watching(result.name)).await;
                    poise::serenity_prelude::Context::set_activity(ctx, serenity::model::gateway::Activity::watching(result.name)).await;
                    match guild.edit_nickname(ctx, Some(nickname.as_str())).await {
                        Ok(val) => val,
                        Err(_) => {tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                            continue 'outer;},
                    };
                    tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                },
                Err(_) => {tokio::time::sleep(std::time::Duration::from_secs(300)).await;}
            };
    
    
        }
    



    } else {
        println!("No preset config");
    }
    


    // This registers commands for the bot, guild commands are instantly active on specified servers
    //
    // The commands you specify here only work in your own guild!
    // This is useful if you want to control your bot from within your personal server,
    // but dont want other servers to have access to it.
    // For example sending an announcement to all servers it is located in.
    let builder = poise::builtins::create_application_commands(&framework.options().commands);
    let commands =
        serenity::GuildId::set_application_commands(&PRIVATEGUILDID, &ctx.http, |commands| {
            *commands = builder.clone();

            commands
        })
        .await;
    // This line runs on start-up to tell you which commands succesfully booted.
    // println!(
    //     "I now have the following guild slash commands: \n{:#?}",
    //     commands
    // );

    // Below we register Global commands, global commands can take some time to update on all servers the bot is active in
    //
    // Global commands are availabe in every server, including DM's.
    // We call the commands folder, the ping file and then the register function.
    let global_command1 =
        serenity::Command::set_global_application_commands(&ctx.http, |commands| {
            *commands = builder;
            commands
        })
        .await;
    // println!(
    //     "I now have the following guild slash commands: \n{:#?}",
    //     global_command1
    // );

    Ok(())
}

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {

    // let subscriber = tracing_subscriber::FmtSubscriber::builder()
    // .with_max_level(Level::DEBUG)
    // .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    // Build our client.
    let client = poise::Framework::builder()
        .token(DISCORD_TOKEN)
        .intents(serenity::GatewayIntents::GUILDS)
        .options(poise::FrameworkOptions {
            commands: vec![
                // Do not remove the help command,
                // it uses that line to place in new commands at the right position.
                // Might change this in the future, but am lazy and this was the easiest.
                commands::nickname::nickname(),
                commands::cancel::cancel(),
                commands::icon::icon(),
                // commands::nicknamefunction::nicknamefunction(),
                commands::help::help(),
            ],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

    // Start client, show error, and then ask user to provide bot secret as that is the most common cause for failure
    if let Err(why) = client.start().await {
        println!("Client error: {:?}\n\n**********************\nTry entering a working bot-secret in the .env file", why);
    }
}
