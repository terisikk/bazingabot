use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::{debug, error};

#[command]
pub async fn rand(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut vec = Vec::new();
    debug!("rand requested");
    for arg in args.iter::<String>() {
        match arg {
            Ok(arg) => {
                vec.push(arg);
            }
            Err(_arg) => {
                error!("ERROR: Could not iterate args for rand")
            }
        }
    }
    let result = vec.choose(&mut rand::thread_rng());
    if let Some(res) = result {
        msg.channel_id.say(&ctx.http, format!("{}", res)).await?;
    }

    Ok(())
}
