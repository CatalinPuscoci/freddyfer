use serenity::model::prelude::Message;
use serenity::Result as SerenityResult;
use std::fmt::Debug;

/// Checks that a result was successfully received; if not, then logs why to stdout.
pub fn check_result<T, E>(result: Result<T, E>, error_message: &str)
where
    E: Debug,
{
    if let Err(why) = result {
        println!("{}: {:?}", error_message, why);
    }
}

pub(crate) fn check_msg(result: SerenityResult<Message>) {
    check_result(result, "Error sending message")
}
