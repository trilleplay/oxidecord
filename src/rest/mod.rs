use std::collections::HashMap;

const DISCORD_API_URL: &'static str = "https://discordapp.com/api/v6/";

pub fn findgateway() -> Result<(String), Box<std::error::Error>> {
   // This finds the correct gateway URL, should not be used for bots with 2.5K+ guilds or if the bot owner wants to shard.
   let discord_get_gateway: &'static str = "gateway";
   let api_and_endpoint = format!("{}{}", DISCORD_API_URL, discord_get_gateway);
   let resp: HashMap<String, String> = reqwest::get(&api_and_endpoint)?
       .json()?;
   let gateway = &resp["url"];
   Ok(gateway.to_string())
}
