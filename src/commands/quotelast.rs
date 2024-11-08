use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::{debug, error};

use std::env;

use crate::ReqwestClientContainer;

#[derive(Deserialize, Serialize)]
struct QuoteJson {
    a_channel: String,
    a_victim: String,
    a_adder: String,
    a_quote: String,
}

#[command]
async fn quotelast(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.kind != MessageType::InlineReply {
        msg.reply(ctx, "Quotelast only works with reply").await?;
    };

    debug!("quotelast requested");
    let query = String::from("https://quotes.teemurisikko.com/api/rpc/quotelast");
    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");

    let msg_ref = &*msg.referenced_message.as_ref().unwrap();
    let quoteobj = QuoteJson {
        a_channel: msg_ref.channel_id.name(&ctx.cache).await.unwrap(),
        a_victim: msg_ref.author.name.clone(),
        a_adder: msg.author.name.clone(),
        a_quote: msg_ref.content.clone(),
    };

    let data = ctx.data.read().await;
    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client
            .post(&query)
            .json(&quoteobj)
            .bearer_auth(token)
            .send()
            .await
        {
            Ok(_) => (),
            Err(e) => error!("ERROR parsing quotelast result, error: {}", e),
        }
    } else {
        msg.reply(ctx, "There was a problem getting the reqwest client")
            .await?;
    }
    Ok(())
}
