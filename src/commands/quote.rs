use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::env;

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    let quote_url = "https://quotes.teemurisikko.com/api/random_quotes?limit=1";

    // TODO reuse single client instead for all quotes
    // => Init into global data of context
    // => own client for apina.biz? avoid auth only for quotes

    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");

    // TODO pass via globals
    let client = reqwest::Client::new();
    match client.get(quote_url).bearer_auth(token).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                msg.channel_id.say(&ctx.http, format!("{}", text)).await?;
            }
            Err(_) => println!("ERROR reading {}", quote_url),
        },
        Err(_) => println!("ERROR downloading {}", quote_url),
    }
    Ok(())
}
