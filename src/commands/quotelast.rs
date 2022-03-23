use serde::Serialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::{debug, error};

use std::env;

use crate::ReqwestClientContainer;

#[derive(Serialize)]
struct QuoteJson {
    a_channel: String,
    a_victim: String,
    a_adder: String,
    a_quote: String,
}
static NOT_FOUND_REPLY: &str = "???";

#[command]
async fn quotelast(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if msg.kind != MessageType::InlineReply {
        msg.reply(ctx, "Quotelast only works with reply").await?;
    };

    debug!("quotelast requested");
    let mut query = String::from("https://quotes.teemurisikko.com/rpc/quotelast");
    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");

    let quoteobj = QuoteJson {
        a_channel: msg.referenced_message.channel_id.name(),
        a_victim: msg.referenced_message.author.name,
        a_adder: msg.author.name,
        a_quote: msg.content,
    };

    let data = ctx.data.read().await;
    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client.post(&query).json(&quoteobj).bearer_auth(token).send().await {
            Ok(resp) => match resp.json::<Vec<QuoteJson>>().await {
                Err(e) => error!("ERROR quotelasting, {} ", e),
            },
            Err(e) => error!("ERROR parsing quotelast result, error: {}", e),
        }
    } else {
        msg.reply(ctx, "There was a problem getting the reqwest client")
            .await?;
    }
    Ok(())
}
