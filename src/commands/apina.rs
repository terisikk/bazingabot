use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ReqwestClientContainer;

#[command]
async fn apina(ctx: &Context, msg: &Message) -> CommandResult {
    let apina_url = "https://m.apina.biz/random";
    let data = ctx.data.read().await;

    if let Some(client) = data.get::<ReqwestClientContainer>() {
        match client.get(apina_url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    if let Some(image_url) = get_image_url(&text) {
                        msg.channel_id
                            .say(&ctx.http, format!("{}", &image_url))
                            .await?;
                    }
                }
                Err(_) => println!("ERROR reading {}", apina_url),
            },
            Err(_) => println!("ERROR downloading {}", apina_url),
        }
    }
    Ok(())
}

fn get_image_url(string: &str) -> Option<String> {
    use regex::Regex;
    let re = Regex::new(r"(https://images.apina.biz/full/)[a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4}(/\S*)?")
        .unwrap();
    if let Some(url) = re.captures(string) {
        if url.len() == 0 {
            return None;
        }
        return Some(url[0].to_string());
    }

    return None;
}
