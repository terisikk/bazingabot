use serde::Deserialize;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::env;

use crate::ReqwestClientContainer;

#[derive(Deserialize)]
struct QuoteJson {
    quote: String,
}

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    let quote_url = "https://quotes.teemurisikko.com/api/random_quotes?limit=1";
    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");
    let data = ctx.data.read().await;

    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client.get(quote_url).bearer_auth(token).send().await {
            Ok(resp) => match resp.json::<Vec<QuoteJson>>().await {
                Ok(json) => {
                    msg.channel_id
                        .say(&ctx.http, format!("{}", json[0].quote))
                        .await?;
                }
                Err(_) => println!("ERROR reading quote from {}", quote_url),
            },
            Err(_) => println!("ERROR parsing quote result"),
        }
    } else {
        msg.reply(ctx, "There was a problem getting the reqwest client")
            .await?;
    }
    Ok(())
}
