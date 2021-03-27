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
                    if let Some(image_url) = _get_image_url(&text) {
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

fn _get_image_url(string: &str) -> Option<String> {
    use regex::Regex;
    let re =
        Regex::new(r"(https://images.apina.biz/full/)[a-zA-Z0-9\-\.]+\.[a-zA-Z3-4_.-]{2,4}(/\S*)?")
            .unwrap();
    if let Some(url) = re.captures(string) {
        if url.len() == 0 {
            return None;
        }
        return Some(url[0].to_string());
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test apina query itself?
    // -> it's possible to mock the http via mockito
    // -> it's possible to store trait into ReqwestClientContainer instead of actual client
    // .. however, how to take control of the Message and set expectations?

    #[test]
    fn test_get_image_url_parse() {
        let expected = "https://images.apina.biz/full/12345.jpg";
        let content =
            "asd lol <img> header// ding\\ dong/\tp </img>https://images.apina.biz/full/12345.jpg *.jpg";
        assert_eq!(expected, _get_image_url(content).unwrap());
    }

    #[test]
    fn test_get_image_url_parse_mp4() {
        let expected = "https://images.apina.biz/full/12345.mp4";
        let content =
            "asd lol <img> header// ding\\ dong/\tp jpg </img>https://images.apina.biz/full/12345.mp4 asd njdkf *.jpg";
        assert_eq!(expected, _get_image_url(content).unwrap());
    }

    #[test]
    fn test_get_image_url_malformed_parse() {
        let content =
            "asd lol <img> header// ding\\ dong/\tp htt://images.apina.biz/full/12345.jpg *.jpg";
        assert_eq!(None, _get_image_url(content));
    }
}
