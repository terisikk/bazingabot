use riven::consts::Region;
use riven::RiotApi;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;
use tracing::{debug, info, warn};

#[command]
async fn masteries(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("masteries requested");
    let api_key = env::var("RIOTAPI_TOKEN").expect("Expected a token  for riot api");
    let riot_api = RiotApi::with_key(api_key);

    let summoner_name = _parse_summoner_name(&mut args);
    if summoner_name == None {
        msg.channel_id
            .say(&ctx.http, format!("Summoner name?"))
            .await?;
        info!("summoner name missing from mastery query");
        return Ok(());
    }
    let mastery_count = _parse_mastery_count(&mut args);

    // Get summoner data.
    let _ = match riot_api
        .summoner_v4()
        .get_by_summoner_name(Region::EUNE, &summoner_name.clone().unwrap())
        .await?
    {
        Some(summoner) => {
            // Get champion mastery data.
            let masteries = riot_api
                .champion_mastery_v4()
                .get_all_champion_masteries(Region::EUNE, &summoner.id)
                .await
                .expect("Get champion masteries failed.");

            // Print champioon masteries.
            let row_count = std::cmp::max(1, std::cmp::min(10, mastery_count.unwrap_or(5)));
            let mut res = summoner.name.to_string() + &format!(" top {} masteries:\n", row_count);

            for (i, mastery) in masteries[..row_count as usize].iter().enumerate() {
                res.push_str(&format!(
                    "{: >2}) {: <9}    {: >7} ({})\n",
                    i + 1,
                    mastery.champion_id.to_string(),
                    mastery.champion_points,
                    mastery.champion_level,
                ));
            }
            msg.channel_id.say(&ctx.http, res).await?;
        }
        None => {
            info!(
                "Summoner {} not found from eune",
                summoner_name.clone().unwrap()
            );
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Could not find summoner with name {}",
                        summoner_name.clone().unwrap()
                    ),
                )
                .await?;
        }
    };

    Ok(())
}

#[command]
async fn f2p(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("masteries requested");
    let api_key = env::var("RIOTAPI_TOKEN").expect("Expected a token  for riot api");
    let riot_api = RiotApi::with_key(api_key);

    // Get summoner data.
    let _ = match riot_api.champion_v3().get_champion_info(Region::EUNE).await {
        Ok(champion_info) => {
            let ids = champion_info.free_champion_ids;
            let mut res = "".to_string();

            for (_, champ) in ids.iter().enumerate() {
                res.push_str(&format!("{}, ", champ.to_string()));
            }
            // Maybe not the most idiomatic way -> learn & fix
            res.pop();
            res.pop();
            msg.channel_id.say(&ctx.http, res).await?;
        }
        Err(e) => {
            warn!("Champion info not found from eune: {}", e);
            msg.channel_id
                .say(&ctx.http, format!("Could not find champion info"))
                .await?;
        }
    };

    Ok(())
}

fn _parse_summoner_name(args: &mut Args) -> Option<String> {
    return args.single::<String>().ok();
}

fn _parse_mastery_count(args: &mut Args) -> Option<u32> {
    return args.single::<u32>().ok();
}
