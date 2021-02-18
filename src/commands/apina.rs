use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn apina(ctx: &Context, msg: &Message) -> CommandResult {
    let origin_url = "https://m.apina.biz/random";

    // TODO reuse single client instead
    // => Init into global data of context
    match reqwest::get(origin_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                if let Some(image_url) = get_image_url(&text) {
                    msg.channel_id
                        .say(&ctx.http, format!("{}", &image_url))
                        .await?;
                }
            }
            Err(_) => println!("ERROR reading {}", origin_url),
        },
        Err(_) => println!("ERROR downloading {}", origin_url),
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
