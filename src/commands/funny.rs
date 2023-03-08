use std::time::Duration;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};

use crate::utils::{checks::check_result, parse::get_repeat_count};

#[group]
#[commands(ba)]
pub struct Funny;

#[command]
#[only_in(guilds)]
pub async fn ba(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = manager.get_or_insert(guild_id);

    let repeat_count = get_repeat_count(args, 6);

    for i in 0..repeat_count {
        let mut handler = handler_lock.lock().await;

        if i > 0 {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        if i % 2 == 0 {
            check_result(handler.join(connect_to).await, "Could not join");
        } else {
            check_result(handler.leave().await, "Could not leave");
        }
    }

    let mut handler = handler_lock.lock().await;
    check_result(handler.join(connect_to).await, "Could not join");

    Ok(())
}
