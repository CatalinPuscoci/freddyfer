use rand::seq::SliceRandom;
use serenity::framework::standard::Args;
use std::path::Path;

pub(crate) fn get_repeat_count(mut args: Args, fallback_value: i32) -> i32 {
    match args.single::<i32>() {
        Ok(count) => {
            if 0 < count && count < 50 {
                count
            } else {
                1
            }
        }
        Err(_) => fallback_value,
    }
}

pub(crate) fn get_sound_path(sound_name: &str) -> String {
    let path = format!("sounds/{}", sound_name);
    if Path::new(path.as_str()).exists() {
        return path
    }

    else {
        let sounds = vec!["sounds/ilie_cum.mp3","sounds/ilie_ha.mp3"];
        return sounds.choose(&mut rand::thread_rng()).unwrap().to_string();
    }
}
