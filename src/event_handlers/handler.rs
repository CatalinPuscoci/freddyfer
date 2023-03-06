use std::{thread, time::Duration};

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, voice::VoiceState},
};

use crate::utils::{
    checks::{check_msg, check_result},
    diacritics::clean_all,
    parse::get_sound_path,
};
use songbird::{input, tracks::create_player};

pub struct MainEventHandler;

#[async_trait]
impl EventHandler for MainEventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
        println!(
            "{} joined voice channel {}",
            _new.member.unwrap().user,
            _new.channel_id.unwrap()
        );

        let manager = songbird::get(&ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        let guild = _new.guild_id.unwrap();
        let channel_id = _new.channel_id.unwrap();

        if let Some(handler_lock) = manager.get(guild) {
            let mut handler = handler_lock.lock().await;
            if channel_id.0 == handler.current_channel().unwrap().0 {
                let oldchannel = match _old {
                    Some(oldstate) => oldstate.channel_id,
                    None => None,
                };
                if oldchannel.is_none() || oldchannel.unwrap().0 != channel_id.0 {
                    let source = input::ffmpeg(get_sound_path("Aloooo.mp3")).await.unwrap();
                    let (mut audio, _) = create_player(source);
                    thread::sleep(Duration::from_millis(1000));
                    audio.set_volume(0.25);
                    handler.play(audio);
                    println!("playing")
                }
            }
        }
    }

    async fn message(&self, _ctx: Context, _new_message: Message) {
        match _new_message.mentions_me(&_ctx).await {
            Ok(mentions) => {
                if mentions {
                    let without_diacritics =
                        clean_all(_new_message.content.to_lowercase().as_str());

                    if without_diacritics.contains("tacusi") {
                        check_msg(_new_message.channel_id.say(&_ctx, "Gata 😔").await);

                        mute(&_ctx, &_new_message).await
                    } else if without_diacritics.contains("glumesc") {
                        check_msg(_new_message.channel_id.say(&_ctx, "😊").await);

                        unmute(&_ctx, &_new_message).await;
                    }
                }
            }
            Err(why) => {
                println!("Err trying to see if message mentions me: {:?}", why);
            }
        };
    }
}

async fn unmute(ctx: &Context, msg: &Message) {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        check_result(handler.mute(false).await, "Err when unmuting");
    }
}

async fn mute(ctx: &Context, msg: &Message) {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return;
        }
    };

    let mut handler = handler_lock.lock().await;

    if !handler.is_mute() {
        check_result(handler.mute(true).await, "Err when muting");
    }
}
