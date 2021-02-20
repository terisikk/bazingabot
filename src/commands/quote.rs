use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

use crate::ReqwestClientContainer;

#[command]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    let quote_url = "https://quotes.teemurisikko.com/api/random_quotes?limit=1";
    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");
    let data = ctx.data.read().await;

    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client.get(quote_url).bearer_auth(token).send().await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    msg.channel_id.say(&ctx.http, format!("{}", text)).await?;
                }
                Err(_) => println!("ERROR reading {}", quote_url),
            },
            Err(_) => println!("ERROR downloading {}", quote_url),
        }
    } else {
        msg.reply(ctx, "There was a problem getting the reqwest client")
            .await?;
    }
    Ok(())
}
