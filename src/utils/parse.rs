use serenity::framework::standard::Args;

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
    format!("sounds/{}", sound_name)
}
