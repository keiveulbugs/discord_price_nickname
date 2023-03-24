use crate::Error;
use serenity::utils::Colour;
use serde_derive::Deserialize;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use serenity::model::prelude::Activity;
use anyhow::Context as _;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct L1 {
    pub pairs: Vec<L2>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct L2 {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    pub pair_address: String,
    pub price_native: String,
    pub price_usd: String,
    pub price_change: Change,
    pub liquidity: Value,
    pub volume: Value,
    pub base_token: Name,
    pub quote_token: Name,
    //pub fdv: f64,
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Change {
    pub h24: f64,
    pub h6: f64,
    pub h1: f64,
    pub m5: f64,
}
#[derive(Default, Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Name {
    pub name: String,
    pub symbol: String,
}

pub struct Resultstruct {
    pub name: String,
    pub usd: String,
    pub change: String,
    pub volume: String,
    pub colour: Colour,
    pub urlresult: String,
}


/// Get info on a coin by entering their symbol
#[poise::command(slash_command)]
pub async fn nickname(ctx: poise::Context<'_>,     
    #[description = "Enter the smart contract address of the pair"] address: String,
    #[description = "Enter the chain id according to dexscreener"] chainid: String,
) -> Result<(), Error> {

    let url = format!("https://api.dexscreener.com/latest/dex/pairs/{}/{}", chainid, address);
    
    let filename = format!("bot{}", ctx.framework().bot_id);
    
    if Path::new(&filename).exists() {
        std::fs::remove_file(&filename)?;
    };

    // need to clear all data from file
    let mut envfile = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&filename).map_err(|_| "Couldn't create a new save file or open it")?;

    envfile.write_all(url.as_bytes()).map_err(|_| "Couldn't save the address")?;
    
    ctx.send(|b| b.content("**Saved the token / coin to the config file**").ephemeral(true)).await?;
    
    ctx.send(|b| b.content("**Set nickname and start updating every 5 minutes**").ephemeral(true)).await?;
    let guildid = ctx.guild_id().unwrap();
    'outer: loop {
        if !Path::new(&filename).exists() {
            ctx.send(|b| b.content("Couldn't find an address! No longer updating the nickname").ephemeral(true))
            .await?;
            break 'outer;
        } else {
            let content = std::fs::read_to_string(&filename)?;
            let result = vectorinfo(&content).await?;
            let nickname = format!("{} | {}", result.usd, result.change);
            serenity::prelude::Context::set_activity(ctx.serenity_context(), Activity::watching(result.name)).await;
            guildid.edit_nickname(ctx, Some(nickname.as_str())).await?;
        };
        tokio::time::sleep(std::time::Duration::from_secs(300)).await;


    }




    


    
    Ok(())
}

async fn vectorinfo(url: &str) -> Result<Resultstruct, Error> {
    let v = reqwest::get(url)
        .await
        .map_err(|_| "The dexscreener api can not be reached")?
        .error_for_status()
        .map_err(|_| {
            "This pair can not be retrieved from dexscreener, make sure you write it down correctly"
        })?
        .json::<L1>()
        .await
        .map_err(|_| "Something went wrong with parsing the data")?;
    let w = v.pairs;
    let usd0 = format!("${:.4}", w[0].price_usd.parse::<f64>().unwrap());
    let volume = &w[0].volume["h24"].to_string();
    let name0 = &w[0].base_token.name;
    let change0 = w[0].price_change.h24;
    let changestring = format!("{}%", change0);
    let colour1 = if change0 > 0.0 {
        Colour::DARK_GREEN
    } else if change0 < 0.0 {
        Colour::RED
    } else {
        Colour::GOLD
    };

    let finalstruct = Resultstruct {
        name: name0.to_string(),
        usd: usd0,
        change: changestring,
        volume: volume.to_string(),
        colour: colour1,
        urlresult: w[0].url.to_string(),
    };
    Ok(finalstruct)
}

#[allow(dead_code)]
async fn vectorinfoinverseinbase(url: &str) -> Result<Resultstruct, Error> {
    let v = reqwest::get(url)
        .await
        .map_err(|_| "The dexscreener api can not be reached")?
        .error_for_status()
        .map_err(|_| {
            "This pair can not be retrieved from dexscreener, make sure you write it down correctly"
        })?
        .json::<L1>()
        .await
        .map_err(|_| "Something went wrong with parsing the data")?;
    let w = v.pairs;
    let price0 = w[0].price_native.parse::<f64>().unwrap();
    //let usd0 = w[0].price_usd.parse::<f64>().unwrap();
    //let usd1 = usd0 / price0;
    let name1 = &w[0].quote_token.name;
    let volume = &w[0].volume["h24"].to_string();
    let change0 = w[0].price_change.h24;
    let changestring = format!("{}%", change0);
    let colour1 = if change0 > 0.0 {
        Colour::DARK_GREEN
    } else if change0 < 0.0 {
        Colour::RED
    } else {
        Colour::GOLD
    };

    let finalstruct = Resultstruct {
        name: name1.to_string(),
        usd: format!(
            "{} : {} {}",
            &w[0].base_token.symbol, price0, &w[0].quote_token.symbol
        ),
        volume: volume.to_string(),
        change: changestring,
        colour: colour1,
        urlresult: w[0].url.to_string(),
    };
    Ok(finalstruct)
}
