use std::{thread, time::Duration};

use rand::Rng;
use serenity::{
    builder,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};
use songbird::input::{self, Restartable};

use crate::{
    cache::app_cache::AppCacheKey,
    utils::{
        checks::check_msg,
        parse::{get_repeat_count, get_sound_path},
    },
};

#[group]
#[commands(play, queue, skip, stop, sound, sounds, spam, siren)]
pub struct Sounds;

#[command]
#[only_in(guilds)]
pub async fn sound(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let file = match args.single_quoted::<String>() {
        Ok(file) => file,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(ctx, "Sound name required! Use the \"sounds\" command to get a list of all the sounds.")
                    .await,
            );

            return Ok(());
        }
    };
    if file.starts_with('/')
        || file.starts_with('\\')
        || file.starts_with('.')
        || file.starts_with('~')
    {
        check_msg(msg.channel_id.say(ctx, "uhhhh no").await);
        return Ok(());
    }

    let path = get_sound_path(file.as_str());

    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match input::ffmpeg(path.clone()).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(ctx, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        // This handler object will allow you to, as needed,
        // control the audio track via events and further commands.
        handler.play_only_source(source);
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn sounds(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let page_index = args.single::<usize>().unwrap_or(0);

    if page_index == 0 {
        check_msg(
            msg.channel_id
                .say(ctx, "Command requires a page index greater than 0.")
                .await,
        );

        return Ok(());
    }

    let ctx_data = ctx.data.read().await;

    let sound_page = match ctx_data.get::<AppCacheKey>() {
        Some(app_cache) => app_cache.sounds_pages.get_page(page_index - 1),
        None => {
            println!("App cache not found.");
            return Ok(());
        }
    };

    if sound_page.is_none() {
        check_msg(msg.channel_id.say(ctx, "Page not found").await);

        return Ok(());
    }

    let mut embed = builder::CreateEmbed::default();

    let embed_text = sound_page.unwrap().join("\n");
    embed.field(page_index.to_string(), embed_text, true);

    check_msg(
        msg.channel_id
            .send_message(ctx, |m| m.set_embed(embed))
            .await,
    );

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn spam(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let file = match args.single_quoted::<String>() {
        Ok(file) => file,
        Err(_) => {
            check_msg(msg.channel_id.say(ctx, "Not a string???").await);

            return Ok(());
        }
    };

    if file.starts_with('/')
        || file.starts_with('\\')
        || file.starts_with('.')
        || file.starts_with('~')
    {
        check_msg(msg.channel_id.say(ctx, "uhhhh no").await);

        return Ok(());
    }

    let path = get_sound_path(file.as_str());

    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let repeat_count = get_repeat_count(args, 10);

        for _ in 0..repeat_count {
            let source = input::ffmpeg(path.clone()).await.unwrap();
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(25..125)));
            handler.play_source(source);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn siren(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let option = match args.single::<String>() {
        Ok(option) => option,
        Err(_) => {
            check_msg(msg.reply(ctx, "Not even a string???😤😤😤").await);
            return Ok(());
        }
    };
    for c in option.chars() {
        if c.is_numeric() {
            check_msg(
                msg.reply(ctx, "Usage: .siren <tense|taci> <repeat count>")
                    .await,
            );
            return Ok(());
        }
    }
    if !(option.eq(&"taci".to_string()) || option.eq(&"tense".to_string())) {
        check_msg(msg.reply(ctx, "Stiu doar tense si taci").await);
        return Ok(());
    }
    let pathl = get_sound_path(format!("{}l.ogg", option).as_str());
    let pathr = get_sound_path(format!("{}r.ogg", option).as_str());
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let repeat_count = get_repeat_count(args, 10);

        // This handler object will allow you to, as needed,
        // control the audio track via events and further commands.
        let mut use_left_path = false;

        for _ in 0..repeat_count {
            use_left_path = !use_left_path;

            let path = if use_left_path {
                pathl.clone()
            } else {
                pathr.clone()
            };

            let source = match input::ffmpeg(path).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);
                    check_msg(msg.channel_id.say(ctx, "Error sourcing ffmpeg").await);
                    return Ok(());
                }
            };

            handler.enqueue_source(source);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(ctx, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(ctx, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match input::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(ctx, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        handler.enqueue_source(source);

        check_msg(msg.channel_id.say(ctx, "Playing song").await);
    } else {
        check_msg(
            msg.channel_id
                .say(ctx, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(ctx, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(ctx, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(ctx, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        handler.enqueue_source(source.into());

        check_msg(
            msg.channel_id
                .say(
                    ctx,
                    format!("Added song to queue: position {}", handler.queue().len()),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(ctx, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        check_msg(
            msg.channel_id
                .say(ctx, format!("Song skipped: {} in queue.", queue.len()))
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(ctx, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.stop();
        let queue = handler.queue();
        queue.stop();

        check_msg(msg.channel_id.say(ctx, "Queue cleared.").await);
    } else {
        check_msg(
            msg.channel_id
                .say(ctx, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
