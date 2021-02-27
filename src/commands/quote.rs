use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::env;

use crate::ReqwestClientContainer;

#[derive(Deserialize)]
struct QuoteJson {
    quote: String,
}
static NOT_FOUND_REPLY: &str = "???";

#[command]
async fn quote(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut query = String::from("https://quotes.teemurisikko.com/api/random_quotes?limit=1");
    let token = env::var("QUOTEAPI_TOKEN").expect("Expected a token  for quoteapi");
    let arguments = _parse_arguments(args);
    query += &arguments;

    let data = ctx.data.read().await;
    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client.get(&query).bearer_auth(token).send().await {
            Ok(resp) => match resp.json::<Vec<QuoteJson>>().await {
                Ok(json) => {
                    if json.len() == 0 || json[0].quote.len() == 0 {
                        msg.channel_id.say(&ctx.http, NOT_FOUND_REPLY).await?;
                    } else {
                        msg.channel_id
                            .say(&ctx.http, format!("{}", json[0].quote))
                            .await?;
                    }
                }
                Err(_) => println!("ERROR reading quote from {}", query),
            },
            Err(_) => println!("ERROR parsing quote result"),
        }
    } else {
        msg.reply(ctx, "There was a problem getting the reqwest client")
            .await?;
    }
    Ok(())
}

pub async fn quote_semicolon(ctx: &Context, msg: &Message) -> CommandResult {
    let dummy_args = Args::new("", &[Delimiter::Single(' ')]);
    quote(ctx, msg, dummy_args).await?;
    Ok(())
}

fn _parse_arguments(mut args: Args) -> String {
    let mut ret = String::new();
    for arg in args.iter::<String>() {
        match arg {
            Ok(arg) => {
                ret += &_parse_argument(arg);
            }
            Err(_arg) => {
                println!("ERROR: Could not iterate args for rand")
            }
        }
    }
    return ret;
}

fn _parse_argument(mut arg: String) -> String {
    let op = if arg.chars().nth(0) == Some('-') {
        arg.remove(0);
        "not.ilike"
    } else {
        "ilike"
    };
    return format!("&quote={}.*{}*", op, arg).to_string();
}
